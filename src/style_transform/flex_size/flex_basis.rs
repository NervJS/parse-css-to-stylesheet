use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{length::LengthPercentageOrAuto, number::CSSNumber, percentage::DimensionPercentage},
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number, Str};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FlexBasis {
  String(String),
  Number(CSSNumber),
}

impl ToExpr for FlexBasis {
  fn to_expr(&self) -> Expr {
    match self {
      FlexBasis::String(value) => Expr::Lit(Lit::Str(Str::from(value.to_string()))).into(),
      FlexBasis::Number(value) => Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: *value as f64,
        raw: None,
      }))
      .into(),
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
            DimensionPercentage::Dimension(value) => FlexBasis::Number(value.to_unit_value().0),
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
