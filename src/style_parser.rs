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
            obj: Box::new(Expr::Ident(Ident::new(
              "TextDecorationType".into(),
              DUMMY_SP,
            ))),
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
      kind: if value == "" {
        "None".to_string()
      } else {
        value.to_string()
      },
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

impl BorderRadius {
  pub fn new() -> Self {
    BorderRadius {
      top_left: "0".to_string(),
      top_right: "0".to_string(),
      bottom_left: "0".to_string(),
      bottom_right: "0".to_string(),
    }
  }
  pub fn is_zero(&self) -> bool {
    self.top_left == "0"
      && self.top_right == "0"
      && self.bottom_left == "0"
      && self.bottom_right == "0"
  }

  pub fn set_top_left(&mut self, top_left: &str) {
    self.top_left = top_left.to_string();
  }

  pub fn set_top_right(&mut self, top_right: &str) {
    self.top_right = top_right.to_string();
  }

  pub fn set_bottom_left(&mut self, bottom_left: &str) {
    self.bottom_left = bottom_left.to_string();
  }

  pub fn set_bottom_right(&mut self, bottom_right: &str) {
    self.bottom_right = bottom_right.to_string();
  }
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
pub struct MarginPadding {
  pub top: String,
  pub right: String,
  pub bottom: String,
  pub left: String,
}

impl MarginPadding {
  pub fn new() -> Self {
    MarginPadding {
      top: "0".to_string(),
      right: "0".to_string(),
      bottom: "0".to_string(),
      left: "0".to_string(),
    }
  }

  pub fn is_zero(&self) -> bool {
    self.top == "0" && self.right == "0" && self.bottom == "0" && self.left == "0"
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = top.to_string();
  }

  pub fn set_right(&mut self, right: &str) {
    self.right = right.to_string();
  }

  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = bottom.to_string();
  }

  pub fn set_left(&mut self, left: &str) {
    self.left = left.to_string();
  }
}

impl ToObjectExpr for MarginPadding {
  fn to_object_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.right.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.left.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for MarginPadding {
  fn from(value: &str) -> Self {
    let margin_padding = value.to_string();
    let margin_padding = margin_padding.split(" ").collect::<Vec<&str>>();
    let margin_padding = match margin_padding.len() {
      1 => MarginPadding {
        top: margin_padding[0].to_string(),
        right: margin_padding[0].to_string(),
        bottom: margin_padding[0].to_string(),
        left: margin_padding[0].to_string(),
      },
      2 => MarginPadding {
        top: margin_padding[0].to_string(),
        right: margin_padding[1].to_string(),
        bottom: margin_padding[0].to_string(),
        left: margin_padding[1].to_string(),
      },
      3 => MarginPadding {
        top: margin_padding[0].to_string(),
        right: margin_padding[1].to_string(),
        bottom: margin_padding[2].to_string(),
        left: margin_padding[1].to_string(),
      },
      4 => MarginPadding {
        top: margin_padding[0].to_string(),
        right: margin_padding[1].to_string(),
        bottom: margin_padding[2].to_string(),
        left: margin_padding[3].to_string(),
      },
      _ => MarginPadding {
        top: "0".to_string(),
        right: "0".to_string(),
        bottom: "0".to_string(),
        left: "0".to_string(),
      },
    };
    margin_padding
  }
}

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(String),
  TextDecoration(TextDecoration),
  BorderRadius(BorderRadius),
  MarginPadding(MarginPadding),
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
      StyleValueType::MarginPadding(value) => {
        write!(
          f,
          "{} {} {} {}",
          value.top, value.right, value.bottom, value.left
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

  fn parse_style_style(&self, style_value: &mut StyleDeclaration<'i>) -> StyleValue {
    let properties = style_value
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
          property,
        )
      })
      .collect::<HashMap<_, _>>();
    let mut final_properties = HashMap::new();
    let mut margin = None;
    let mut margin_left = None;
    let mut margin_right = None;
    let mut margin_top = None;
    let mut margin_bottom = None;
    let mut padding = None;
    let mut padding_left = None;
    let mut padding_right = None;
    let mut padding_top = None;
    let mut padding_bottom = None;
    let mut border_radius = None;
    let mut border_radius_top_left = None;
    let mut border_radius_top_right = None;
    let mut border_radius_bottom_left = None;
    let mut border_radius_bottom_right = None;
    let mut text_decoration = None;
    let mut color = None;

    for (id, value) in properties.into_iter() {
      if id == "margin" {
        margin = Some(value.clone());
      } else if id == "marginLeft" {
        margin_left = Some(value.clone());
      } else if id == "marginRight" {
        margin_right = Some(value.clone());
      } else if id == "marginTop" {
        margin_top = Some(value.clone());
      } else if id == "marginBottom" {
        margin_bottom = Some(value.clone());
      } else if id == "padding" {
        padding = Some(value.clone());
      } else if id == "paddingLeft" {
        padding_left = Some(value.clone());
      } else if id == "paddingRight" {
        padding_right = Some(value.clone());
      } else if id == "paddingTop" {
        padding_top = Some(value.clone());
      } else if id == "paddingBottom" {
        padding_bottom = Some(value.clone());
      } else if id == "borderRadius" {
        border_radius = Some(value.clone());
      } else if id == "borderRadiusTopLeft" {
        border_radius_top_left = Some(value.clone());
      } else if id == "borderRadiusTopRight" {
        border_radius_top_right = Some(value.clone());
      } else if id == "borderRadiusBottomLeft" {
        border_radius_bottom_left = Some(value.clone());
      } else if id == "borderRadiusBottomRight" {
        border_radius_bottom_right = Some(value.clone());
      } else if id == "textDecoration" {
        text_decoration = Some(value.clone());
      } else if id == "color" {
        color = Some(value.clone());
        final_properties.insert(
          id,
          StyleValueType::Normal(
            value
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
        );
      } else {
        final_properties.insert(
          id,
          StyleValueType::Normal(
            value
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
        );
      }
    }

    if let Some(text_decoration) = text_decoration {
      match &text_decoration {
        Property::TextDecoration(decoration, _) => {
          final_properties.insert(
            "textDecoration".to_string(),
            StyleValueType::TextDecoration(TextDecoration {
              kind: decoration
                .line
                .to_css_string(PrinterOptions::default())
                .unwrap(),
              color: match color {
                Some(color) => to_camel_case(
                  color
                    .value_to_css_string(PrinterOptions::default())
                    .unwrap()
                    .as_str(),
                  true,
                ),
                None => "black".to_string(),
              },
            }),
          );
        }
        _ => {}
      }
    }

    let mut margin = match margin {
      Some(margin) => match margin {
        Property::Margin(value) => {
          let mut margin = MarginPadding::new();
          margin.set_top(
            value
              .top
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin.set_right(
            value
              .right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin.set_bottom(
            value
              .bottom
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin.set_left(
            value
              .left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin
        }
        _ => MarginPadding::new(),
      },
      None => MarginPadding::new(),
    };

    if let Some(margin_left) = margin_left {
      margin.set_left(
        match margin_left {
          Property::MarginLeft(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(margin_right) = margin_right {
      margin.set_right(
        match margin_right {
          Property::MarginRight(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(margin_top) = margin_top {
      margin.set_top(
        match margin_top {
          Property::MarginTop(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(margin_bottom) = margin_bottom {
      margin.set_bottom(
        match margin_bottom {
          Property::MarginBottom(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if margin.is_zero() {
      final_properties.remove("margin");
    } else {
      final_properties.insert("margin".to_string(), StyleValueType::MarginPadding(margin));
    }

    let mut padding = match padding {
      Some(padding) => match padding {
        Property::Padding(value) => {
          let mut padding = MarginPadding::new();
          padding.set_top(
            value
              .top
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          padding.set_right(
            value
              .right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          padding.set_bottom(
            value
              .bottom
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          padding.set_left(
            value
              .left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          padding
        }
        _ => MarginPadding::new(),
      },
      None => MarginPadding::new(),
    };

    if let Some(padding_left) = padding_left {
      padding.set_left(
        match padding_left {
          Property::PaddingLeft(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(padding_right) = padding_right {
      padding.set_right(
        match padding_right {
          Property::PaddingRight(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(padding_top) = padding_top {
      padding.set_top(
        match padding_top {
          Property::PaddingTop(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(padding_bottom) = padding_bottom {
      padding.set_bottom(
        match padding_bottom {
          Property::PaddingBottom(value) => value.to_css_string(PrinterOptions::default()).unwrap(),
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if padding.is_zero() {
      final_properties.remove("padding");
    } else {
      final_properties.insert(
        "padding".to_string(),
        StyleValueType::MarginPadding(padding),
      );
    }

    let mut border_radius = match border_radius {
      Some(border_radius) => match border_radius {
        Property::BorderRadius(value, _) => {
          let mut border_radius = BorderRadius::new();
          border_radius.set_top_left(
            value
              .top_left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_top_right(
            value
              .top_right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_bottom_left(
            value
              .bottom_left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_bottom_right(
            value
              .bottom_right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius
        }
        _ => BorderRadius::new(),
      },
      None => BorderRadius::new(),
    };

    if let Some(border_radius_top_left) = border_radius_top_left {
      border_radius.set_top_left(
        match border_radius_top_left {
          Property::BorderTopLeftRadius(value, _) => {
            value.to_css_string(PrinterOptions::default()).unwrap()
          }
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(border_radius_top_right) = border_radius_top_right {
      border_radius.set_top_right(
        match border_radius_top_right {
          Property::BorderTopRightRadius(value, _) => {
            value.to_css_string(PrinterOptions::default()).unwrap()
          }
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(border_radius_bottom_left) = border_radius_bottom_left {
      border_radius.set_bottom_left(
        match border_radius_bottom_left {
          Property::BorderBottomLeftRadius(value, _) => {
            value.to_css_string(PrinterOptions::default()).unwrap()
          }
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if let Some(border_radius_bottom_right) = border_radius_bottom_right {
      border_radius.set_bottom_right(
        match border_radius_bottom_right {
          Property::BorderBottomRightRadius(value, _) => {
            value.to_css_string(PrinterOptions::default()).unwrap()
          }
          _ => "0".to_string(),
        }
        .as_str(),
      );
    }
    if border_radius.is_zero() {
      final_properties.remove("borderRadius");
    } else {
      final_properties.insert(
        "borderRadius".to_string(),
        StyleValueType::BorderRadius(border_radius),
      );
    }

    final_properties
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let mut final_all_style = self.calc_style_record(&mut all_style);

    let mut final_all_style = final_all_style
      .iter_mut()
      .map(|(selector, style_value)| (selector.to_owned(), self.parse_style_style(style_value)))
      .collect::<HashMap<_, _>>();

    for (selector, style_value) in final_all_style.iter_mut() {
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
