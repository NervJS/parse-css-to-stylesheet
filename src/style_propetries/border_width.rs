use lightningcss::{
  properties::{Property, border::BorderSideWidth},
  traits::ToCss
};
use swc_atoms::Atom;
use swc_ecma_ast::{PropName, Expr, Tpl};
use crate::{generate_prop_name, generate_expr_lit_str, generate_expr_by_length, generate_invalid_expr };

use super::{traits::ToExpr, unit::PropertyTuple};


#[macro_export]
macro_rules! generate_expr_by_border_side_width {
  ($val:expr) => {{
    use $crate::{generate_invalid_expr, generate_expr_by_length};
    use lightningcss::properties::border::BorderSideWidth;
    match $val {
      BorderSideWidth::Thin | BorderSideWidth::Medium | BorderSideWidth::Thick => generate_invalid_expr!(),
      BorderSideWidth::Length(length) => {
        generate_expr_by_length!(length, Platform::ReactNative)
      },
    }
  }};
}

#[derive(Debug, Clone)]
pub struct BorderWidth {
  pub id: String,
  pub top: Option<BorderSideWidth>,
  pub right: Option<BorderSideWidth>,
  pub bottom: Option<BorderSideWidth>,
  pub left: Option<BorderSideWidth>
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

  pub fn set_all (&mut self, val: BorderSideWidth) {
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
      let mut props: Vec<(PropName, Expr)> = vec![];
      if let Some(top) = &self.top {
        props.push((generate_prop_name!(self.id.clone()), generate_expr_by_border_side_width!(top)))
      }
      if let Some(bottom) = &self.bottom {
        props.push((generate_prop_name!(self.id.clone()), generate_expr_by_border_side_width!(bottom)))
      }
      if let Some(left) = &self.left {
        props.push((generate_prop_name!(self.id.clone()), generate_expr_by_border_side_width!(left)))
      }
      if let Some(right) = &self.right {
        props.push((generate_prop_name!(self.id.clone()), generate_expr_by_border_side_width!(right)))
      }
      PropertyTuple::Array(props)
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;
      if prop_name == "borderWidth" {
        // border-width
        let top = generate_expr_by_border_side_width!(self.top.as_ref().unwrap());
        let right = generate_expr_by_border_side_width!(self.right.as_ref().unwrap());
        let bottom = generate_expr_by_border_side_width!(self.bottom.as_ref().unwrap());
        let left = generate_expr_by_border_side_width!(self.left.as_ref().unwrap());
        // 判断top\left\bottom\right是否存在Invalid
        for (_, k) in [&top, &right, &bottom, &left].iter().enumerate() {
          if let Expr::Invalid(_) = k {
            return PropertyTuple::One(
              generate_prop_name!(prop_name.clone()), 
              generate_invalid_expr!()
            )
          }
        }
        let border_width = vec![Box::new(top), Box::new(right), Box::new(bottom), Box::new(left)];

        let tpl_expr = Expr::Tpl(Tpl {
          span: swc_common::DUMMY_SP,
          exprs: border_width,
          quasis: vec![
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: false,
              cooked: None,
              raw: Atom::from("").into(),
            },
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: false,
              cooked: Some(" ".into()),
              raw: Atom::from(" ").into(),
            },
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: false,
              cooked: Some(" ".into()),
              raw: Atom::from(" ").into(),
            },
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: false,
              cooked: Some(" ".into()),
              raw: Atom::from(" ").into(),
            },
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: true,
              cooked: None,
              raw: Atom::from("").into(),
            }
          ]
        });

        PropertyTuple::One(
          generate_prop_name!(prop_name.clone()), 
          tpl_expr
        )
        
      } else {
        let mut props: Vec<(PropName, Expr)> = vec![];
        // 单个边框颜色
        if let Some(top) = &self.top {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_border_side_width!(top)))
        }
        if let Some(bottom) = &self.bottom {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_border_side_width!(bottom)))
        }
        if let Some(left) = &self.left {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_border_side_width!(left)))
        }
        if let Some(right) = &self.right {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_border_side_width!(right)))
        }
        PropertyTuple::Array(props)
      }
    }
}