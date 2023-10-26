use lightningcss::values::number::CSSNumber;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number, Str};

use super::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum StringNumber {
  String(String),
  Number(CSSNumber),
}

impl ToExpr for StringNumber {
  fn to_expr(&self) -> Expr {
    match self {
      StringNumber::String(value) => Expr::Lit(Lit::Str(Str::from(value.to_string()))).into(),
      StringNumber::Number(value) => Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: *value as f64,
        raw: None,
      }))
      .into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct WrapCSSNumber(pub CSSNumber);

impl ToExpr for WrapCSSNumber {
  fn to_expr(&self) -> Expr {
    Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value: self.0 as f64,
      raw: None,
    }))
    .into()
  }
}
