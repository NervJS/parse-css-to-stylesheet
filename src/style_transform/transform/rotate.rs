use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, ObjectLit, PropOrSpread, KeyValueProp, Lit, Prop, PropName};

use crate::{
  impl_to_expr_for_transform_mem,
  style_transform::{
    traits::ToExpr,
    utils::{StringNumber, WrapCSSNumber},
  },
  utils::to_camel_case,
};

#[derive(Debug, Clone)]
pub struct Rotate {
  pub x: Option<WrapCSSNumber>,
  pub y: Option<WrapCSSNumber>,
  pub z: Option<WrapCSSNumber>,
  pub angle: StringNumber,
  pub center_x: Option<StringNumber>,
  pub center_y: Option<StringNumber>,
}

impl Rotate {
  pub fn new() -> Self {
    Rotate {
      x: None,
      y: None,
      z: None,
      angle: StringNumber::Number(0.0),
      center_x: None,
      center_y: None,
    }
  }
}

impl_to_expr_for_transform_mem!(Rotate; x, y, z; angle);
