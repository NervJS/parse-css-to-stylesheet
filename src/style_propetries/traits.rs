use swc_ecma_ast::Expr;

use super::unit::Platform;

pub trait ToExpr {
  fn to_expr(&self) -> Expr;

  fn to_rn_expr(&self) -> Expr {
    self.to_expr()
  }
}

pub trait ToStyleValue {
  fn to_expr(&self, platform: Platform) -> Expr;
}