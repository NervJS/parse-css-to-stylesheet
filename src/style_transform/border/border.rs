use lightningcss::properties::{Property, border::{GenericBorder, LineStyle}};
use super::{border_width::BorderWidth, border_color::BorderColor, border_style::BorderStyle};

fn parse_border (value: &GenericBorder<LineStyle, 10>) -> Border {
  let mut border = Border::new();
  border.width = BorderWidth::from(&value.width);
  border.style = BorderStyle::from(&value.style);
  border.color = BorderColor::from(&value.color);
  border
}

#[derive(Debug, Clone)]
pub struct Border {
  pub width: BorderWidth,
  pub color: BorderColor,
  pub style: BorderStyle,
}

impl Border {
  pub fn new() -> Self {
    Border {
      width: BorderWidth::new(),
      color: BorderColor::new(),
      style: BorderStyle::new(),
    }
  }
}

impl From<&Property<'_>> for Border {
  fn from(value: &Property<'_>) -> Self {
    let mut border = Border::new();
    match value {
      Property::Border(value) => {
        border = parse_border(&value)
      },
      Property::BorderTop(value) => {
        border.width.top = BorderWidth::from(&value.width).top;
        border.style.top = BorderStyle::from(&value.style).top;
        border.color.top = BorderColor::from(&value.color).top;
      },
      Property::BorderRight(value) => {
        border.width.right = BorderWidth::from(&value.width).right;
        border.style.right = BorderStyle::from(&value.style).right;
        border.color.right = BorderColor::from(&value.color).right;
      },
      Property::BorderBottom(value) => {
        border.width.bottom = BorderWidth::from(&value.width).bottom;
        border.style.bottom = BorderStyle::from(&value.style).bottom;
        border.color.bottom = BorderColor::from(&value.color).bottom;
      },
      Property::BorderLeft(value) => {
        border.width.left = BorderWidth::from(&value.width).left;
        border.style.left = BorderStyle::from(&value.style).left;
        border.color.left = BorderColor::from(&value.color).left;
      },
      _ => {}
    }
    border
  }
}

