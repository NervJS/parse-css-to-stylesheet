use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr};

use crate::style_transform::{traits::ToExpr, utils::WrapCSSNumber};

#[derive(Debug, Clone)]
pub struct Matrix {
  pub m00: WrapCSSNumber,
  pub m01: WrapCSSNumber,
  pub m02: WrapCSSNumber,
  pub m03: WrapCSSNumber,
  pub m10: WrapCSSNumber,
  pub m11: WrapCSSNumber,
  pub m12: WrapCSSNumber,
  pub m13: WrapCSSNumber,
  pub m20: WrapCSSNumber,
  pub m21: WrapCSSNumber,
  pub m22: WrapCSSNumber,
  pub m23: WrapCSSNumber,
  pub m30: WrapCSSNumber,
  pub m31: WrapCSSNumber,
  pub m32: WrapCSSNumber,
  pub m33: WrapCSSNumber,
}

impl Matrix {
  pub fn new() -> Self {
    Matrix {
      m00: WrapCSSNumber(1.0),
      m01: WrapCSSNumber(0.0),
      m02: WrapCSSNumber(0.0),
      m03: WrapCSSNumber(0.0),
      m10: WrapCSSNumber(0.0),
      m11: WrapCSSNumber(1.0),
      m12: WrapCSSNumber(0.0),
      m13: WrapCSSNumber(0.0),
      m20: WrapCSSNumber(0.0),
      m21: WrapCSSNumber(0.0),
      m22: WrapCSSNumber(1.0),
      m23: WrapCSSNumber(0.0),
      m30: WrapCSSNumber(0.0),
      m31: WrapCSSNumber(0.0),
      m32: WrapCSSNumber(0.0),
      m33: WrapCSSNumber(1.0),
    }
  }
}

impl ToExpr for Matrix {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: vec![
        Some(self.m00.to_expr().into()),
        Some(self.m01.to_expr().into()),
        Some(self.m02.to_expr().into()),
        Some(self.m03.to_expr().into()),
        Some(self.m10.to_expr().into()),
        Some(self.m11.to_expr().into()),
        Some(self.m12.to_expr().into()),
        Some(self.m13.to_expr().into()),
        Some(self.m20.to_expr().into()),
        Some(self.m21.to_expr().into()),
        Some(self.m22.to_expr().into()),
        Some(self.m23.to_expr().into()),
        Some(self.m30.to_expr().into()),
        Some(self.m31.to_expr().into()),
        Some(self.m32.to_expr().into()),
        Some(self.m33.to_expr().into()),
      ],
    })
  }
}
