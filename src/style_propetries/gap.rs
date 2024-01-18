use lightningcss::{
  properties::{Property, align::GapValue},
  values::{length::LengthValue, percentage::{DimensionPercentage, Percentage}}, traits::ToCss, stylesheet::PrinterOptions,
};

use crate::{generate_expr_lit_str, generate_expr_lit_num, generate_prop_name};

use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform, PropertyTuple}};


#[derive(Debug, Clone)]
pub struct Gap {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  LengthValue(LengthValue),
  Percentage(Percentage),
  String(String),
  Normal
}

impl ToExpr for Gap {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      match &self.value {
        EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
        EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
        EnumValue::Normal => generate_expr_lit_num!(0.0),
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      match &self.value {
        EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
        EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
        EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
        EnumValue::Normal => generate_expr_lit_str!("normal"),
      }
    )
  }
}

macro_rules! generate_gap {
  ($value:expr) => {
    match &$value {
      GapValue::Normal => EnumValue::Normal,
      GapValue::LengthPercentage(value) => match value {
        DimensionPercentage::Dimension(value) => {
          EnumValue::LengthValue(value.clone())
        }
        DimensionPercentage::Percentage(value) => {
          EnumValue::Percentage(value.clone())
        }
        DimensionPercentage::Calc(value) => {
          EnumValue::String(value.to_css_string(PrinterOptions::default()).unwrap())
        }
      },
    }
  }
}

impl From<(String, &Property<'_>)> for Gap {
  fn from(prop: (String, &Property<'_>)) -> Self {
    Gap {
      id: prop.0,
      value: match &prop.1 {
        Property::RowGap(value) => {
          generate_gap!(value)
        }
        Property::ColumnGap(value) => {
          generate_gap!(value)
        }
        _ => EnumValue::Normal
      }
    }
  }
}
