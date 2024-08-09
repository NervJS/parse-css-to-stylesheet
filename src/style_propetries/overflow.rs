use lightningcss::properties::{overflow::OverflowKeyword, Property};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Overflow {
  pub id: String,
  pub value: EnumValue,
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
        Property::Overflow(value) => match value.x {
          OverflowKeyword::Hidden => EnumValue::Hidden,
          OverflowKeyword::Visible => EnumValue::Visible,
          OverflowKeyword::Clip => EnumValue::Invalid,
          OverflowKeyword::Scroll => EnumValue::Scroll,
          OverflowKeyword::Auto => EnumValue::Scroll,
        },
        _ => EnumValue::Invalid,
      },
    }
  }
}

impl ToExpr for Overflow {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::Overflow,
      match &self.value {
        EnumValue::Hidden => generate_expr_enum!(style_property_enum::Overflow::Hidden),
        EnumValue::Visible => generate_expr_enum!(style_property_enum::Overflow::Visible),
        EnumValue::Scroll => generate_expr_enum!(style_property_enum::Overflow::Scroll),
        EnumValue::Invalid => generate_invalid_expr!(),
      },
    )
  }
}
