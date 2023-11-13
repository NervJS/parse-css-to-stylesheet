use lightningcss::{
  properties::{Property, border::BorderSideWidth},
  stylesheet::PrinterOptions,
  traits::ToCss
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, Prop, KeyValueProp, Ident, PropName, ObjectLit};

use crate::{style_transform::traits::ToExpr, utils::convert_px_to_units};


#[derive(Debug, Clone)]
pub struct  BorderWidth {
  pub left: Option<String>,
  pub top: Option<String>,
  pub bottom: Option<String>,
  pub right: Option<String>
  
}

impl BorderWidth {
  pub fn new() -> Self {
    BorderWidth {
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

impl ToExpr for BorderWidth {
  fn to_expr(&self) -> Expr {

    let mut arr = vec![];
    
    if let Some(left) = &self.left {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
        value: convert_px_to_units(left.to_string()).into(),
      }))))
    }
    if let Some(right) = &self.right {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
        value: convert_px_to_units(right.to_string()).into(),
      }))))
    }
    if let Some(bottom) = &self.bottom {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
        value: convert_px_to_units(bottom.to_string()).into(),
      }))))
    }
    if let Some(top) = &self.top {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
        value: convert_px_to_units(top.to_string()).into(),
      }))))
    }

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })
  }
}

impl From<&Property<'_>> for BorderWidth {
  fn from(value: &Property<'_>) -> Self {

    let mut border_width = BorderWidth {
      left: None,
      top: None,
      bottom: None,
      right: None
    };

    match value {
      Property::BorderWidth(value) => {
        match &value.top {
          BorderSideWidth::Length(value) => {
            border_width.set_top(value.to_css_string(PrinterOptions::default()).unwrap().as_str());
          },
          _ => {}
        };
        
        match &value.bottom {
          BorderSideWidth::Length(value) => {
            border_width.set_bottom(value.to_css_string(PrinterOptions::default()).unwrap().as_str());
          },
          _ => {}
        };

        match &value.left {
          BorderSideWidth::Length(value) => {
            border_width.set_left(value.to_css_string(PrinterOptions::default()).unwrap().as_str());
          },
          _ => {}
        };

        match &value.right {
          BorderSideWidth::Length(value) => {
            border_width.set_right(value.to_css_string(PrinterOptions::default()).unwrap().as_str());
          },
          _ => {}
        };

      }
      _ => {}
    };

    border_width
  }
}