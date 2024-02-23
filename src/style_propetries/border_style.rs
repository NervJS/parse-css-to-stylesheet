use lightningcss::properties::{Property, border::LineStyle};
use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{PropName, Expr, Tpl, MemberExpr, Ident, MemberProp};
use crate::{generate_invalid_expr, generate_prop_name, generate_tpl_expr };

use super::{traits::ToExpr, unit::{PropertyTuple, Platform}};

#[macro_export]
macro_rules! generate_expr_by_line_style {
  ($val:expr, $platform:expr) => {{
    use $crate::{generate_invalid_expr, generate_expr_lit_str};
    use lightningcss::properties::border::LineStyle;
    match $val {
      LineStyle::Dotted => {
        match $platform {
          Platform::ReactNative => generate_expr_lit_str!("dotted"),
          Platform::Harmony => get_expr_by_val("Dotted")
        }
      },
      LineStyle::Dashed => {
        match $platform {
          Platform::ReactNative => generate_expr_lit_str!("dashed"),
          Platform::Harmony => get_expr_by_val("Dashed")
        }
      }
      LineStyle::Solid => {
        match $platform {
          Platform::ReactNative => generate_expr_lit_str!("solid"),
          Platform::Harmony => get_expr_by_val("Solid")
        }
      }
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
      let mut props: Vec<(String, Expr)> = vec![];
      if let Some(top) = &self.top {
        props.push(("borderTopStyle".to_string(), generate_expr_by_line_style!(top, Platform::Harmony)))
      }
      if let Some(bottom) = &self.bottom {
        props.push(("borderBottomStyle".to_string(), generate_expr_by_line_style!(bottom, Platform::Harmony)))
      }
      if let Some(left) = &self.left {
        props.push(("borderLeftStyle".to_string(), generate_expr_by_line_style!(left, Platform::Harmony)))
      }
      if let Some(right) = &self.right {
        props.push(("borderRightStyle".to_string(), generate_expr_by_line_style!(right, Platform::Harmony)))
      }
      PropertyTuple::Array(props)
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;


      if prop_name == "borderStyle" {
        // border-width
        PropertyTuple::One(
          prop_name.clone(), 
          generate_expr_by_line_style!(self.top.as_ref().unwrap(), Platform::ReactNative)
        )
      } else {
        let mut props: Vec<(String, Expr)> = vec![];
        // 单个边框颜色
        if let Some(top) = &self.top {
          props.push(("borderStyle".to_string(), generate_expr_by_line_style!(top, Platform::ReactNative)))
        }
        if let Some(bottom) = &self.bottom {
          props.push(("borderStyle".to_string(), generate_expr_by_line_style!(bottom, Platform::ReactNative)))
        }
        if let Some(left) = &self.left {
          props.push(("borderStyle".to_string(), generate_expr_by_line_style!(left, Platform::ReactNative)))
        }
        if let Some(right) = &self.right {
          props.push(("borderStyle".to_string(), generate_expr_by_line_style!(right, Platform::ReactNative)))
        }
        PropertyTuple::Array(props)
      }
    }
}


pub fn get_expr_by_val(val: &str) -> Expr {
  Expr::Member(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(Expr::Ident(Ident::new("BorderStyle".into(), DUMMY_SP))),
    prop: MemberProp::Ident(Ident {
      span: DUMMY_SP,
      sym: val.into(),
      optional: false,
    }),
  })
  .into()
}
