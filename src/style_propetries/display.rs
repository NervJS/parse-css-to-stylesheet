

use lightningcss::properties::{Property, display::{Display::{Keyword, Pair}, DisplayKeyword, DisplayInside}};
use swc_ecma_ast::Expr;

use crate::generate_expr_lit_str;

use super::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum Display {
  None,
  Flex,
  Invalid,
}

impl From<&Property<'_>> for Display {
  fn from(value: &Property<'_>) -> Self {
    if let Property::Display(value) = &value {
      match &value {
        Keyword(value) => match &value {
          DisplayKeyword::None => Display::None,
          _ => Display::Invalid,
        },
        Pair(value) => {
          if let DisplayInside::Flex(_) = value.inside {
            Display::Flex
          } else {
            Display::Invalid
          }
        }
      }
    } else {
      Display::Invalid
    }
  }
}


impl ToExpr for Display {
  fn to_expr(&self) -> Expr {
    match self {
      Display::None => generate_expr_lit_str!("none"),
      Display::Flex => generate_expr_lit_str!("flex"),
      Display::Invalid => generate_expr_lit_str!("flex"),
    }
  }

  fn to_rn_expr(&self) -> Expr {
    match self {
      Display::None => generate_expr_lit_str!("none"),
      Display::Flex => generate_expr_lit_str!("flex"),
      Display::Invalid => generate_expr_lit_str!("flex"),
    }
  }
}

