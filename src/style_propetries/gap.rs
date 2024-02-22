use std::borrow::Borrow;

use lightningcss::{
  properties::{Property, align::GapValue}, traits::ToCss, 
};
use swc_ecma_ast::Expr;

use crate::{generate_expr_by_length_percentage, generate_expr_lit_num, generate_expr_lit_str, generate_prop_name, generate_tpl_expr};

use super::{traits::ToExpr, unit::{Platform, PropertyTuple}};


macro_rules! generate_expr_gap {
  ($value:expr) => {
    match $value {
      GapValue::Normal => generate_expr_lit_num!(0.0),
      GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::ReactNative),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Gap {
  pub id: String,
  pub row: Option<GapValue>,
  pub column: Option<GapValue>,
  pub gap: Option<(GapValue, GapValue)>,
}

impl ToExpr for Gap {
  fn to_expr(&self) -> PropertyTuple {
    let mut expr = vec![];
    if let Some(row) = &self.row {
      expr.push((
        generate_prop_name!("rowGap"),
        match &row {
            GapValue::Normal => generate_expr_lit_num!(0.0),
            GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::Harmony),
        }
      ));
    }
    if let Some(column) = &self.column {
      expr.push((
        generate_prop_name!("columnGap"),
        match &column {
          GapValue::Normal => generate_expr_lit_num!(0.0),
          GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::Harmony),
        }
      ));
    }
    if let Some(gap) = &self.gap {
      expr.push((
        generate_prop_name!("rowGap"),
        match &gap.0 {
          GapValue::Normal => generate_expr_lit_num!(0.0),
          GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::Harmony),
        }
      ));
      expr.push((
        generate_prop_name!("columnGap"),
        match &gap.1 {
          GapValue::Normal => generate_expr_lit_num!(0.0),
          GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::Harmony),
        }
      ));
    }
    PropertyTuple::Array(expr)
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    let mut expr = vec![];
    if let Some(row) = &self.row {
      expr.push((
        generate_prop_name!("rowGap"),
        match &row {
            GapValue::Normal => generate_expr_lit_num!(0.0),
            GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::ReactNative),
        }
      ))
    }
    if let Some(column) = &self.column {
      expr.push((
        generate_prop_name!("columnGap"),
        match &column {
          GapValue::Normal => generate_expr_lit_num!(0.0),
          GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, Platform::ReactNative),
        }
      ))
    }
    if let Some(gap) = &self.gap {
      let items: Vec<swc_ecma_ast::Expr> = vec![
        generate_expr_gap!(gap.0.borrow()), 
        generate_expr_gap!(gap.1.borrow())
      ];

      expr.push((
        generate_prop_name!("gap"),
        generate_tpl_expr!(items)
      ))
    }
    PropertyTuple::Array(
      expr
    )
  }
}


impl From<(String, &Property<'_>)> for Gap {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut gap = Gap {
      id: prop.0,
      row: None,
      column: None,
      gap: None
    };
    match prop.1 {
      Property::Gap(value) => {
        gap.gap = Some((value.row.clone(), value.column.clone()));
      }
      Property::RowGap(value) => {
        gap.row = Some(value.clone())
      }
      Property::ColumnGap(value) => {
        gap.column = Some(value.clone())
      }
      _ => {}
    };
    gap
  }
}
