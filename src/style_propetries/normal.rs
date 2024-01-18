

use swc_ecma_ast::Expr;

use crate::{utils::convert_px_to_units, generate_ident};

use super::{traits::ToExpr, unit::{generate_expr_with_css_input, PropertyTuple}};



#[derive(Debug, Clone)]
pub struct Normal(String, String);

impl Normal {
  pub fn new(id: String, value: String) -> Self {
    Self(id, value)
  }
}


impl ToExpr for Normal {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_ident!(self.0.clone()),
      generate_expr_with_css_input(self.1.clone())
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_ident!(self.0.clone()),
      generate_expr_with_css_input(self.1.clone())
    )
  }
}

