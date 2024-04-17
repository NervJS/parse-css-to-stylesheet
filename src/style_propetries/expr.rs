use swc_core::ecma::ast;

use super::{traits::ToExpr, unit::PropertyTuple};



#[derive(Debug, Clone)]
pub struct Expr(String, ast::Expr);

impl Expr {
  pub fn new(id: String, value: ast::Expr) -> Self {
    Self(id, value)
  }
}


impl ToExpr for Expr {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.0.clone(),
      self.1.clone()
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.0.clone(),
      self.1.clone()
    )
  }
}

