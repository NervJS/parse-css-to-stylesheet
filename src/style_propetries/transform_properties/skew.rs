use std::{borrow::Borrow, vec};

use lightningcss::{printer::PrinterOptions, traits::ToCss, values::angle::Angle};
use swc_core::ecma::ast::*;

use crate::{generate_expr_enum, style_propetries::transform_properties::ETransformType};
use swc_core::{atoms::Atom, common::DUMMY_SP};

use crate::{generate_expr_lit_num, generate_expr_lit_str, utils::to_camel_case};

#[derive(Debug, Clone)]
pub struct Skew {
  pub x: Option<Angle>,
  pub y: Option<Angle>,
}

impl Skew {
  pub fn new() -> Self {
    Skew { x: None, y: None }
  }

  pub fn to_expr_or_spread(&self) -> Option<ExprOrSpread> {
    let mut props = vec![];

    [("skewX", &self.x), ("skewY", &self.y)]
      .into_iter()
      .for_each(|item| {
        if let (name, Some(value)) = item.borrow() {
          if let Angle::Deg(angle) = value {
            props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new(Atom::new(*name), DUMMY_SP)),
              value: Box::new(generate_expr_lit_num!(Into::<f64>::into(*angle))),
            }))))
          }
        }
      });

    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
      value: Box::new(generate_expr_enum!(ETransformType::Skew)),
    }))));

    Some(ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Object(ObjectLit {
        span: Default::default(),
        props: props,
      })),
    })
  }
  pub fn to_rn_expr(&self) -> Vec<Expr> {
    let mut props = vec![];
    [("x", &self.x), ("y", &self.y)]
      .into_iter()
      .for_each(|item| {
        if let (name, Some(side)) = item.borrow() {
          props.push(Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new(
                to_camel_case(format!("{}{}", "skew-", name).as_str(), false).into(),
                DUMMY_SP,
              )),
              value: Box::new(generate_expr_lit_str!(side
                .to_css_string(PrinterOptions::default())
                .unwrap())),
            })))],
          }))
        }
      });
    props
  }
}
