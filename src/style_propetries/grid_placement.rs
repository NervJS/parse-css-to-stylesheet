use swc_core::{common::DUMMY_SP, ecma::ast::*};

use lightningcss::properties::{grid::GridLine, Property};

use crate::generate_expr_lit_num_grid;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

/// grid-row / grid-column / grid-row-start / grid-row-end /
/// grid-column-start / grid-column-end / grid-area
///
/// 数值化输出：DoubleArray。
/// - 单线（start/end 子属性、`grid-area` 四角）: 每条线 2 槽 `[type, index]`
///   - `grid-row: 1 / 3`       → [Line,1, Line,3]
///   - `grid-column: span 2`   → [Span,2, Auto,0]
/// - `grid-area` 一次性产出 4 条：8 槽 `[rs,ri, cs,ci, re,ri, ce,ci]`
///
/// 命名线（`grid-column: header / footer`）数值化不可行，该条声明在编译期被跳过。
#[derive(Debug, Clone)]
pub struct GridPlacement {
  pub id: CSSPropertyType,
  pub slots: Option<Vec<f64>>,
}

pub const GRID_LINE_TYPE_AUTO: f64 = 0.0;
pub const GRID_LINE_TYPE_LINE: f64 = 1.0;
pub const GRID_LINE_TYPE_SPAN: f64 = 2.0;

fn grid_line_to_slots(line: &GridLine) -> Option<[f64; 2]> {
  match line {
    GridLine::Auto => Some([GRID_LINE_TYPE_AUTO, 0.0]),
    GridLine::Line { index, name: None } => Some([GRID_LINE_TYPE_LINE, *index as f64]),
    GridLine::Span { index, name: None } => Some([GRID_LINE_TYPE_SPAN, *index as f64]),
    // 命名线：放弃数值化
    GridLine::Area { .. } | GridLine::Line { name: Some(_), .. } | GridLine::Span { name: Some(_), .. } => None,
  }
}

fn placement_pair(start: &GridLine, end: &GridLine) -> Option<Vec<f64>> {
  let s = grid_line_to_slots(start)?;
  let e = grid_line_to_slots(end)?;
  Some(vec![s[0], s[1], e[0], e[1]])
}

impl From<(String, &Property<'_>)> for GridPlacement {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let (id, slots) = match prop.1 {
      Property::GridRow(v) => (
        CSSPropertyType::GridRow,
        placement_pair(&v.start, &v.end),
      ),
      Property::GridColumn(v) => (
        CSSPropertyType::GridColumn,
        placement_pair(&v.start, &v.end),
      ),
      Property::GridRowStart(v) => (
        CSSPropertyType::GridRowStart,
        grid_line_to_slots(v).map(|s| s.to_vec()),
      ),
      Property::GridRowEnd(v) => (
        CSSPropertyType::GridRowEnd,
        grid_line_to_slots(v).map(|s| s.to_vec()),
      ),
      Property::GridColumnStart(v) => (
        CSSPropertyType::GridColumnStart,
        grid_line_to_slots(v).map(|s| s.to_vec()),
      ),
      Property::GridColumnEnd(v) => (
        CSSPropertyType::GridColumnEnd,
        grid_line_to_slots(v).map(|s| s.to_vec()),
      ),
      Property::GridArea(v) => {
        let rs = grid_line_to_slots(&v.row_start);
        let cs = grid_line_to_slots(&v.column_start);
        let re = grid_line_to_slots(&v.row_end);
        let ce = grid_line_to_slots(&v.column_end);
        let merged = match (rs, cs, re, ce) {
          (Some(rs), Some(cs), Some(re), Some(ce)) => Some(vec![
            rs[0], rs[1], cs[0], cs[1], re[0], re[1], ce[0], ce[1],
          ]),
          _ => None,
        };
        (CSSPropertyType::GridArea, merged)
      }
      _ => (CSSPropertyType::Invalid, None),
    };
    GridPlacement { id, slots }
  }
}

impl ToExpr for GridPlacement {
  fn to_expr(&self) -> PropertyTuple {
    match &self.slots {
      Some(slots) => {
        let elems: Vec<Option<ExprOrSpread>> = slots
          .iter()
          .map(|v| {
            Some(ExprOrSpread {
              spread: None,
              expr: Box::new(generate_expr_lit_num_grid!(*v)),
            })
          })
          .collect();
        PropertyTuple::One(
          self.id,
          Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems,
          }),
        )
      }
      // 命名线等降级场景：不产出该条声明
      None => PropertyTuple::One(
        CSSPropertyType::Invalid,
        Expr::Invalid(Invalid { span: DUMMY_SP }),
      ),
    }
  }
}
