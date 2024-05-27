

use lightningcss::properties::{display::{Display::{Keyword, Pair}, DisplayInside, DisplayKeyword, DisplayOutside}, Property};

use crate::{generate_expr_enum, generate_expr_lit_str, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Display {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  None,
  Flex,
  Block,
  Invalid,
}

impl From<(String, &Property<'_>)> for Display {
  fn from(value: (String, &Property<'_>)) -> Self {
    Display {
      id: value.0,
      value: {
        if let Property::Display(value) = &value.1 {
          match &value {
            Keyword(value) => match &value {
              DisplayKeyword::None => EnumValue::None,
              _ => EnumValue::Block,
            },
            Pair(value) => {
              if let DisplayInside::Flex(_) = value.inside {
                EnumValue::Flex
              } else {
                if let DisplayOutside::Block = value.outside {
                  EnumValue::Block
                } else {
                  EnumValue::Block
                }
              }
            }
          }
        } else {
          EnumValue::Invalid
        }
      }
    }
  }
}


impl ToExpr for Display {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::Display,
      match &self.value {
        EnumValue::None => generate_expr_enum!(style_property_enum::Display::None),
        EnumValue::Flex => generate_expr_enum!(style_property_enum::Display::Flex),
        EnumValue::Block => generate_expr_enum!(style_property_enum::Display::Block),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

}

