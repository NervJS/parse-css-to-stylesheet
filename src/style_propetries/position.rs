

use lightningcss::properties::{position, Property};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Position {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Static,
  Relative,
  Absolute,
  Fixed,
  Invalid
}

impl From<(String, &Property<'_>)> for Position {
  fn from(value: (String, &Property<'_>)) -> Self {
    Position {
      id: value.0,
      value: {
        if let Property::Position(value) = &value.1 {
          match &value {
            position::Position::Static => EnumValue::Static,
            position::Position::Relative => EnumValue::Relative,
            position::Position::Absolute => EnumValue::Absolute,
            position::Position::Fixed => EnumValue::Fixed,
            _ => EnumValue::Invalid
          }
        } else {
          EnumValue::Invalid
        }
      }
    }
  }
}


impl ToExpr for Position {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::Position,
      match &self.value {
        EnumValue::Static => generate_expr_enum!(style_property_enum::Position::Static),
        EnumValue::Relative => generate_expr_enum!(style_property_enum::Position::Relative),
        EnumValue::Absolute => generate_expr_enum!(style_property_enum::Position::Absolute),
        EnumValue::Fixed => generate_expr_enum!(style_property_enum::Position::Fixed),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

}

