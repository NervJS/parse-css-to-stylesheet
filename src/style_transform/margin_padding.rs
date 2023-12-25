use lightningcss::{properties::Property, stylesheet::PrinterOptions, traits::ToCss };
use super::style_value_type::StyleValueType;

#[derive(Debug, Clone)]
pub struct MarginPadding {
  pub top: Option<StyleValueType>,
  pub right: Option<StyleValueType>,
  pub bottom: Option<StyleValueType>,
  pub left: Option<StyleValueType>,
}

impl MarginPadding {
  pub fn new() -> Self {
    MarginPadding {
      top: None,
      right: None,
      bottom: None,
      left: None
    }
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
    self.left = Some(left)
  }
}

impl Default for MarginPadding {
  fn default() -> Self {
    MarginPadding::new()
  }
}

impl From<&Property<'_>> for MarginPadding {
  fn from(value: &Property<'_>) -> Self {
    let mut margin_padding = MarginPadding::new();
    match value {
      Property::Margin(value) => {
        margin_padding.set_top(
          StyleValueType::Length(
            value
              .top
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_right(
          StyleValueType::Length(
            value
              .right
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_bottom(
          StyleValueType::Length(
            value
              .bottom
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_left(
          StyleValueType::Length(
            value
              .left
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
      }
      Property::Padding(value) => {
        margin_padding.set_top(
          StyleValueType::Length(
            value
              .top
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_right(
          StyleValueType::Length(
            value
              .right
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_bottom(
          StyleValueType::Length(
            value
              .bottom
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
        margin_padding.set_left(
          StyleValueType::Length(
            value
              .left
              .to_css_string(PrinterOptions::default())
              .unwrap()
          )
        );
      }
      _ => {}
    }
    margin_padding
  }
}
