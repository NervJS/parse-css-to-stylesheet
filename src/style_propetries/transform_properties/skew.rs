use std::{borrow::Borrow, vec};

use lightningcss::{printer::PrinterOptions, traits::ToCss, values::angle::Angle}
;
use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_lit_str, utils::to_camel_case};

#[derive(Debug, Clone)]
pub struct Skew {
  pub x: Option<Angle>,
  pub y: Option<Angle>,
}


impl Skew {
  pub fn new() -> Self {
    Skew {
      x: None,
      y: None,
    }
  }
  pub fn to_rn_expr(&self) -> Vec<Expr> {
    let mut props = vec![];
    [("x", &self.x), ("y", &self.y)].into_iter().for_each(|item| {
      if let (name, Some(side)) = item.borrow() {
        props.push(
          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new(to_camel_case(format!("{}{}", "skew-", name).as_str(), false).into(), DUMMY_SP)),
              value: Box::new(generate_expr_lit_str!(side.to_css_string(PrinterOptions::default()).unwrap()))
            })))]
          })
        )
      }
    });
    props
  }
}
