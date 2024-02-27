use std::borrow::Borrow;

use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  traits::ToCss, targets::{Features, Targets}, values::color::CssColor
};

use crate::style_transform::style_value_type::StyleValueType;

#[derive(Debug, Clone)]

pub struct BorderColor {
  pub top: Option<StyleValueType>,
  pub right: Option<StyleValueType>,
  pub bottom: Option<StyleValueType>,
  pub left: Option<StyleValueType>
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

  pub fn set_all (&mut self, color: &StyleValueType) {
    self.top = Some(color.clone());
    self.right = Some(color.clone());
    self.bottom = Some(color.clone());
    self.left = Some(color.clone());
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

impl From<&CssColor> for BorderColor {
  fn from(value: &CssColor) -> Self {
    let mut border_color = BorderColor::new();
    if *value != CssColor::default() {
      let color = value.to_css_string(PrinterOptions {
        minify: false,
        targets: Targets {
          include: Features::HexAlphaColors,
          ..Targets::default()
        },
        ..PrinterOptions::default()
      }).unwrap();
      border_color.set_all(StyleValueType::Color(color).borrow());
    }
    border_color
  
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
                0 => border_color.set_top(StyleValueType::Color(color)),
                1 => border_color.set_right(StyleValueType::Color(color)),
                2 => border_color.set_bottom(StyleValueType::Color(color)),
                3 => border_color.set_left(StyleValueType::Color(color)),
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