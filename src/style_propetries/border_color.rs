use lightningcss::{properties::Property, values::color::CssColor};

use swc_core::ecma::ast::Expr;

use crate::generate_expr_lit_color;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct BorderColor {
  pub id: String,
  pub top: Option<CssColor>,
  pub right: Option<CssColor>,
  pub bottom: Option<CssColor>,
  pub left: Option<CssColor>,
}

impl BorderColor {
  pub fn new(id: String) -> Self {
    BorderColor {
      id: id,
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }

  pub fn set_all(&mut self, color: CssColor) {
    self.top = Some(color.clone());
    self.right = Some(color.clone());
    self.bottom = Some(color.clone());
    self.left = Some(color.clone());
  }

  pub fn set_top(&mut self, top: CssColor) {
    self.top = Some(top);
  }
  pub fn set_right(&mut self, right: CssColor) {
    self.right = Some(right);
  }
  pub fn set_bottom(&mut self, bottom: CssColor) {
    self.bottom = Some(bottom);
  }
  pub fn set_left(&mut self, left: CssColor) {
    self.left = Some(left);
  }
}

impl From<(String, &Property<'_>)> for BorderColor {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut border_color = BorderColor::new(prop.0);
    match prop.1 {
      Property::BorderColor(value) => {
        border_color.set_top(value.top.to_owned());
        border_color.set_bottom(value.bottom.to_owned());
        border_color.set_left(value.left.to_owned());
        border_color.set_right(value.right.to_owned());
      }
      Property::BorderTopColor(value) => {
        border_color.set_top(value.to_owned());
      }
      Property::BorderRightColor(value) => {
        border_color.set_right(value.to_owned());
      }
      Property::BorderBottomColor(value) => {
        border_color.set_bottom(value.to_owned());
      }
      Property::BorderLeftColor(value) => {
        border_color.set_left(value.to_owned());
      }
      _ => {}
    }
    border_color
  }
}

impl ToExpr for BorderColor {
  fn to_expr(&self) -> PropertyTuple {
    let mut props: Vec<(CSSPropertyType, Expr)> = vec![];
    if let Some(top) = &self.top {
      props.push((
        CSSPropertyType::BorderTopColor,
        generate_expr_lit_color!(top),
      ))
    }
    if let Some(bottom) = &self.bottom {
      props.push((
        CSSPropertyType::BorderBottomColor,
        generate_expr_lit_color!(bottom),
      ))
    }
    if let Some(left) = &self.left {
      props.push((
        CSSPropertyType::BorderLeftColor,
        generate_expr_lit_color!(left),
      ))
    }
    if let Some(right) = &self.right {
      props.push((
        CSSPropertyType::BorderRightColor,
        generate_expr_lit_color!(right),
      ))
    }
    PropertyTuple::Array(props)
  }
}
