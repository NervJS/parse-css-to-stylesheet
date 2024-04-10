use std::{borrow::Borrow, vec};

use lightningcss::{printer::PrinterOptions, traits::ToCss, values::
    angle::Angle
  }
;
use swc_core::ecma::ast::*;
use swc_core::{
  atoms::Atom,
  common::DUMMY_SP
};

use crate::{generate_expr_lit_num, generate_expr_lit_str, utils::to_camel_case};


#[derive(Debug, Clone)]
pub struct Rotate {
  pub x: Option<f32>,
  pub y: Option<f32>,
  pub z: Option<f32>,
  pub rotate: Option<f32>,
  pub angle: Angle,
}


impl Rotate {
  pub fn new() -> Self {
    Rotate {
      x: None,
      y: None,
      z: None,
      rotate: None,
      angle: Angle::Deg(0.0),
    }
  }

  pub fn to_expr(&self) -> PropOrSpread {

    let mut props = vec![];

    [("x", &self.x), ("y", &self.y), ("z", &self.z), ("z", &self.rotate)].into_iter().for_each(|item| {
      if let (name, Some(value)) = item.borrow() {
        props.push(
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new(Atom::new(*name), DUMMY_SP)),
            value: Box::new(generate_expr_lit_num!(*value as f64))
          })))
        )
      }
    });

    if let Angle::Deg(angle) = self.angle {
      props.push(
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("angle".into(), DUMMY_SP)),
          value: Box::new(generate_expr_lit_num!(angle as f64))
        })))
      );
    }

    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("Rotate".into(), DUMMY_SP)),
      value: Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
      })
      .into(),
    })))
  }

  pub fn to_rn_expr(&self) -> Vec<Expr> {
    let mut props = vec![];
    [("-x", &self.x), ("-y", &self.y), ("-z", &self.z), ("", &self.rotate)].into_iter().for_each(|item| {
      if let (name, Some(_)) = item.borrow() {
        props.push(
          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new(to_camel_case(format!("{}{}", "rotate", name).as_str(), false).into(), DUMMY_SP)),
              value: Box::new(generate_expr_lit_str!(self.angle.to_css_string(PrinterOptions::default()).unwrap()))
            })))]
          })
        );
      }
    });
    props
  }
}
