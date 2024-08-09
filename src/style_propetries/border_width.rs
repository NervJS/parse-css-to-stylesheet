use crate::{generate_expr_by_length, generate_invalid_expr};
use lightningcss::properties::{border::BorderSideWidth, Property};
use swc_core::ecma::ast::Expr;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[macro_export]
macro_rules! generate_expr_by_border_side_width {
  ($val:expr, $platform:expr) => {{
    use lightningcss::properties::border::BorderSideWidth;
    use $crate::{generate_expr_by_length, generate_invalid_expr};
    match $val {
      BorderSideWidth::Thin | BorderSideWidth::Medium | BorderSideWidth::Thick => {
        generate_invalid_expr!()
      }
      BorderSideWidth::Length(length) => {
        generate_expr_by_length!(length, $platform)
      }
    }
  }};
}

#[derive(Debug, Clone)]
pub struct BorderWidth {
  pub id: String,
  pub top: Option<BorderSideWidth>,
  pub right: Option<BorderSideWidth>,
  pub bottom: Option<BorderSideWidth>,
  pub left: Option<BorderSideWidth>,
}

impl BorderWidth {
  pub fn new(id: String) -> Self {
    BorderWidth {
      id: id,
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }

  pub fn set_all(&mut self, val: BorderSideWidth) {
    self.top = Some(val.clone());
    self.right = Some(val.clone());
    self.bottom = Some(val.clone());
    self.left = Some(val.clone());
  }

  pub fn set_top(&mut self, top: BorderSideWidth) {
    self.top = Some(top);
  }
  pub fn set_right(&mut self, right: BorderSideWidth) {
    self.right = Some(right);
  }
  pub fn set_bottom(&mut self, bottom: BorderSideWidth) {
    self.bottom = Some(bottom);
  }
  pub fn set_left(&mut self, left: BorderSideWidth) {
    self.left = Some(left);
  }
}

impl From<(String, &Property<'_>)> for BorderWidth {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut border_width = BorderWidth::new(prop.0);
    match prop.1 {
      Property::BorderWidth(value) => {
        border_width.set_top(value.top.to_owned());
        border_width.set_bottom(value.bottom.to_owned());
        border_width.set_left(value.left.to_owned());
        border_width.set_right(value.right.to_owned());
      }
      Property::BorderTopWidth(value) => {
        border_width.set_top(value.to_owned());
      }
      Property::BorderRightWidth(value) => {
        border_width.set_right(value.to_owned());
      }
      Property::BorderBottomWidth(value) => {
        border_width.set_bottom(value.to_owned());
      }
      Property::BorderLeftWidth(value) => {
        border_width.set_left(value.to_owned());
      }
      _ => {}
    }
    border_width
  }
}

impl ToExpr for BorderWidth {
  fn to_expr(&self) -> PropertyTuple {
    let mut props: Vec<(CSSPropertyType, Expr)> = vec![];
    if let Some(top) = &self.top {
      props.push((
        CSSPropertyType::BorderTopWidth,
        generate_expr_by_border_side_width!(top, Platform::Harmony),
      ))
    }
    if let Some(bottom) = &self.bottom {
      props.push((
        CSSPropertyType::BorderBottomWidth,
        generate_expr_by_border_side_width!(bottom, Platform::Harmony),
      ))
    }
    if let Some(left) = &self.left {
      props.push((
        CSSPropertyType::BorderLeftWidth,
        generate_expr_by_border_side_width!(left, Platform::Harmony),
      ))
    }
    if let Some(right) = &self.right {
      props.push((
        CSSPropertyType::BorderRightWidth,
        generate_expr_by_border_side_width!(right, Platform::Harmony),
      ))
    }
    PropertyTuple::Array(props)
  }
}
