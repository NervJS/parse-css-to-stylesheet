use lightningcss::{
  properties::Property,
  values::{length::{LengthPercentageOrAuto, LengthValue}, percentage::Percentage},
  traits::ToCss
};

use crate::{generate_dimension_percentage, generate_expr_lit_calc, generate_expr_lit_str};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::{generate_expr_by_length_value, Platform, PropertyTuple}};

#[derive(Debug, Clone)]
pub struct FlexBasis {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  LengthValue(LengthValue),
  Percentage(Percentage),
  String(String)
}

impl ToExpr for FlexBasis {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FlexBasis,
      match &self.value {
        EnumValue::String(value) => generate_expr_lit_calc!(value, Platform::Harmony),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
        EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
      }
    )
  }
}

impl From<(String, &Property<'_>)> for FlexBasis {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexBasis {
      id: prop.0,
      value: match prop.1 {
        Property::FlexBasis(value, _) => {
          match &value {
            LengthPercentageOrAuto::Auto => EnumValue::String("auto".to_string()),
            LengthPercentageOrAuto::LengthPercentage(value) => generate_dimension_percentage!(EnumValue, value),
          }
        }
        _ => EnumValue::String("auto".to_string())
      }
    }
  }
}

impl FlexBasis {
  pub fn from_value(prop: (String, LengthPercentageOrAuto)) -> Self {
    FlexBasis {
      id: prop.0,
      value: match prop.1 {
        LengthPercentageOrAuto::Auto => EnumValue::String("auto".to_string()),
        LengthPercentageOrAuto::LengthPercentage(value) => generate_dimension_percentage!(EnumValue, value),
      }
    }
  }
}