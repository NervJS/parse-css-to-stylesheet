use std::{cell::RefCell, convert::Infallible, rc::Rc};

use indexmap::IndexMap;
use lightningcss::{declaration::DeclarationBlock, properties::Property, rules::{keyframes::KeyframeSelector, CssRule}, stylesheet::{ParserOptions, PrinterOptions, StyleSheet}, traits::ToCss, visit_types, visitor::{Visit, VisitTypes, Visitor}};
use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use crate::{parse_style_properties::DeclsAndVars, style_propetries::{style_value_type::StyleValueType, unit::Platform}, utils::to_camel_case};
use crate::visitor::parse_style_values;
use super::parse_style_properties::parse_style_properties;

use crate::style_propetries::style_media::StyleMedia;

pub type StyleValue = Vec<StyleValueType>;
#[derive(Debug)]
pub struct StyleData {
  pub all_style: Rc<RefCell<IndexMap<(u32,String), DeclsAndVars>>>,
  pub all_keyframes: Rc<RefCell<IndexMap<(u32, String), Vec<KeyFrameItem>>>>,
  pub all_medias: Rc<RefCell<Vec<StyleMedia>>>,
}

pub struct KeyFramesData {
  pub name: String,
  pub keyframes: Vec<KeyFrameItem>
}

#[derive(Debug, Clone)]
pub struct KeyFrameItem {
  pub percentage: f32,
  pub declarations: Vec<StyleValueType>
}
impl KeyFrameItem {
  pub fn to_expr(&self)->Vec<PropOrSpread>{
    let arr_keyframe_items = vec![
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
        key: PropName::Ident(Ident::new("percent".into(), DUMMY_SP)),
        value: Box::new(Expr::Lit(Lit::Num(Number::from(self.percentage as f64)))),
      }))),
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp{
        key: PropName::Str("event".into()),
        value: Box::new(Expr::Array(ArrayLit{
            span: DUMMY_SP,
            elems: parse_style_values(self.declarations.clone(), Platform::Harmony)
        }))
      })))
    ];
    return arr_keyframe_items;
  }
}
#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

struct StyleVisitor<'i> {
  all_style: Rc<RefCell<Vec<(u32, String, Vec<StyleDeclaration<'i>>)>>>,
  keyframes: Rc<RefCell<Vec<(u32, String, Vec<KeyFrameItem>)>>>,
  medias: Rc<RefCell<Vec<StyleMedia>>>,
  media_index : u32,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    all_style: Rc<RefCell<Vec<(u32, String, Vec<StyleDeclaration<'i>>)>>>,
    keyframes: Rc<RefCell<Vec<(u32, String, Vec<KeyFrameItem>)>>>,
    medias: Rc<RefCell<Vec<StyleMedia>>>,
    media_index: u32
  ) -> Self {
    StyleVisitor {
      all_style,
      keyframes,
      medias,
      media_index
    }
  }
}

// 收集所有的样式到 all_style 中，以元祖的形式存在 (selector, vec[declaration1, declaration2, ...])
impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;

  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      // 属性规则收集
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors: Vec<&str> = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().to_string();
          let mut all_style = self.all_style.borrow_mut();
          let decorations = all_style.iter_mut().find(|(media_idx, id, _)| id == &selector&&media_idx==&self.media_index);
          if let Some((_, _, declarations)) = decorations {
            declarations.push(StyleDeclaration {
              specificity: style.selectors.0.get(index).unwrap().specificity(),
              declaration: style.declarations.clone(),
            });
          } else {
            all_style.push((
              self.media_index,
              selector.clone(),
              vec![StyleDeclaration {
                specificity: style.selectors.0.get(index).unwrap().specificity(),
                declaration: style.declarations.clone(),
              }],
            ));
          }
        }
      }
      // media
      CssRule::Media(media) =>{
        //let mut medias = self.medias.borrow_mut();
        let media_id = self.medias.borrow_mut().len() as u32 + 1;
        let mut media_data = StyleMedia { media_id, conditions: vec![] };
        media_data.parse(&media.query.media_queries);

        self.medias.borrow_mut().push(media_data);
        self.media_index = self.medias.borrow_mut().len() as u32;
        let _ = self.visit_rule_list(&mut media.rules);
        self.media_index = 0;
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
            let item = parse_style_properties(&properties);
            let keyframe_item = KeyFrameItem {
              percentage: match selector {
                KeyframeSelector::Percentage(percentage) => {
                  percentage.0
                }
                KeyframeSelector::From => 0.0,
                KeyframeSelector::To => 1.0,
              },
              declarations: item.decls
            };

            keyframe_data.keyframes.push(keyframe_item)
            
          });
        });
        // 更具percentage排序
        keyframe_data.keyframes.sort_by(|a, b| a.percentage.partial_cmp(&b.percentage).unwrap());

        let mut keyframes = self.keyframes.borrow_mut();
        keyframes.push((self.media_index, keyframe_data.name, keyframe_data.keyframes));
      }
      _ => {}
    }
    Ok(())
  }
  
  fn visit_types(&self) -> VisitTypes {
       visit_types!(RULES)
  }

}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<Vec<(u32, String, Vec<StyleDeclaration<'i>>)>>>,
  pub all_keyframes: Rc<RefCell<Vec<(u32, String, Vec<KeyFrameItem>)>>>,
  pub all_medias: Rc<RefCell<Vec<StyleMedia>>>,
}

impl<'i> StyleParser<'i> {
  pub fn new(_: Platform) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(vec![])),
      all_keyframes: Rc::new(RefCell::new(vec![])),
      all_medias: Rc::new(RefCell::new(vec![])),
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(Rc::clone(&self.all_style), Rc::clone(&self.all_keyframes), Rc::clone(&self.all_medias), 0);
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    //let mut all_style = self.all_style.borrow_mut();
    // final_all_style 转换为驼峰命名
    let mut final_all_style = vec![];
    //self.calc_style_record(&mut all_style).iter_mut().for_each(|(media_index, selector, style_value)| {
    let mut binding = self.calc_style_record();
    binding.iter_mut().for_each(|(media_index, selector, style_value)| {
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
      final_all_style.push((media_index,selector.to_owned(), properties));
    });

    // 进行样式解析优化，提前解析 ArkUI 的样式，减少运行时的计算
    let final_all_style = final_all_style
    .iter_mut()
    .map(|(media_index, selector, properties)| {
      (
        (media_index.to_owned(), selector.to_owned()),
        parse_style_properties(
          &properties
            .iter()
            .map(|(k, v)| (k.to_owned(), v.clone()))
            .collect::<Vec<_>>()
        ),
      )
    })
    .collect::<IndexMap<(_, _), _>>();

    let final_all_keyframes = self.all_keyframes.borrow_mut().iter_mut().map(|(media_index, name, keyframe)|{
      ((media_index.to_owned(),name.to_owned()),keyframe.to_vec())
    }).collect::<IndexMap<(_, _), _>>();

    return StyleData {
      all_style: Rc::new(RefCell::new(final_all_style)),
      all_keyframes: Rc::new(RefCell::new(final_all_keyframes)),
      all_medias: self.all_medias.clone()
    }

  }

  // 合并相同类型的 style，比如 .a { color: red } .a { color: blue } => .a { color: blue }，并且 !important 的优先级高于普通的
  fn calc_style_record(
    &self,
    //style_record: &mut Vec<(u32, T, Vec<StyleDeclaration<'i>>)>,
  ) -> Vec<(u32, String, StyleDeclaration<'i>)> {
    let mut style_record = self.all_style.borrow_mut();
    // 创建一个新的向量 final_style_record，用于存储最终的样式记录
    let mut final_style_record = vec![];
    // 对输入的 style_record 中的每个元素进行迭代
    for (media_index, id, declarations) in style_record.iter_mut() {
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
        media_index.to_owned(),
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
