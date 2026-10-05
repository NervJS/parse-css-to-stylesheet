use swc_core::ecma::ast;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::{Platform, PropertyTuple}};
use crate::generate_expr_lit_str;

#[derive(Debug, Clone)]
pub struct Expr(CSSPropertyType, ast::Expr);

impl Expr {
  pub fn new(id: CSSPropertyType, value: ast::Expr) -> Self {
    Self(id, value)
  }

  pub fn to_expr_for_platform(&self, _platform: Platform) -> PropertyTuple {
    if matches!(self.0, CSSPropertyType::Filter | CSSPropertyType::BackdropFilter) {
      if let ast::Expr::Lit(ast::Lit::Str(value)) = &self.1 {
        // PostCSS has already normalized design pixels. Preserve that contract
        // as lpx on both native targets, just like other compiled CSS lengths.
        // Keeping px here would make the string parser treat the doubled design
        // radius as vp, unlike the same authored inline style.
        return PropertyTuple::One(self.0, generate_expr_lit_str!(value.value));
      }
    }
    self.to_expr()
  }
}

impl ToExpr for Expr {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(self.0, self.1.clone())
  }
}
