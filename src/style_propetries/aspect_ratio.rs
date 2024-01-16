

use lightningcss::properties::Property;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, Lit, Number};

use crate::{generate_expr_lit_num, generate_expr_ident};

use super::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum AspactRatio {
  Auto,
  Ratio(f64, f64),
}

impl From<&Property<'_>> for AspactRatio {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::AspectRatio(value) => {
        match value.auto {
          true => AspactRatio::Auto,
          false => {
            if let Some(ratio) = &value.ratio {
              AspactRatio::Ratio(ratio.0.into(), ratio.1.into())
            } else {
              AspactRatio::Auto
            }
          }
        }
      },
      _ => AspactRatio::Auto,
    }
  }
}

impl ToExpr for AspactRatio {
  fn to_expr(&self) -> Expr {
    match self {
      AspactRatio::Ratio(first, second) => generate_expr_lit_num!(first / second),
      _ => generate_expr_lit_num!(1.0),
    }
  }

  fn to_rn_expr(&self) -> Expr {
    match self {
      AspactRatio::Ratio(first, second) => generate_expr_lit_num!(first / second),
      AspactRatio::Auto => generate_expr_ident!("auto"),
    }
  }
}