use swc_ecma_ast::Expr;

pub trait ToExpr {
  fn to_expr(&self) -> Expr;
}
