use std::{
  cell::RefCell, collections::HashMap, convert::Infallible, fmt::Display, hash::Hash, rc::Rc,
};

use lightningcss::{
  declaration::DeclarationBlock,
  properties::Property,
  rules::CssRule,
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
  traits::ToCss,
  visit_types,
  visitor::{Visit, VisitTypes, Visitor},
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ComputedPropName, Expr, Ident, KeyValueProp, Lit, MemberExpr, MemberProp, ObjectLit, Prop,
  PropName, PropOrSpread, Str,
};

use crate::{document::JSXDocument, utils::to_camel_case, visitor::SpanKey};

pub type StyleValue = HashMap<String, StyleValueType>;

pub struct StyleData {
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleValue>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

pub trait ToObjectExpr {
  fn to_object_expr(&self) -> Expr;
}

#[derive(Debug, Clone)]
pub struct TextDecoration {
  pub kind: String,
  pub color: String,
}

impl TextDecoration {
  pub fn change_color(&mut self, color: &str) {
    self.color = color.to_string();
  }
}

impl ToObjectExpr for TextDecoration {
  fn to_object_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
          value: Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new("TextDecoration".into(), DUMMY_SP))),
            prop: MemberProp::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Expr::Lit(Lit::Str(Str::from(to_camel_case(self.kind.as_str(), true)))).into(),
            }),
          })
          .into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("color".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.color.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for TextDecoration {
  fn from(value: &str) -> Self {
    TextDecoration {
      kind: value.to_string(),
      color: "black".to_string(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct BorderRadius {
  pub top_left: String,
  pub top_right: String,
  pub bottom_left: String,
  pub bottom_right: String,
}

impl ToObjectExpr for BorderRadius {
  fn to_object_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("topLeft".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top_left.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("topRight".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top_right.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottomLeft".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom_left.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottomRight".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom_right.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for BorderRadius {
  fn from(value: &str) -> Self {
    let border_radius = value.to_string();
    let border_radius = border_radius.split(" ").collect::<Vec<&str>>();
    let border_radius = match border_radius.len() {
      1 => BorderRadius {
        top_left: border_radius[0].to_string(),
        top_right: border_radius[0].to_string(),
        bottom_left: border_radius[0].to_string(),
        bottom_right: border_radius[0].to_string(),
      },
      2 => BorderRadius {
        top_left: border_radius[0].to_string(),
        top_right: border_radius[1].to_string(),
        bottom_left: border_radius[0].to_string(),
        bottom_right: border_radius[1].to_string(),
      },
      3 => BorderRadius {
        top_left: border_radius[0].to_string(),
        top_right: border_radius[1].to_string(),
        bottom_left: border_radius[2].to_string(),
        bottom_right: border_radius[1].to_string(),
      },
      4 => BorderRadius {
        top_left: border_radius[0].to_string(),
        top_right: border_radius[1].to_string(),
        bottom_left: border_radius[2].to_string(),
        bottom_right: border_radius[3].to_string(),
      },
      _ => BorderRadius {
        top_left: "0".to_string(),
        top_right: "0".to_string(),
        bottom_left: "0".to_string(),
        bottom_right: "0".to_string(),
      },
    };
    border_radius
  }
}

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(String),
  TextDecoration(TextDecoration),
  BorderRadius(BorderRadius),
}

impl Display for StyleValueType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StyleValueType::Normal(value) => write!(f, "{}", value),
      StyleValueType::TextDecoration(value) => {
        write!(f, "{}", to_camel_case(value.kind.as_str(), true))
      }
      StyleValueType::BorderRadius(value) => {
        write!(
          f,
          "{} {} {} {}",
          value.top_left, value.top_right, value.bottom_left, value.bottom_right
        )
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

pub struct StyleVisitor<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    document: &'i JSXDocument,
    all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  ) -> Self {
    StyleVisitor {
      all_style,
      document,
    }
  }
}

impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().replace(".", "");
          let mut all_style = self.all_style.borrow_mut();
          let declarations: &mut Vec<StyleDeclaration<'_>> =
            all_style.entry(selector.clone()).or_insert(vec![]);
          declarations.push(StyleDeclaration {
            specificity: style.selectors.0.get(index).unwrap().specificity(),
            declaration: style.declarations.clone(),
          });
        }
      }
      _ => {}
    }
    Ok(())
  }
}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(HashMap::new())),
      document,
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(self.document, Rc::clone(&self.all_style));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  fn calc_style_record<T: Hash + Eq + Clone>(
    &self,
    style_record: &mut HashMap<T, Vec<StyleDeclaration<'i>>>,
  ) -> HashMap<T, StyleDeclaration<'i>> {
    let mut final_style_record = HashMap::new();
    for (id, declarations) in style_record.iter_mut() {
      declarations.sort_by(|a, b| a.specificity.cmp(&b.specificity));
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
      final_style_record.insert(
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: vec![],
          },
        },
      );
    }
    final_style_record
  }

  fn parse_style_value(&self, style_value: &mut StyleValue) {
    let mut text_decoration = None;
    let mut color = None;
    let mut border_radius = None;
    for property in style_value.iter_mut() {
      if property.0 == "textDecoration" {
        text_decoration = Some(property.1.clone());
      } else if property.0 == "color" {
        color = Some(property.1.clone());
      } else if property.0 == "borderRadius" {
        border_radius = Some(property.1.clone());
      }
    }

    if let Some(text_decoration) = text_decoration {
      style_value.insert(
        "textDecoration".to_string(),
        StyleValueType::TextDecoration(TextDecoration {
          kind: text_decoration.to_string(),
          color: match color {
            Some(color) => color.to_string(),
            None => "black".to_string(),
          },
        }),
      );
    }

    if let Some(border_radius) = border_radius {
      style_value.insert(
        "borderRadius".to_string(),
        StyleValueType::BorderRadius(BorderRadius::from(border_radius.to_string().as_str())),
      );
    }
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let final_all_style = self.calc_style_record(&mut all_style);

    let mut final_all_style = final_all_style
      .iter()
      .map(|(selector, style_value)| {
        (
          selector.to_owned(),
          style_value
            .declaration
            .declarations
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
                StyleValueType::Normal(
                  property
                    .value_to_css_string(PrinterOptions {
                      minify: false,
                      targets: Targets {
                        include: Features::HexAlphaColors,
                        ..Targets::default()
                      },
                      ..PrinterOptions::default()
                    })
                    .unwrap(),
                ),
              )
            })
            .collect::<HashMap<_, _>>(),
        )
      })
      .collect::<HashMap<_, _>>();

    for (selector, style_value) in final_all_style.iter_mut() {
      self.parse_style_value(style_value);
      let elements = self.document.select(selector);
      for element in elements {
        let declarations = style_record.entry(element.span).or_insert(vec![]);
        declarations.push(style_value.clone());
      }
    }
    let final_style_record = style_record
      .iter_mut()
      .map(|(selector, style_value)| {
        (
          *selector,
          style_value
            .iter_mut()
            .reduce(|a, b| {
              a.extend(b.drain());
              a
            })
            .unwrap()
            .to_owned(),
        )
      })
      .collect::<HashMap<_, _>>();
    StyleData {
      style_record: Rc::new(RefCell::new(final_style_record)),
      all_style: Rc::new(RefCell::new(final_all_style)),
    }
  }
}
