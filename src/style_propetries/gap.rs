use lightningcss::properties::{Property, align::GapValue};

use crate::{generate_expr_by_length_percentage, generate_expr_lit_num, generate_expr_lit_str};

use super::{traits::ToExpr, unit::{Platform, PropertyTuple}};


macro_rules! generate_expr_gap {
  ($value:expr, $platform:expr) => {
    match $value {
      GapValue::Normal => generate_expr_lit_num!(0.0),
      GapValue::LengthPercentage(val) => generate_expr_by_length_percentage!(val, $platform),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Gap {
  pub id: String,
  pub row: Option<GapValue>,
  pub column: Option<GapValue>,
}

impl ToExpr for Gap {
  fn to_expr(&self) -> PropertyTuple {
    let mut expr = vec![];
    if let Some(row) = &self.row {
      expr.push((
        "rowGap".to_string(),
        generate_expr_gap!(row, Platform::Harmony)
      ));
    }
    if let Some(column) = &self.column {
      expr.push((
        "columnGap".to_string(),
        generate_expr_gap!(column, Platform::Harmony)
      ));
    }
    PropertyTuple::Array(expr)
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    let mut expr = vec![];

    if self.id == "gap" && self.row == self.column {
      expr.push((
        "gap".to_string(),
        generate_expr_gap!(self.row.as_ref().unwrap(), Platform::ReactNative)
      ))
    } else {
      if let Some(row) = &self.row {
        expr.push((
          "rowGap".to_string(),
          generate_expr_gap!(row, Platform::ReactNative)
        ))
      }
      if let Some(column) = &self.column {
        expr.push((
          "columnGap".to_string(),
          generate_expr_gap!(column, Platform::ReactNative)
        ))
      }
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
      column: None
    };
    match prop.1 {
      Property::Gap(value) => {
        gap.row = Some(value.row.clone());
        gap.column = Some(value.column.clone());
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
