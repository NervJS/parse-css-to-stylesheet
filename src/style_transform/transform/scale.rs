use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, ObjectLit, PropOrSpread, KeyValueProp, Lit, Prop, PropName};

use crate::{
  impl_to_expr_for_transform_mem,
  style_transform::{
    traits::ToExpr,
    utils::WrapCSSNumber,
  },
  utils::to_camel_case,
};

#[derive(Debug, Clone)]
pub struct Scale {
  pub x: Option<WrapCSSNumber>,
  pub y: Option<WrapCSSNumber>,
  pub z: Option<WrapCSSNumber>,
}

impl Scale {
  pub fn new() -> Self {
    Scale {
      x: None,
      y: None,
      z: None,
    }
  }
}

impl_to_expr_for_transform_mem!(Scale; x, y, z;);
