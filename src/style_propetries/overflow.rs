use lightningcss::properties::{
  Property, overflow:: OverflowKeyword,
};
use swc_ecma_ast::Expr;

use crate::generate_expr_lit_str;

use super::traits::ToExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
  Hidden,
  Visible,
  Scroll,
  Invalid,
}

impl From<&Property<'_>> for Overflow {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::Overflow(value) => {
        match value.x {
          OverflowKeyword::Hidden => Overflow::Hidden,
          OverflowKeyword::Visible => Overflow::Visible,
          OverflowKeyword::Clip => Overflow::Invalid,
          OverflowKeyword::Scroll => Overflow::Scroll,
          OverflowKeyword::Auto => Overflow::Invalid
        }
      },
      _ => Overflow::Invalid
    }
  }
}

impl ToExpr for Overflow {
  fn to_expr(&self) -> Expr {
    match self {
      Overflow::Hidden => generate_expr_lit_str!("hidden"),
      Overflow::Visible => generate_expr_lit_str!("visible"),
      Overflow::Scroll => generate_expr_lit_str!("scroll"),
      Overflow::Invalid => generate_expr_lit_str!("invalid"),
    }
  }

  fn to_rn_expr(&self) -> Expr {
    match self {
      Overflow::Hidden => generate_expr_lit_str!("hidden"),
      Overflow::Visible => generate_expr_lit_str!("visible"),
      Overflow::Scroll => generate_expr_lit_str!("scroll"),
      Overflow::Invalid => generate_expr_lit_str!("invalid"),
    }
  }

}
