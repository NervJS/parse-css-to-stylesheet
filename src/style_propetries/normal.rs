

use lightningcss::properties::{Property, display::{Display::{Keyword, Pair}, DisplayKeyword, DisplayInside}};
use swc_ecma_ast::{Expr, Lit};

use crate::{generate_expr_lit_str, utils::convert_px_to_units};

use super::{traits::ToExpr, unit::generate_expr_with_css_input};

#[derive(Debug, Clone)]
pub struct Normal(String);

impl Normal {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}


impl ToExpr for Normal {
  fn to_expr(&self) -> Expr {
    convert_px_to_units(self.0.clone())
  }

  fn to_rn_expr(&self) -> Expr {
    generate_expr_with_css_input(self.0.clone())
  }
}

