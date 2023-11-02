use lightningcss::{properties::Property, stylesheet::PrinterOptions, traits::ToCss };
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str};

use super::{traits::ToExpr, utils::StringNumber };

#[derive(Debug, Clone)]
pub struct MarginPadding {
  pub top: StringNumber,
  pub right: StringNumber,
  pub bottom: StringNumber,
  pub left: StringNumber,
}

impl MarginPadding {
  pub fn new() -> Self {
    MarginPadding {
      top: StringNumber::String("0".to_string()),
      right: StringNumber::String("0".to_string()),
      bottom: StringNumber::String("0".to_string()),
      left: StringNumber::String("0".to_string()),
    }
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = StringNumber::String(top.to_string());
    // match parse_px_string(top) {
    //     Some(num) => {
    //         self.top = StringNumber::Number(num);
    //     },
    //     None => {
    //       self.top = StringNumber::String(top.to_string());
    //     }
    // }
  }

  pub fn set_right(&mut self, right: &str) {
    self.right = StringNumber::String(right.to_string());
  }

  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = StringNumber::String(bottom.to_string());
  }

  pub fn set_left(&mut self, left: &str) {
    self.left = StringNumber::String(left.to_string());
  }
}

impl Default for MarginPadding {
  fn default() -> Self {
    MarginPadding::new()
  }
}

impl ToExpr for MarginPadding {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
          value: Expr::Lit(
            match &self.top {
              StringNumber::String(str) => Lit::Str(Str::from(str.to_string())),
              StringNumber::Number(num) => {
                let number: swc_ecma_ast::Number = (*num as f64).into();
                Lit::Num(number)
              },
            }
          ).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
          value: Expr::Lit(
            match &self.right {
              StringNumber::String(str) => Lit::Str(Str::from(str.to_string())),
              StringNumber::Number(num) => {
                let number: swc_ecma_ast::Number = (*num as f64).into();
                Lit::Num(number)
              },
            }
          ).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
          value: Expr::Lit(
            match &self.bottom {
              StringNumber::String(str) => Lit::Str(Str::from(str.to_string())),
              StringNumber::Number(num) => {
                let number: swc_ecma_ast::Number = (*num as f64).into();
                Lit::Num(number)
              },
            }
          ).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
          value: Expr::Lit(
            match &self.left {
              StringNumber::String(str) => Lit::Str(Str::from(str.to_string())),
              StringNumber::Number(num) => {
                let number: swc_ecma_ast::Number = (*num as f64).into();
                Lit::Num(number)
              },
            }
          ).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&Property<'_>> for MarginPadding {
  fn from(value: &Property<'_>) -> Self {
    let mut margin_padding = MarginPadding::new();
    match value {
      Property::Margin(value) => {
        margin_padding.set_top(
          value
            .top
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str()
        );
        margin_padding.set_right(
          value
            .right
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
        margin_padding.set_bottom(
          value
            .bottom
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
        margin_padding.set_left(
          value
            .left
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
      }
      Property::Padding(value) => {
        margin_padding.set_top(
          value
            .top
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
        margin_padding.set_right(
          value
            .right
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
        margin_padding.set_bottom(
          value
            .bottom
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
        margin_padding.set_left(
          value
            .left
            .to_css_string(PrinterOptions::default())
            .unwrap()
            .as_str(),
        );
      }
      _ => {}
    }
    margin_padding
  }
}
