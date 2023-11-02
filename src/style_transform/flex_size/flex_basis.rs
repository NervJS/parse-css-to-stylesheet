use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{length::LengthPercentageOrAuto, percentage::DimensionPercentage},
};
use swc_ecma_ast::{Expr, Lit, Str};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FlexBasis {
  String(String)
}

impl ToExpr for FlexBasis {
  fn to_expr(&self) -> Expr {
    match self {
      FlexBasis::String(value) => Expr::Lit(Lit::Str(Str::from(value.to_string()))).into(),
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
            DimensionPercentage::Dimension(value) => FlexBasis::String(value.to_css_string(PrinterOptions::default()).unwrap()),
            DimensionPercentage::Percentage(value) => {
              FlexBasis::String(value.to_css_string(PrinterOptions::default()).unwrap())
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
