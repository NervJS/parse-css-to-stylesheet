use lightningcss::{
  properties::{Property, align::GapValue},
  values::{length::LengthValue, percentage::{DimensionPercentage, Percentage}}, traits::ToCss, stylesheet::PrinterOptions,
};
use swc_ecma_ast::Expr;

use crate::{generate_expr_lit_str, generate_expr_lit_num};

use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};


#[derive(Debug, Clone)]
pub enum Gap {
  LengthValue(LengthValue),
  Percentage(Percentage),
  String(String),
  Normal
}

impl ToExpr for Gap {
  fn to_expr(&self) -> Expr {
    match &self {
      Gap::String(value) => generate_expr_lit_str!(value.to_owned()),
      Gap::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
      Gap::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
      Gap::Normal => generate_expr_lit_num!(0.0),
    }
  }

  fn to_rn_expr(&self) -> Expr {
    match &self {
      Gap::String(value) => generate_expr_lit_str!(value.to_owned()),
      Gap::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
      Gap::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
      Gap::Normal => generate_expr_lit_str!("normal"),
    }
  }
}

macro_rules! generate_gap {
  ($value:expr) => {
    match &$value {
      GapValue::Normal => Gap::Normal,
      GapValue::LengthPercentage(value) => match value {
        DimensionPercentage::Dimension(value) => {
          Gap::LengthValue(value.clone())
        }
        DimensionPercentage::Percentage(value) => {
          Gap::Percentage(value.clone())
        }
        DimensionPercentage::Calc(value) => {
          Gap::String(value.to_css_string(PrinterOptions::default()).unwrap())
        }
      },
    }
  }
}

impl From<&Property<'_>> for Gap {
  fn from(value: &Property<'_>) -> Self {
    let mut gap = Gap::Normal;
    match &value {
      Property::RowGap(value) => {
        gap = generate_gap!(value);
      }
      Property::ColumnGap(value) => {
        gap = generate_gap!(value);
      }
      _ => {}
    }
    gap
  }
}
