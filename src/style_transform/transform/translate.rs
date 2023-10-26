use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

use crate::{
  impl_to_expr_for_transform_mem,
  style_transform::{traits::ToExpr, utils::StringNumber},
  utils::to_camel_case,
};

#[derive(Debug, Clone)]
pub struct Translate {
  pub x: Option<StringNumber>,
  pub y: Option<StringNumber>,
  pub z: Option<StringNumber>,
}

impl Translate {
  pub fn new() -> Self {
    Translate {
      x: None,
      y: None,
      z: None,
    }
  }
}

impl_to_expr_for_transform_mem!(Translate; x, y;);
