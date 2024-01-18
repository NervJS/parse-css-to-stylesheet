use swc_ecma_ast::Expr;

use super::unit::{Platform, PropertyTuple};

pub trait ToExpr {
  fn to_expr(&self) -> PropertyTuple;

  fn to_rn_expr(&self) -> PropertyTuple;
}

pub trait ToStyleValue {
  fn to_expr(&self, platform: Platform) -> PropertyTuple;
}