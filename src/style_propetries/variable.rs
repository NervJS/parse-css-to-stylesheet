use swc_core::ecma::ast;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Variable(CSSPropertyType, ast::Expr);

impl Variable {
  pub fn new(id: CSSPropertyType, value: ast::Expr) -> Self {
    Self(id, value)
  }
}

impl ToExpr for Variable {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(self.0, self.1.clone())
  }
}
