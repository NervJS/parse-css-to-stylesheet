use std::{rc::Rc, cell::RefCell, convert::Infallible, collections::HashMap, hash::Hash};

use lightningcss::{declaration::DeclarationBlock, properties::Property, rules::{keyframes::KeyframeSelector, CssRule}, stylesheet::{ParserOptions, PrinterOptions, StyleSheet}, traits::ToCss, visit_types, visitor::{Visit, VisitTypes, Visitor}};

use crate::{constants::SUPPORT_PSEUDO_KEYS, document::JSXDocument, style_propetries::{style_value_type::StyleValueType, unit::Platform}, utils::to_camel_case, visitor::SpanKey};

use super::parse_style_properties::parse_style_properties;

pub type StyleValue = Vec<StyleValueType>;

pub struct StyleData<'i> {
  pub pesudo_style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Vec<(String, Property<'i>)>)>>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
  pub has_nesting: bool
}

pub struct KeyFramesData {
  pub name: String,
  pub keyframes: Vec<KeyFrameItem>
}

#[derive(Debug)]
pub struct KeyFrameItem {
  pub percentage: f32,
  pub declarations: Vec<StyleValueType>
}

#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

struct StyleVisitor<'i> {
  all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  keyframes: Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
    keyframes: Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>,
  ) -> Self {
    StyleVisitor {
      all_style,
      keyframes
    }
  }
}

// 收集所有的样式到 all_style 中，以元祖的形式存在 (selector, vec[declaration1, declaration2, ...])
impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);

  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      // 属性规则收集
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors: Vec<&str> = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().to_string();
          let mut all_style = self.all_style.borrow_mut();
          let decorations = all_style.iter_mut().find(|(id, _)| id == &selector);
          if let Some((_, declarations)) = decorations {
            declarations.push(StyleDeclaration {
              specificity: style.selectors.0.get(index).unwrap().specificity(),
              declaration: style.declarations.clone(),
            });
          } else {
            all_style.push((
              selector.clone(),
              vec![StyleDeclaration {
                specificity: style.selectors.0.get(index).unwrap().specificity(),
                declaration: style.declarations.clone(),
              }],
            ));
          }
        }
      }
      // 动画收集
      CssRule::Keyframes(keyframes_rule) => {
        let mut keyframe_data = KeyFramesData {
          name: keyframes_rule.name.to_css_string(PrinterOptions::default()).unwrap(),
          keyframes: vec![]
        };
        keyframes_rule.keyframes.clone().into_iter().for_each(|keyframe| {
          keyframe.selectors.into_iter().for_each(|selector| {
            let properties = keyframe.declarations.iter().map(|property| {
              (
                to_camel_case(
                  property.0
                    .property_id()
                    .to_css_string(PrinterOptions::default())
                    .unwrap()
                    .as_str(),
                  false,
                ),
                property.0.clone(),
              )
            })
            .collect::<Vec<(_, _)>>(); // Speci
            let keyframe_item = KeyFrameItem {
              percentage: match selector {
                KeyframeSelector::Percentage(percentage) => {
                  percentage.0
                }
                KeyframeSelector::From => 0.0,
                KeyframeSelector::To => 100.0,
              },
              declarations: parse_style_properties(&properties, None)
            };

            keyframe_data.keyframes.push(keyframe_item)
            
          });
        });

        let mut keyframes = self.keyframes.borrow_mut();
        keyframes.insert(keyframe_data.name, keyframe_data.keyframes);
      }
      _ => {}
    }
    Ok(())
  }
}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<Vec<(String, Vec<StyleDeclaration<'i>>)>>>,
  pub keyframes: Rc<RefCell<HashMap<String, Vec<KeyFrameItem>>>>,
  pub document: &'i JSXDocument,
  pub platform: Platform
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument, platform:Platform) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(vec![])),
      keyframes: Rc::new(RefCell::new(HashMap::new())),
      document,
      platform
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(Rc::clone(&self.all_style), Rc::clone(&self.keyframes));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  pub fn calc(&self) -> StyleData<'i> {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let mut pesudo_style_record = HashMap::new();
    let mut css_variables = vec![];
    // 是否含有嵌套选择器
    let mut has_nesting = false;

    // final_all_style 转换为驼峰命名
    let mut final_all_style = vec![];
    self.calc_style_record(&mut all_style).iter_mut().for_each(|(selector, style_value)| {
      // 判断是否含有伪类:root
      if selector == ":root" {
        css_variables = style_value.declaration.declarations.iter().map(|property| {
          (
            property
              .property_id()
              .to_css_string(PrinterOptions::default())
              .unwrap(),
            property.clone(),
          )
        }).collect::<Vec<(_, _)>>();
        return;
      }
      let properties = style_value.declaration.declarations.iter().map(|property| {
        (
          to_camel_case(
            property
              .property_id()
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
            false,
          ),
          property.clone(),
        )
      })
      .collect::<Vec<(_, _)>>(); // Specify the lifetime of the tuple elements to match the input data
      // 判断是否含有嵌套选择器
      if selector.contains(" ") || selector.chars().filter(|&c| c == '.').count() > 1 {
        has_nesting = true
      }
      final_all_style.push((selector.to_owned(), properties));
    });

    let mut pesudo_selector = None;
    for (selector, style_value) in final_all_style.iter_mut() {
      // 用于查询的选择器
      let mut element_selector = selector.clone();
      // 判断是否伪类(暂时支持鸿蒙)
      if (SUPPORT_PSEUDO_KEYS.into_iter().any(|s| selector.contains(s))) && self.platform == Platform::Harmony {
        let _selectors = selector.split(":").collect::<Vec<&str>>();
        pesudo_selector = _selectors[1].parse::<String>().ok();
        // 伪类需要把 : 之后的选择器去掉，只保留 : 之前的选择器，用于查询所属的element
        element_selector = _selectors[0].to_string();
      }

      let elements = self.document.select(element_selector.as_str());
      for element in elements {
        match pesudo_selector {
          Some(ref selector) => {
            let declarations= pesudo_style_record.entry(element.span).or_insert(vec![]);
            declarations.push((selector.clone(), style_value.clone()));
          }
          None => {
            let declarations: &mut Vec<Vec<(String, Property<'_>)>> = style_record.entry(element.span).or_insert(vec![]);
            declarations.push(style_value.clone());
          }
        }
      }
    }             

    // 进行样式解析优化，提前解析 ArkUI 的样式，减少运行时的计算
    let final_all_style = final_all_style
    .iter_mut()
    .map(|(selector, properties)| {
      (
        selector.to_owned(),
        parse_style_properties(
          &properties
            .iter()
            .map(|(k, v)| (k.to_owned(), v.clone()))
            .collect::<Vec<_>>(),
          Some(self.keyframes.clone())
        ),
      )
    })
    .collect::<HashMap<_, _>>();

    let final_pesudo_style_record = pesudo_style_record;

    StyleData {
      pesudo_style_record: Rc::new(RefCell::new(final_pesudo_style_record)),
      all_style: Rc::new(RefCell::new(final_all_style)),
      has_nesting
    }
  }

  // 合并相同类型的 style，比如 .a { color: red } .a { color: blue } => .a { color: blue }，并且 !important 的优先级高于普通的
  fn calc_style_record<T: Hash + Eq + Clone>(
    &self,
    style_record: &mut Vec<(T, Vec<StyleDeclaration<'i>>)>,
  ) -> Vec<(T, StyleDeclaration<'i>)> {
    // 创建一个新的向量 final_style_record，用于存储最终的样式记录
    let mut final_style_record = vec![];
    // 对输入的 style_record 中的每个元素进行迭代
    for (id, declarations) in style_record.iter_mut() {
       // 对每个 declarations 中的 StyleDeclaration 进行按 specificity 排序
      declarations.sort_by(|a: &StyleDeclaration<'_>, b| a.specificity.cmp(&b.specificity));
      let mut final_properties: Vec<Property<'i>> = Vec::new();
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let declarations = &declaration.declarations;
        for declaration in declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let important_declarations = &declaration.important_declarations;
        for declaration in important_declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      final_style_record.push((
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: vec![],
          },
        },
      ));
    }
    final_style_record
  }
}
