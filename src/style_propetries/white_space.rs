use std::default;

use lightningcss::properties::{font, Property};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct WhiteSpace {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  NoWrap,
  Invalid,
}

impl ToExpr for WhiteSpace {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(CSSPropertyType::WhiteSpace, {
      match self.value {
        EnumValue::NoWrap => generate_expr_enum!(style_property_enum::WhiteSpace::NoWrap),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    })
  }
}

impl From<(String, &Property<'_>)> for WhiteSpace {
  fn from(prop: (String, &Property<'_>)) -> Self {
    WhiteSpace {
      id: prop.0,
      value: match prop.1 {
        Property::WhiteSpace(value) => match value {
          lightningcss::properties::text::WhiteSpace::NoWrap => EnumValue::NoWrap,
          _ => EnumValue::Invalid,
        },
        _ => EnumValue::Invalid,
      },
    }
  }
}
