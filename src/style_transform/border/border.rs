use lightningcss::properties::{Property, border::{GenericBorder, LineStyle}};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, ObjectLit, Prop, KeyValueProp, PropName, Ident};

use crate::style_transform::traits::ToExpr;

use super::{border_width::{BorderWidth, parse_border_width_item}, border_color::{BorderColor, parse_border_color_item}, border_style::{BorderStyle, parse_border_style_item}};

fn parse_border (value: &GenericBorder<LineStyle, 10>) -> Border {
  let mut border = Border::new();
  if let Some(width) = parse_border_width_item(&value.width) {
    border.width = width;
  }
  if let Some(style) = parse_border_style_item(&value.style) {
    border.style = style;
  }
  if let Some(color) = parse_border_color_item(&value.color) {
    border.color = color;
  } 
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

impl ToExpr for Border {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("width".into(), DUMMY_SP)),
          value: self.width.to_expr().into()
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("color".into(), DUMMY_SP)),
          value: self.color.to_expr().into()
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("style".into(), DUMMY_SP)),
          value: self.style.to_expr().into()
        }))),
      ]
    })  
  }
}

impl From<&Property<'_>> for Border {
  fn from(value: &Property<'_>) -> Self {
    let mut border = Border::new();
    if let Property::Border(value) = value {
      border = parse_border(&value)
    }
    border
  }
}