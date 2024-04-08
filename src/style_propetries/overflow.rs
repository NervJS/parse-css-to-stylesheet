use lightningcss::properties::{
  Property, overflow::OverflowKeyword
};

use crate::{generate_expr_lit_str, generate_invalid_expr};

use super::{traits::ToExpr, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct Overflow {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
  Hidden,
  Visible,
  Scroll,
  Invalid,
}

impl From<(String, &Property<'_>)> for Overflow {
  fn from(value: (String, &Property<'_>)) -> Self {
    Overflow {
      id: value.0,
      value: match value.1 {
        Property::Overflow(value) => {
          match value.x {
            OverflowKeyword::Hidden => EnumValue::Hidden,
            OverflowKeyword::Visible => EnumValue::Visible,
            OverflowKeyword::Clip => EnumValue::Invalid,
            OverflowKeyword::Scroll => EnumValue::Scroll,
            OverflowKeyword::Auto => EnumValue::Scroll
          }
        },
        _ => EnumValue::Invalid
      }
    }
  }
}

impl ToExpr for Overflow {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      match &self.value {
        EnumValue::Hidden => generate_expr_lit_str!("hidden"),
        EnumValue::Visible => generate_expr_lit_str!("visible"),
        EnumValue::Scroll => generate_expr_lit_str!("scroll"),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      match &self.value {
        EnumValue::Hidden => generate_expr_lit_str!("hidden"),
        EnumValue::Visible => generate_expr_lit_str!("visible"),
        EnumValue::Scroll => generate_expr_lit_str!("scroll"),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

}
