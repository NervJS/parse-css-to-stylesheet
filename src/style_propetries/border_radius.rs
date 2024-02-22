use lightningcss::{
  properties::Property,
  values::{length::LengthValue, percentage::DimensionPercentage}
};
use swc_atoms::Atom;
use swc_ecma_ast::{PropName, Expr, Tpl};
use crate::{generate_prop_name, generate_expr_lit_str, generate_invalid_expr };

use super::{traits::ToExpr, unit::{PropertyTuple, generate_expr_by_length_value, Platform}};



macro_rules! generate_expr_by_dimension_percentage {
  ($val:expr) => {{
    use $crate::{generate_invalid_expr, generate_expr_lit_str};
    match $val {
      DimensionPercentage::Dimension(val) => generate_expr_by_length_value(val, Platform::ReactNative),
      DimensionPercentage::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
      DimensionPercentage::Calc(_) => generate_invalid_expr!()
  }
  }};
}

#[derive(Debug, Clone)]
pub struct BorderRadius {
  pub id: String,
  pub top_left: Option<DimensionPercentage<LengthValue>>,
  pub top_right: Option<DimensionPercentage<LengthValue>>,
  pub bottom_left: Option<DimensionPercentage<LengthValue>>,
  pub bottom_right: Option<DimensionPercentage<LengthValue>>
}

impl BorderRadius {
  pub fn new(id: String) -> Self {
    BorderRadius {
      id: id,
      top_left: None,
      top_right: None,
      bottom_left: None,
      bottom_right: None,
    }
  }

  pub fn set_top_left(&mut self, top: DimensionPercentage<LengthValue>) {
    self.top_left = Some(top);
  }
  pub fn set_top_right(&mut self, right: DimensionPercentage<LengthValue>) {
    self.top_right = Some(right);
  }
  pub fn set_bottom_left(&mut self, bottom: DimensionPercentage<LengthValue>) {
    self.bottom_left = Some(bottom);
  }
  pub fn set_bottom_right(&mut self, left: DimensionPercentage<LengthValue>) {
    self.bottom_right = Some(left);
  }
}


impl From<(String, &Property<'_>)> for BorderRadius {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut border_width = BorderRadius::new(prop.0);
    match prop.1 {
      Property::BorderRadius(value, _) => {
        border_width.set_top_left(value.top_left.0.to_owned());
        border_width.set_top_right(value.top_right.0.to_owned());
        border_width.set_bottom_right(value.bottom_right.0.to_owned());
        border_width.set_bottom_left(value.bottom_left.0.to_owned());
      }
      Property::BorderTopLeftRadius(value,_) => {
        border_width.set_top_left(value.0.to_owned());
      }
      Property::BorderTopRightRadius(value,_) => {
        border_width.set_top_right(value.0.to_owned());
      }
      Property::BorderBottomRightRadius(value, _) => {
        border_width.set_bottom_right(value.0.to_owned());
      }
      Property::BorderBottomLeftRadius(value, _) => {
        border_width.set_bottom_left(value.0.to_owned());
      }
      _ => {}
    }
    border_width
  }
}

impl ToExpr for BorderRadius {
    fn to_expr(&self) -> PropertyTuple {
      let mut props: Vec<(PropName, Expr)> = vec![];

      if let Some(top) = &self.top_left {
        props.push((generate_prop_name!("borderTopLeftRadius"), generate_expr_by_dimension_percentage!(top)))
      }
      if let Some(bottom) = &self.top_right {
        props.push((generate_prop_name!("borderTopRightRadius"), generate_expr_by_dimension_percentage!(bottom)))
      }
      if let Some(left) = &self.bottom_left {
        props.push((generate_prop_name!("borderBottomLeftRadius"), generate_expr_by_dimension_percentage!(left)))
      }
      if let Some(right) = &self.bottom_right {
        props.push((generate_prop_name!("borderBottomRightRadius"), generate_expr_by_dimension_percentage!(right)))
      }
      PropertyTuple::Array(props)
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;
      if prop_name == "borderRadius" {
        // border-radius
        let top_left: Expr = generate_expr_by_dimension_percentage!(self.top_left.as_ref().unwrap());
        let top_right = generate_expr_by_dimension_percentage!(self.top_right.as_ref().unwrap());
        let bottom_right = generate_expr_by_dimension_percentage!(self.bottom_right.as_ref().unwrap());
        let bottom_left = generate_expr_by_dimension_percentage!(self.bottom_left.as_ref().unwrap());
        // 判断top\left\bottom\right是否存在Invalid
        for (_, k) in [&top_left, &top_right, &bottom_right, &bottom_left].iter().enumerate() {
          if let Expr::Invalid(_) = k {
            return PropertyTuple::One(
              generate_prop_name!(prop_name.clone()), 
              generate_invalid_expr!()
            )
          }
        }
        let border_radius = vec![Box::new(top_left), Box::new(top_right), Box::new(bottom_right), Box::new(bottom_left)];

        let tpl_expr = Expr::Tpl(Tpl {
          span: swc_common::DUMMY_SP,
          exprs: border_radius,
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
        if let Some(top) = &self.top_left {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_dimension_percentage!(top)))
        }
        if let Some(bottom) = &self.top_right {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_dimension_percentage!(bottom)))
        }
        if let Some(left) = &self.bottom_left {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_dimension_percentage!(left)))
        }
        if let Some(right) = &self.bottom_right {
          props.push((generate_prop_name!(prop_name.clone()), generate_expr_by_dimension_percentage!(right)))
        }
        PropertyTuple::Array(props)
      }
    }
}