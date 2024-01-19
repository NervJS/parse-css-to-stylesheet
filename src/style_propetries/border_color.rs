use lightningcss::{
  properties::Property,
  traits::ToCss, values::color::CssColor
};
use swc_ecma_ast::{PropName, Expr};

use crate::{generate_prop_name, generate_string_by_css_color, generate_expr_lit_str};

use super::{traits::ToExpr, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct BorderColor {
  pub id: String,
  pub top: Option<CssColor>,
  pub right: Option<CssColor>,
  pub bottom: Option<CssColor>,
  pub left: Option<CssColor>
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

  pub fn set_all (&mut self, color: CssColor) {
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
      let mut props: Vec<(PropName, Expr)> = vec![];
      if let Some(top) = &self.top {
        props.push((generate_prop_name!(self.id.clone()), generate_string_by_css_color!(top)))
      }
      if let Some(bottom) = &self.bottom {
        props.push((generate_prop_name!(self.id.clone()), generate_string_by_css_color!(bottom)))
      }
      if let Some(left) = &self.left {
        props.push((generate_prop_name!(self.id.clone()), generate_string_by_css_color!(left)))
      }
      if let Some(right) = &self.right {
        props.push((generate_prop_name!(self.id.clone()), generate_string_by_css_color!(right)))
      }
      PropertyTuple::Array(props)
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;
      if prop_name == "borderColor" {
        // border-color
        let border_color: Vec<String> = vec![
          generate_string_by_css_color!(self.top.as_ref().unwrap()),
          generate_string_by_css_color!(self.right.as_ref().unwrap()),
          generate_string_by_css_color!(self.bottom.as_ref().unwrap()),
          generate_string_by_css_color!(self.left.as_ref().unwrap()),
        ];
        // vec 转 string: ["#000", "#f00"] => "#000 #f00"
        let border_color_string = border_color.join(" ");
        PropertyTuple::One(
          generate_prop_name!(prop_name.clone()), 
          generate_expr_lit_str!(border_color_string)
        )
      } else {
        let mut props: Vec<(PropName, Expr)> = vec![];
        // 单个边框颜色
        if let Some(top) = &self.top {
          props.push((generate_prop_name!(prop_name.clone()), generate_string_by_css_color!(top)))
        }
        if let Some(bottom) = &self.bottom {
          props.push((generate_prop_name!(prop_name.clone()), generate_string_by_css_color!(bottom)))
        }
        if let Some(left) = &self.left {
          props.push((generate_prop_name!(prop_name.clone()), generate_string_by_css_color!(left)))
        }
        if let Some(right) = &self.right {
          props.push((generate_prop_name!(prop_name.clone()), generate_string_by_css_color!(right)))
        }
        PropertyTuple::Array(props)
      }
    }
}