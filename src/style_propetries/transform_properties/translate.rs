use std::{borrow::Borrow, vec};

use lightningcss::
  values::{
    length::{Length, LengthValue}, percentage::DimensionPercentage
  }
;
use swc_atoms::Atom;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread};

use crate::{style_propetries::unit::{generate_expr_by_length_value, Platform}, utils::to_camel_case};


#[derive(Debug, Clone)]
pub struct Translate {
  pub x: Option<DimensionPercentage<LengthValue>>,
  pub y: Option<DimensionPercentage<LengthValue>>,
  pub z: Option<Length>,
}


impl Translate {
  pub fn new() -> Self {
    Translate {
      x: None,
      y: None,
      z: None,
    }
  }

  pub fn to_expr(&self) -> Vec<Expr> {

    let mut props = vec![];

    [("x", &self.x), ("y", &self.y)].into_iter().for_each(|item| {
      if let (name, Some(side)) = item.borrow() {
        match &side {
          DimensionPercentage::Dimension(value) => {
            props.push(
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new(Atom::new(*name), DUMMY_SP)),
                value: Box::new(generate_expr_by_length_value(value, Platform::Harmony))
              })))
            );
          },
          _ => {}
        }
      }
    });

    if let Some(z) = &self.z {
      match z {
        Length::Value(val) => props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(stringify!("z").into(), DUMMY_SP)),
          value: Box::new(generate_expr_by_length_value(&val, Platform::Harmony))
        })))),
        _ => {}
      };
    }

    let expr = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(swc_ecma_ast::Str {
            span: DUMMY_SP,
            value: "Translate".into(),
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
          DimensionPercentage::Dimension(value) => {
            props.push(
              Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new(to_camel_case(format!("{}{}", "translate-", name).as_str(), false).into(), DUMMY_SP)),
                  value: Box::new(generate_expr_by_length_value(value, Platform::ReactNative))
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
