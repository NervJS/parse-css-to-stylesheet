use lightningcss::properties::{
  display::{
    Display::{Keyword, Pair},
    DisplayInside, DisplayKeyword, DisplayOutside,
  },
  Property,
};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Display {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  None,
  Flex,
  Block,
  Invalid,
  Box,
  InlineBlock,
  Inline,
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
              } else if let DisplayInside::Box(_) = value.inside {
                EnumValue::Box
              } else if let DisplayInside::FlowRoot = value.inside {
                if let DisplayOutside::Inline = value.outside {
                  EnumValue::InlineBlock
                } else {
                  EnumValue::Block
                }
              } else if let DisplayOutside::Inline = value.outside {
                EnumValue::Inline
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
      },
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
        EnumValue::Box => generate_expr_enum!(style_property_enum::Display::Box),
        EnumValue::InlineBlock => generate_expr_enum!(style_property_enum::Display::InlineBlock),
        EnumValue::Inline => generate_expr_enum!(style_property_enum::Display::Inline),
        EnumValue::Invalid => generate_invalid_expr!(),
      },
    )
  }
}
