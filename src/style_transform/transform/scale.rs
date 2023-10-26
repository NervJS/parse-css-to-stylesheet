use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

use crate::{
  impl_to_expr_for_transform_mem,
  style_transform::{
    traits::ToExpr,
    utils::{StringNumber, WrapCSSNumber},
  },
  utils::to_camel_case,
};

#[derive(Debug, Clone)]
pub struct Scale {
  pub x: Option<WrapCSSNumber>,
  pub y: Option<WrapCSSNumber>,
  pub z: Option<WrapCSSNumber>,
  pub center_x: Option<StringNumber>,
  pub center_y: Option<StringNumber>,
}

impl Scale {
  pub fn new() -> Self {
    Scale {
      x: None,
      y: None,
      z: None,
      center_x: None,
      center_y: None,
    }
  }
}

impl_to_expr_for_transform_mem!(Scale; x, y, z, center_x, center_y;);
