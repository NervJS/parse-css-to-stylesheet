use crate::{generate_expr_enum, style_propetries::style_property_enum};
use lightningcss::properties::{flex::BoxOrient as CSSBoxOrient, Property};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct BoxOrient {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Horizontal,
  Vertical,
  InlineAxis,
  BlockAxis,
}

impl From<(String, &Property<'_>)> for BoxOrient {
  fn from(prop: (String, &Property<'_>)) -> Self {
    BoxOrient {
      id: prop.0,
      value: match prop.1 {
        Property::BoxOrient(value, _) => match value {
          CSSBoxOrient::Horizontal => EnumValue::Horizontal,
          CSSBoxOrient::Vertical => EnumValue::Vertical,
          CSSBoxOrient::InlineAxis => EnumValue::InlineAxis,
          CSSBoxOrient::BlockAxis => EnumValue::BlockAxis,
        },
        _ => EnumValue::Vertical,
      },
    }
  }
}

impl ToExpr for BoxOrient {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(CSSPropertyType::BoxOrient, {
      match self.value {
        EnumValue::Horizontal => generate_expr_enum!(style_property_enum::BoxOrient::Horizontal),
        EnumValue::Vertical => generate_expr_enum!(style_property_enum::BoxOrient::Vertical),
        EnumValue::InlineAxis => generate_expr_enum!(style_property_enum::BoxOrient::InlineAxis),
        EnumValue::BlockAxis => generate_expr_enum!(style_property_enum::BoxOrient::BlockAxis),
      }
    })
  }
}
