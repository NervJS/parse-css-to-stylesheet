use std::{borrow::Borrow, vec};

use lightningcss::
  values::
    percentage::NumberOrPercentage
  
;
use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread};

use crate::{generate_expr_lit_num, utils::to_camel_case};


#[derive(Debug, Clone)]
pub struct Scale {
  pub x: Option<NumberOrPercentage>,
  pub y: Option<NumberOrPercentage>,
  pub z: Option<NumberOrPercentage>
}


impl Scale {
  pub fn new() -> Self {
    Scale {
      x: None,
      y: None,
      z: None,
    }
  }

  pub fn to_expr(&self) -> Vec<Expr> {

    let mut props = vec![];

    [("x", &self.x), ("y", &self.y), ("z", &self.z)].into_iter().for_each(|item| {
      if let (name, Some(side)) = item.borrow() {
        match &side {
          NumberOrPercentage::Number(value) => {
            props.push(
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new(Atom::new(*name), DUMMY_SP)),
                value: Box::new(generate_expr_lit_num!(*value as f64))
              })))
            );
          },
          _ => {}
        }
      }
    });

    let expr = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(swc_ecma_ast::Str {
            span: DUMMY_SP,
            value: "Scale".into(),
            raw: None
          })).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
          value: Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props,
          })
          .into(),
        }))),
      ]
    });

    vec![expr]
    
  }

  pub fn to_rn_expr(&self) -> Vec<Expr> {
    let mut props = vec![];
    [("x", &self.x), ("y", &self.y)].into_iter().for_each(|item| {
      if let (name, Some(side)) = item.borrow() {
        match &side {
            NumberOrPercentage::Number(value) => {
              props.push(
                Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident::new(to_camel_case(format!("{}{}", "scale-", name).as_str(), false).into(), DUMMY_SP)),
                    value: Box::new(generate_expr_lit_num!(*value as f64))
                  })))]
                })
              );
            },
            _ => {}
        }
      }
    });
    props
  }
}
