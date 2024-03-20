use swc_ecma_ast::Expr;

use super::{traits::ToExpr, unit::{generate_expr_with_css_input, Platform, PropertyTuple}};



#[derive(Debug, Clone)]
pub struct Variables(String, Expr);

impl Variables {
  pub fn new(id: String, value: Expr) -> Self {
    Self(id, value)
  }
}


impl ToExpr for Variables {
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

