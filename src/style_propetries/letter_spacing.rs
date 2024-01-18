use lightningcss::{
  properties::{Property,text::Spacing},
  values::{self, length::LengthValue}, stylesheet::PrinterOptions, traits::ToCss
};

use crate::{generate_prop_name, generate_invalid_expr};

use super::{unit::{PropertyTuple, generate_expr_by_length_value, Platform}, traits::ToExpr};


#[derive(Debug, Clone)]
pub struct LetterSpacing {
  pub id: String,
  pub value: EnumValue
}


#[derive(Debug, Clone)]
pub enum EnumValue {
  LengthValue(LengthValue),
  String(String),
  Invalid
}

impl ToExpr for LetterSpacing {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      match &self.value {
        EnumValue::String(_) => generate_invalid_expr!(),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(&length_value, Platform::Harmony),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      match &self.value {
        EnumValue::String(_) => generate_invalid_expr!(),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(&length_value, Platform::ReactNative),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }
}

impl From<(String, &Property<'_>)> for LetterSpacing {
  fn from(prop: (String, &Property<'_>)) -> Self {
    LetterSpacing {
      id: prop.0,
      value: match prop.1 {
        Property::LetterSpacing(value) => {
          match value {
            Spacing::Length(val) => {
              match val {
                values::length::Length::Value(value) =>  EnumValue::LengthValue(value.to_owned()),
                values::length::Length::Calc(calc) => EnumValue::String(calc.to_css_string(PrinterOptions::default()).unwrap())
              }
            }
            Spacing::Normal => EnumValue::Invalid,
          }
        }
        _ => EnumValue::Invalid
      }
    }
  }
}
