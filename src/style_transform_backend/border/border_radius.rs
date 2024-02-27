use std::borrow::Borrow;

use lightningcss::{
  properties::{Property, border::BorderSideWidth},
  stylesheet::PrinterOptions,
  traits::ToCss
};

use crate::style_transform::style_value_type::StyleValueType;


#[derive(Debug, Clone)]
pub struct BorderRadius {
  pub top_left: Option<StyleValueType>,
  pub top_right: Option<StyleValueType>,
  pub bottom_left: Option<StyleValueType>,
  pub bottom_right: Option<StyleValueType>
}

impl BorderRadius {
  pub fn new() -> Self {
    BorderRadius {
      top_left: None,
      top_right: None,
      bottom_left: None,
      bottom_right: None,
    }
  }

  pub fn set_all (&mut self, width: &StyleValueType) {
    self.top_left = Some(width.clone());
    self.top_right = Some(width.clone());
    self.bottom_left = Some(width.clone());
    self.bottom_right = Some(width.clone());
  }

  pub fn top_left(&mut self, top: StyleValueType) {
    self.top_left = Some(top);
  }
  pub fn top_right(&mut self, right: StyleValueType) {
    self.top_right = Some(right);
  }
  pub fn bottom_left(&mut self, value: StyleValueType) {
    self.bottom_left = Some(value);
  }
  pub fn bottom_right(&mut self, left: StyleValueType) {
    self.bottom_right = Some(left);
  }
}


impl Default for BorderRadius {
  fn default() -> Self {
    BorderRadius::new()
  }
}

impl From <&BorderSideWidth> for BorderRadius {
  fn from(value: &BorderSideWidth) -> Self {
    match &value {
      BorderSideWidth::Length(value) => {
        let len = value.to_css_string(PrinterOptions::default()).unwrap();
        let mut border_radius = BorderRadius::new();
        border_radius.set_all(StyleValueType::Length(len).borrow());
        border_radius
      },
      _ => BorderRadius::new()
    }
  }
}

impl From<&Property<'_>> for BorderRadius {
  fn from(value: &Property<'_>) -> Self {
    let mut border_radius = BorderRadius::new();

    match value {
      Property::BorderRadius(value, _) => {
        border_radius.top_left(StyleValueType::Length(value.top_left.to_css_string(PrinterOptions::default()).unwrap()));
        border_radius.top_right(StyleValueType::Length(value.top_right.to_css_string(PrinterOptions::default()).unwrap()));
        border_radius.bottom_left(StyleValueType::Length(value.bottom_left.to_css_string(PrinterOptions::default()).unwrap()));
        border_radius.bottom_right(StyleValueType::Length(value.bottom_right.to_css_string(PrinterOptions::default()).unwrap()));
      },
      _ => {}
    };

    border_radius
  }
}