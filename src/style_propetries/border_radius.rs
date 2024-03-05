use lightningcss::{
  properties::Property,
  values::{length::LengthValue, percentage::DimensionPercentage}
};
use swc_ecma_ast::Expr;
use crate::{generate_expr_lit_str, generate_invalid_expr };

use super::{traits::ToExpr, unit::{PropertyTuple, generate_expr_by_length_value, Platform}};



macro_rules! generate_expr_by_dimension_percentage {
  ($val:expr, $platform:expr) => {{
    use $crate::{generate_invalid_expr, generate_expr_lit_str};
    match $val {
      DimensionPercentage::Dimension(val) => generate_expr_by_length_value(val, $platform),
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
      let mut props: Vec<(String, Expr)> = vec![];

      if let Some(top) = &self.top_left {
        props.push(("borderTopLeftRadius".to_string(), generate_expr_by_dimension_percentage!(top, Platform::Harmony)))
      }
      if let Some(bottom) = &self.top_right {
        props.push(("borderTopRightRadius".to_string(), generate_expr_by_dimension_percentage!(bottom, Platform::Harmony)))
      }
      if let Some(left) = &self.bottom_left {
        props.push(("borderBottomLeftRadius".to_string(), generate_expr_by_dimension_percentage!(left, Platform::Harmony)))
      }
      if let Some(right) = &self.bottom_right {
        props.push(("borderBottomRightRadius".to_string(), generate_expr_by_dimension_percentage!(right, Platform::Harmony)))
      }
      PropertyTuple::Array(props)
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;

      let mut is_same = false;
      // 判断self.top、self.right、self.bottom、self.left是否一致
      if self.top_left == self.top_right && self.top_right == self.bottom_left && self.bottom_left == self.bottom_right {
        is_same = true;
      }

      if prop_name == "borderRadius" && is_same {
        // border-radius
        PropertyTuple::One(
          prop_name.clone(), 
          generate_expr_by_dimension_percentage!(self.top_left.as_ref().unwrap(), Platform::ReactNative)
        )
      } else {
        let mut props: Vec<(String, Expr)> = vec![];
        // 单个边框颜色
        if let Some(top) = &self.top_left {
          props.push(("borderTopLeftRadius".to_string(), generate_expr_by_dimension_percentage!(top, Platform::ReactNative)))
        }
        if let Some(bottom) = &self.top_right {
          props.push(("borderTopRightRadius".to_string(), generate_expr_by_dimension_percentage!(bottom, Platform::ReactNative)))
        }
        if let Some(left) = &self.bottom_left {
          props.push(("borderBottomLeftRadius".to_string(), generate_expr_by_dimension_percentage!(left, Platform::ReactNative)))
        }
        if let Some(right) = &self.bottom_right {
          props.push(("borderBottomRightRadius".to_string(), generate_expr_by_dimension_percentage!(right, Platform::ReactNative)))
        }
        PropertyTuple::Array(props)
      }
    }
}