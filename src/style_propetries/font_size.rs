use lightningcss::{
  properties::{Property, font},
  values::{length::LengthValue, percentage::Percentage}, traits::ToCss,
};

use crate::{generate_dimension_percentage, generate_expr_lit_calc, generate_expr_lit_str, generate_invalid_expr};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::{generate_expr_by_length_value, Platform, PropertyTuple}};


#[derive(Debug, Clone)]
pub struct FontSize {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  LengthValue(LengthValue),
  Percentage(Percentage),
  String(String),
  Invalid
}

impl ToExpr for FontSize {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FontSize,
      match &self.value {
        EnumValue::String(value) => generate_expr_lit_calc!(value, Platform::Harmony),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
        EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

}

impl From<(String, &Property<'_>)> for FontSize {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontSize {
      id: prop.0,
      value: match prop.1 {
        Property::FontSize(value) => {
          match value {
            font::FontSize::Length(val) => generate_dimension_percentage!(EnumValue, val),
            _ => EnumValue::Invalid
          }
        }
        _ => EnumValue::Invalid
      }
    }
  }
}
