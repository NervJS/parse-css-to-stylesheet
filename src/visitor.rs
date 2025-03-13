use std::{
  hash::{Hash, Hasher},
  vec,
};

use indexmap::IndexMap;
use swc_core::common::{Span, DUMMY_SP};
use swc_core::ecma::ast::*;

use crate::{constants::ValueFlag, generate_expr_lit_num, style_propetries::{
  style_value_type::StyleValueType,
  traits::ToStyleValue,
  unit::{Platform, PropertyTuple},
}};

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

// 辅助函数，处理样式值并添加到索引映射中
fn process_style_values(
  style_values: Vec<StyleValueType>,
  index_map: &mut IndexMap<u32, (Box<Expr>, ValueFlag)>,
  platform: &Platform,
  base_flag: ValueFlag,
) {
  style_values.into_iter().for_each(|style_value| {
    // 匹配style_value是否Variable类型
    let variable_flag = if let StyleValueType::Variable(_) = style_value.clone() {
      let prop = style_value.to_expr(platform.clone());
      if let PropertyTuple::One(id, expr) = prop {
        if let Expr::Invalid(_) = expr {
          return;
        }
        index_map.insert(id.clone() as u32, (Box::new(expr), base_flag | ValueFlag::VARIABLE));
      }
      return;
    } else {
      ValueFlag::NONE
    };
    
    let prop = style_value.to_expr(platform.clone());
    match prop {
      PropertyTuple::One(id, expr) => {
        if let Expr::Invalid(_) = expr {
          return;
        }
        index_map.insert(id.clone() as u32, (Box::new(expr), base_flag | variable_flag));
      }
      PropertyTuple::Array(prop_arr) => prop_arr.into_iter().for_each(|(id, expr)| {
        if let Expr::Invalid(_) = expr {
          return;
        }
        index_map.insert(id.clone() as u32, (Box::new(expr), base_flag | variable_flag));
      }),
      _ => {}
    }
  });
}

pub fn parse_style_values(
  value: Vec<StyleValueType>,
  import_value: Vec<StyleValueType>,
  platform: Platform,
) -> Vec<Option<ExprOrSpread>> {
  let mut prop_or_spread = vec![];

  // 使用有序表
  let mut index_map = IndexMap::new();
  
  // 处理普通样式值
  process_style_values(value, &mut index_map, &platform, ValueFlag::NONE);
  // 处理important样式值
  process_style_values(import_value, &mut index_map, &platform, ValueFlag::IMPORTANT);

  index_map.into_iter().for_each(|(id, (expr, value_type))| {
    let id_num = id.clone() as u32;
    let mut elems = vec![
      Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: id_num as f64,
          raw: None,
        }))),
      }),
      Some(ExprOrSpread { spread: None, expr }),
    ];
    if value_type.to_f64() > 0.0 {
      elems.push(Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: value_type.to_f64(),
          raw: None,
        }))),
      }));
    }
    prop_or_spread.push(Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: elems,
    }))
  });

  prop_or_spread
    .into_iter()
    .map(|expr| {
      Some(ExprOrSpread {
        spread: None,
        expr: Box::new(expr),
      })
    })
    .collect()
}
