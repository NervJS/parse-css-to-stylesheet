use lightningcss::properties::{Property, border::LineStyle};

use swc_core::ecma::ast::*;

use crate::generate_invalid_expr;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[macro_export]
macro_rules! generate_expr_by_line_style {
  ($val:expr) => {{
    use $crate::{generate_invalid_expr, generate_expr_enum, style_propetries::style_property_enum};
    use lightningcss::properties::border::LineStyle;
    match $val {
      LineStyle::Dotted => generate_expr_enum!(style_property_enum::ArkUI_BorderStyle::ARKUI_BORDER_STYLE_DOTTED),
      LineStyle::Dashed => generate_expr_enum!(style_property_enum::ArkUI_BorderStyle::ARKUI_BORDER_STYLE_DASHED),
      LineStyle::Solid => generate_expr_enum!(style_property_enum::ArkUI_BorderStyle::ARKUI_BORDER_STYLE_SOLID),
      _ => generate_invalid_expr!()
  }
  }};
}

#[derive(Debug, Clone)]
pub struct BorderStyle {
  pub id: String,
  pub top: Option<LineStyle>,
  pub right: Option<LineStyle>,
  pub bottom: Option<LineStyle>,
  pub left: Option<LineStyle>
}

impl BorderStyle {
  pub fn new(id: String) -> Self {
    BorderStyle {
      id: id,
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }

  pub fn set_all (&mut self, val: LineStyle) {
    self.top = Some(val.clone());
    self.right = Some(val.clone());
    self.bottom = Some(val.clone());
    self.left = Some(val.clone());
  }

  pub fn set_top(&mut self, top: LineStyle) {
    self.top = Some(top);
  }
  pub fn set_right(&mut self, right: LineStyle) {
    self.right = Some(right);
  }
  pub fn set_bottom(&mut self, bottom: LineStyle) {
    self.bottom = Some(bottom);
  }
  pub fn set_left(&mut self, left: LineStyle) {
    self.left = Some(left);
  }
}


impl From<(String, &Property<'_>)> for BorderStyle {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut border_style = BorderStyle::new(prop.0);
    match prop.1 {
      Property::BorderStyle(value) => {
        border_style.set_top(value.top);
        border_style.set_bottom(value.bottom);
        border_style.set_left(value.left);
        border_style.set_right(value.right);
      }
      Property::BorderTopStyle(value) => {
        border_style.set_top(value.to_owned());
      }
      Property::BorderRightStyle(value) => {
        border_style.set_right(value.to_owned());
      }
      Property::BorderBottomStyle(value) => {
        border_style.set_bottom(value.to_owned());
      }
      Property::BorderLeftStyle(value) => {
        border_style.set_left(value.to_owned());
      }
      _ => {}
    }
    border_style
  }
}

impl ToExpr for BorderStyle {
    fn to_expr(&self) -> PropertyTuple {
      let mut props: Vec<(CSSPropertyType, Expr)> = vec![];
      if let Some(top) = &self.top {
        props.push((CSSPropertyType::BorderTopStyle, generate_expr_by_line_style!(top)))
      }
      if let Some(bottom) = &self.bottom {
        props.push((CSSPropertyType::BorderBottomStyle, generate_expr_by_line_style!(bottom)))
      }
      if let Some(left) = &self.left {
        props.push((CSSPropertyType::BorderLeftStyle, generate_expr_by_line_style!(left)))
      }
      if let Some(right) = &self.right {
        props.push((CSSPropertyType::BorderRightStyle, generate_expr_by_line_style!(right)))
      }
      PropertyTuple::Array(props)
    }
    
}
