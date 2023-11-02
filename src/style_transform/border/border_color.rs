use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  traits::ToCss, targets::{Features, Targets}
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, Prop, KeyValueProp, Ident, PropName, ObjectLit};

use crate::style_transform::traits::ToExpr;


#[derive(Debug, Clone)]

pub struct BorderColor {
  pub top: Option<String>,
  pub right: Option<String>,
  pub bottom: Option<String>,
  pub left: Option<String>
}


impl BorderColor {
  
  pub fn new() -> Self {
    BorderColor {
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }

  pub fn is_zero(&self) -> bool {
    self.top == None
      && self.right == None
      && self.bottom == None
      && self.left == None
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = Some(top.to_string());
  }
  pub fn set_right(&mut self, right: &str) {
    self.right = Some(right.to_string());
  }
  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = Some(bottom.to_string());
  }
  pub fn set_left(&mut self, left: &str) {
    self.left = Some(left.to_string());
  }

}

impl ToExpr for BorderColor {
  fn to_expr(&self) -> Expr {
    let mut arr = vec![];
    
    if let Some(left) = &self.left {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
        value: left.to_string().into(),
      }))))
    }
    if let Some(right) = &self.right {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
        value: right.to_string().into(),
      }))))
    }
    if let Some(bottom) = &self.bottom {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
        value: bottom.to_string().into(),
      }))))
    }
    if let Some(top) = &self.top {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
        value: top.to_string().into(),
      }))))
    }

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })
  }
}

impl From<&Property<'_>> for BorderColor {
  fn from(value: &Property<'_>) -> Self {
    let mut border_color = BorderColor {
      top: None,
      right: None,
      bottom: None,
      left: None
    };
    match value {
      Property::BorderColor(value) => {
        
        for (i, k) in [&value.top, &value.right, &value.bottom, &value.left].iter().enumerate() {
          match k.to_css_string(PrinterOptions {
            minify: false,
            targets: Targets {
              include: Features::HexAlphaColors,
              ..Targets::default()
            },
            ..PrinterOptions::default()
          }) {
            Ok(color) => {
              match i {
                0 => border_color.set_top(color.as_str()),
                1 => border_color.set_right(color.as_str()),
                2 => border_color.set_bottom(color.as_str()),
                3 => border_color.set_left(color.as_str()),
                _ => {}
              }
            },
            Err(_) => {}
          };
        }
      }
      _ => {}
    }
    border_color
  }
}