use std::borrow::Borrow;

use lightningcss::{
  properties::{Property, border::BorderSideWidth},
  stylesheet::PrinterOptions,
  traits::ToCss
};

use crate::style_transform::style_value_type::StyleValueType;


#[derive(Debug, Clone)]
pub struct BorderWidth {
  pub left: Option<StyleValueType>,
  pub top: Option<StyleValueType>,
  pub bottom: Option<StyleValueType>,
  pub right: Option<StyleValueType>
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

  pub fn set_all (&mut self, width: &StyleValueType) {
    self.top = Some(width.clone());
    self.right = Some(width.clone());
    self.bottom = Some(width.clone());
    self.left = Some(width.clone());
  }

  pub fn set_top(&mut self, top: StyleValueType) {
    self.top = Some(top);
  }
  pub fn set_right(&mut self, right: StyleValueType) {
    self.right = Some(right);
  }
  pub fn set_bottom(&mut self, bottom: StyleValueType) {
    self.bottom = Some(bottom);
  }
  pub fn set_left(&mut self, left: StyleValueType) {
    self.left = Some(left);
  }
}


impl Default for BorderWidth {
  fn default() -> Self {
    BorderWidth::new()
  }
}

impl From <&BorderSideWidth> for BorderWidth {
  fn from(value: &BorderSideWidth) -> Self {
    match &value {
      BorderSideWidth::Length(value) => {
        let len = value.to_css_string(PrinterOptions::default()).unwrap();
        let mut border_width = BorderWidth::new();
        border_width.set_all(StyleValueType::Length(len).borrow());
        border_width
      },
      _ => BorderWidth::new()
    }
  }
}

impl From<&Property<'_>> for BorderWidth {
  fn from(value: &Property<'_>) -> Self {

    let mut border_width = BorderWidth::new();

    match value {
      Property::BorderWidth(value) => {
        match &value.top {
          BorderSideWidth::Length(value) => {
            border_width.set_top(StyleValueType::Length(
              value.to_css_string(PrinterOptions::default()) .unwrap()
            ));
          },
          _ => {}
        };
        match &value.bottom {
          BorderSideWidth::Length(value) => {
            border_width.set_bottom(StyleValueType::Length(
              value.to_css_string(PrinterOptions::default()) .unwrap()
            ));
          },
          _ => {}
        };
        match &value.left {
          BorderSideWidth::Length(value) => {
            border_width.set_left(StyleValueType::Length(
              value.to_css_string(PrinterOptions::default()) .unwrap()
            ));
          },
          _ => {}
        };
        match &value.right {
          BorderSideWidth::Length(value) => {
            border_width.set_right(StyleValueType::Length(
              value.to_css_string(PrinterOptions::default()) .unwrap()
            ));
          },
          _ => {}
        };

      }
      _ => {}
    };

    border_width
  }
}