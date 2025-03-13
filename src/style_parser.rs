use std::{cell::RefCell, convert::Infallible, rc::Rc};

use super::parse_style_properties::parse_style_properties;
use crate::parse_style_properties::DeclsAndVars;
use crate::style_propetries::style_value_type::CssVariable;
use crate::{generate_expr_enum, generate_expr_lit_str};
use crate::style_propetries::font_weight::{self, FontWeight};
use crate::style_propetries::style_property_enum::ArkUI_FontWeight;
use crate::style_propetries::traits::ToExpr;
use crate::visitor::parse_style_values;
use crate::{
  style_propetries::{style_value_type::StyleValueType, unit::Platform},
  utils::to_camel_case,
};
use indexmap::IndexMap;
use lightningcss::properties::font::FontFamily;
use lightningcss::rules::font_face::{FontFaceProperty, Source};
use lightningcss::{
  declaration::DeclarationBlock,
  properties::{Property, font::{FontWeight as FontWeightProperty, AbsoluteFontWeight}},
  rules::{keyframes::KeyframeSelector, CssRule},
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  traits::ToCss,
  visit_types,
  visitor::{Visit, VisitTypes, Visitor},
};
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::style_propetries::style_media::StyleMedia;

pub type StyleValue = Vec<StyleValueType>;

#[derive(Debug, Clone)]
pub struct RuleItem {
  pub selector: String,
  pub media: u32,
  pub declarations: Vec<StyleValueType>,
  pub important_declarections:  Vec<StyleValueType>,
  pub variables: Vec<CssVariable>,
  pub has_env: bool
}

#[derive(Debug)]
pub struct StyleData {
  pub all_style: Rc<RefCell<Vec<RuleItem>>>, 
  pub all_keyframes: Rc<RefCell<IndexMap<(u32, String), Vec<KeyFrameItem>>>>,
  pub all_medias: Rc<RefCell<Vec<StyleMedia>>>,
  pub all_fonts: Rc<RefCell<Vec<FontFaceItem>>>,
}

pub struct KeyFramesData {
  pub name: String,
  pub keyframes: Vec<KeyFrameItem>,
}

#[derive(Debug, Clone)]
pub struct KeyFrameItem {
  pub percentage: f32,
  pub declarations: Vec<StyleValueType>,
}
impl KeyFrameItem {
  pub fn to_expr(&self) -> Vec<PropOrSpread> {
    let arr_keyframe_items = vec![
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("percent".into(), DUMMY_SP)),
        value: Box::new(Expr::Lit(Lit::Num(Number::from(self.percentage as f64)))),
      }))),
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Str("event".into()),
        value: Box::new(Expr::Array(ArrayLit {
          span: DUMMY_SP,
          elems: parse_style_values(self.declarations.clone(), vec![], Platform::Harmony),
        })),
      }))),
    ];
    return arr_keyframe_items;
  }
}

#[derive(Debug, Clone)]
pub struct FontFaceItem {
  pub font_family: String,
  pub src: String,
  pub font_weight: Option<ArkUI_FontWeight>,
}

impl FontFaceItem {
  pub fn to_expr(&self) -> Vec<PropOrSpread> {
    let mut result = vec![
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("fontFamily".into(), DUMMY_SP)),
        value: Box::new(generate_expr_lit_str!(self.font_family.clone())),
      }))),
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("src".into(), DUMMY_SP)),
        value: Box::new(generate_expr_lit_str!(self.src.clone())),
      }))),
    ];
    if let Some(font_weight) = self.font_weight {
      result.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("fontWeight".into(), DUMMY_SP)),
        value: Box::new(generate_expr_enum!(font_weight)),
      }))));
    }
    return result;
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
  all_fonts: Rc<RefCell<Vec<FontFaceItem>>>,
  medias: Rc<RefCell<Vec<StyleMedia>>>,
  media_index: u32,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    all_style: Rc<RefCell<Vec<(u32, String, Vec<StyleDeclaration<'i>>)>>>,
    keyframes: Rc<RefCell<Vec<(u32, String, Vec<KeyFrameItem>)>>>,
    all_fonts: Rc<RefCell<Vec<FontFaceItem>>>,
    medias: Rc<RefCell<Vec<StyleMedia>>>,
    media_index: u32,
  ) -> Self {
    StyleVisitor {
      all_style,
      keyframes,
      all_fonts,
      medias,
      media_index,
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
          let decorations = all_style
            .iter_mut()
            .find(|(media_idx, id, _)| id == &selector && media_idx == &self.media_index);
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
      CssRule::Media(media) => {
        //let mut medias = self.medias.borrow_mut();
        let media_id = self.medias.borrow_mut().len() as u32 + 1;
        let mut media_data = StyleMedia {
          media_id,
          conditions: vec![],
        };
        media_data.parse(&media.query.media_queries);

        self.medias.borrow_mut().push(media_data);
        self.media_index = self.medias.borrow_mut().len() as u32;
        let _ = self.visit_rule_list(&mut media.rules);
        self.media_index = 0;
      }
      // 动画收集
      CssRule::Keyframes(keyframes_rule) => {
        let mut keyframe_data = KeyFramesData {
          name: keyframes_rule
            .name
            .to_css_string(PrinterOptions::default())
            .unwrap(),
          keyframes: vec![],
        };
        keyframes_rule
          .keyframes
          .clone()
          .into_iter()
          .for_each(|keyframe| {
            keyframe.selectors.into_iter().for_each(|selector| {
              let properties = keyframe
                .declarations
                .iter()
                .map(|property| {
                  (
                    to_camel_case(
                      property
                        .0
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
                  KeyframeSelector::Percentage(percentage) => percentage.0,
                  KeyframeSelector::From => 0.0,
                  KeyframeSelector::To => 1.0,
                },
                declarations: parse_style_properties(&properties).decls,
              };

              keyframe_data.keyframes.push(keyframe_item)
            });
          });
        // 更具percentage排序
        keyframe_data
          .keyframes
          .sort_by(|a, b| a.percentage.partial_cmp(&b.percentage).unwrap());

        let mut keyframes = self.keyframes.borrow_mut();
        keyframes.push((
          self.media_index,
          keyframe_data.name,
          keyframe_data.keyframes,
        ));
      },
      // 字体收集
      CssRule::FontFace(font_face_rule) => {
        let mut font_face = FontFaceItem {
          font_family: "".to_string(),
          src: "".to_string(),
          font_weight: None,
        };
        font_face_rule.properties.iter().for_each(|property| {
          match property {
            FontFaceProperty::FontFamily(value) => {
              font_face.font_family = value.to_css_string(PrinterOptions::default()).unwrap();
            },
            FontFaceProperty::Source(source) => {
              // src 只取第一个
              if let Some(next) = source.iter().next() {
                match next {
                  Source::Url(value) => {
                    font_face.src = value.url.url.as_ref().to_string();
                  },
                  _ => {}
                }
              }
            },
            FontFaceProperty::FontWeight(font_weight) => {
              font_face.font_weight = Some(match &font_weight.0 {
                FontWeightProperty::Bolder => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_BOLDER,
                FontWeightProperty::Lighter => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_LIGHTER,
                FontWeightProperty::Absolute(val) => {
                  match val {
                    AbsoluteFontWeight::Bold => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_BOLD,
                    AbsoluteFontWeight::Normal => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_NORMAL,
                    AbsoluteFontWeight::Weight(num) => {
                      let new_num = ((num / 100.0).ceil() * 100.0) as i32;
                      match new_num {
                        100 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W100,
                        200 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W200,
                        300 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W300,
                        400 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W400,
                        500 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W500,
                        600 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W600,
                        700 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W700,
                        800 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W800,
                        900 => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W900,
                        _ => ArkUI_FontWeight::ARKUI_FONT_WEIGHT_NORMAL,
                      }
                    },
                  }
                },
              });
            },
            _ => {}
          };
          if !font_face.font_family.is_empty() && !font_face.src.is_empty() {
            let mut all_fonts = self.all_fonts.borrow_mut();
            let has_font_index = all_fonts
              .iter()
              .position(|font| font.font_family == font_face.font_family);
            if let Some(index) = has_font_index {
              all_fonts[index] = font_face.clone();
            } else {
              all_fonts.push(font_face.clone());
            }
          }
        });
      },
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
  pub all_fonts: Rc<RefCell<Vec<FontFaceItem>>>,
}

impl<'i> StyleParser<'i> {
  pub fn new(_: Platform) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(vec![])),
      all_keyframes: Rc::new(RefCell::new(vec![])),
      all_medias: Rc::new(RefCell::new(vec![])),
      all_fonts: Rc::new(RefCell::new(vec![])),
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(
      Rc::clone(&self.all_style),
      Rc::clone(&self.all_keyframes),
      Rc::clone(&self.all_fonts),
      Rc::clone(&self.all_medias),
      0,
    );
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    //let mut all_style = self.all_style.borrow_mut();
    // final_all_style 转换为驼峰命名
    let mut final_all_style = vec![];
    //self.calc_style_record(&mut all_style).iter_mut().for_each(|(media_index, selector, style_value)| {
    let mut binding = self.calc_style_record();
    binding
      .iter_mut()
      .for_each(|(media_index, selector, style_value)| {
        // 辅助函数，用于处理属性转换，减少代码重复
        let convert_properties = |props: &Vec<Property<'i>>| -> Vec<(String, Property<'i>)> {
          props
            .iter()
            .map(|property| {
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
            .collect()
        };
        
        // 处理普通属性和important属性
        let properties = convert_properties(&style_value.declaration.declarations);
        let important_properties = convert_properties(&style_value.declaration.important_declarations);
        
        final_all_style.push((media_index, selector.to_owned(), properties, important_properties));
      });

    // 进行样式解析优化，提前解析 ArkUI 的样式，减少运行时的计算
    let final_all_style = final_all_style
      .iter_mut()
      .map(|(media_index, selector, properties, important_properties)| {
        let decls_and_vars = parse_style_properties(
          &properties
            .iter()
            .map(|(k, v)| (k.to_owned(), v.clone()))
            .collect::<Vec<_>>()
        );
        let import_decls_and_vars = parse_style_properties(
          &important_properties
            .iter()
            .map(|(k, v)| (k.to_owned(), v.clone()))
            .collect::<Vec<_>>()
        );
        RuleItem {
          selector: selector.to_owned(),
          media: media_index.to_owned(),
          declarations: decls_and_vars.decls,
          important_declarections: import_decls_and_vars.decls,
          variables: decls_and_vars.vars,
          has_env: decls_and_vars.has_env
        }
      }).collect::<Vec<RuleItem>>();

    let final_all_keyframes = self
      .all_keyframes
      .borrow_mut()
      .iter_mut()
      .map(|(media_index, name, keyframe)| {
        ((media_index.to_owned(), name.to_owned()), keyframe.to_vec())
      })
      .collect::<IndexMap<(_, _), _>>();

    return StyleData {
      all_style: Rc::new(RefCell::new(final_all_style)),
      all_keyframes: Rc::new(RefCell::new(final_all_keyframes)),
      all_medias: self.all_medias.clone(),
      all_fonts: self.all_fonts.clone(),
    };
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
          final_properties.push(declaration.clone());
        }
      }
      let mut important_properties: Vec<Property<'i>> = Vec::new();
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let important_declarations = &declaration.important_declarations;
        for declaration in important_declarations.iter() {
          important_properties.push(declaration.clone());
        }
      }
      final_style_record.push((
        media_index.to_owned(),
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: important_properties,
          },
        },
      ));
    }
    final_style_record
  }
}
