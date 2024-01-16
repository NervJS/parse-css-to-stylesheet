use lightningcss::{
  properties::Property,
  values::{length::{LengthPercentageOrAuto, LengthValue}, percentage::{DimensionPercentage, Percentage}},
};
use swc_ecma_ast::Expr;

use crate::generate_expr_lit_str;

use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};


#[derive(Debug, Clone)]
pub enum FlexBasis {
  LengthValue(LengthValue),
  Percentage(Percentage),
  String(String)
}

impl ToExpr for FlexBasis {
  fn to_expr(&self) -> Expr {
    match &self {
      FlexBasis::String(value) => generate_expr_lit_str!(value.to_owned()),
      FlexBasis::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
      FlexBasis::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
    }
  }

  fn to_rn_expr(&self) -> Expr {
    match &self {
      FlexBasis::String(value) => generate_expr_lit_str!(value.to_owned()),
      FlexBasis::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
      FlexBasis::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
    }
  }
}

impl From<&Property<'_>> for FlexBasis {
  fn from(value: &Property<'_>) -> Self {
    let mut flex_basis = FlexBasis::String("auto".to_string());
    match value {
      Property::FlexBasis(value, _) => {
        flex_basis = match &value {
          LengthPercentageOrAuto::Auto => FlexBasis::String("auto".to_string()),
          LengthPercentageOrAuto::LengthPercentage(value) => match value {
            DimensionPercentage::Dimension(value) => {
              FlexBasis::LengthValue(value.clone())
            }
            DimensionPercentage::Percentage(value) => {
              FlexBasis::Percentage(value.clone())
            }
            _ => FlexBasis::String("auto".to_string()),
          },
        };
      }
      _ => {}
    }
    flex_basis
  }
}
