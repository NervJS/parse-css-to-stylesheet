use lightningcss::{properties::Property, stylesheet::PrinterOptions, traits::ToCss};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str};

use super::traits::ToExpr;

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

impl Default for BorderRadius {
  fn default() -> Self {
    BorderRadius::new()
  }
}

impl ToExpr for BorderRadius {
  fn to_expr(&self) -> Expr {
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

impl From<&Property<'_>> for BorderRadius {
  fn from(value: &Property<'_>) -> Self {
    let mut border_radius = BorderRadius::new();
    match value {
      Property::BorderRadius(value, _) => {
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
      }
      _ => {}
    }
    border_radius
  }
}
