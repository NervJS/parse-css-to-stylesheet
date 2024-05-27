
use std::{
  hash::{Hash, Hasher}, vec
};


use indexmap::IndexMap;
use swc_core::common::{Span, DUMMY_SP};
use swc_core::ecma::ast::*;

use crate::style_propetries::{style_value_type::StyleValueType, traits::ToStyleValue, unit::{Platform, PropertyTuple}};

#[derive(Eq, Clone, Copy, Debug)]
pub struct SpanKey(pub Span);

impl PartialEq for SpanKey {
  fn eq(&self, other: &Self) -> bool {
    self.0.lo == other.0.lo && self.0.hi == other.0.hi
  }
}

impl Hash for SpanKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.lo.hash(state);
    self.0.hi.hash(state);
  }
}


pub fn parse_style_values(value: Vec<StyleValueType>, platform: Platform) -> Vec<Option<ExprOrSpread>> {
  
  let mut prop_or_spread = vec![];

  // 使用有序表
  let mut index_map = IndexMap::new();
  
  value.into_iter().for_each(|style_value| {
    let prop = style_value.to_expr(platform.clone());
    match prop {
      PropertyTuple::One(id, expr) => {
        if let Expr::Invalid(_) = expr { return }
        index_map.insert(id.clone(), Box::new(expr));
      }
      PropertyTuple::Array(prop_arr) => {
        prop_arr.into_iter().for_each(|(id, expr)| {
          if let Expr::Invalid(_) = expr { return }
          index_map.insert(id.clone(), Box::new(expr));
        })
      }
    }
  });

  index_map.into_iter().for_each(|(id, expr)| {
    let id_num = id.clone() as u32;
    prop_or_spread.push(Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: vec![
        Some(ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: id_num as f64,
            raw: None
          })))
        }),
        Some(ExprOrSpread {
          spread: None,
          expr
        })
      ]
    }))
  });

  prop_or_spread.into_iter().map(|expr| {
    Some(ExprOrSpread {
      spread: None,
      expr: Box::new(expr)
    })
  }).collect()
}

