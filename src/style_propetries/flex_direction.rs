use lightningcss::properties::{flex::FlexDirection as LNFlexDirection, Property};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::generate_expr_lit_str;

use super::traits::ToExpr;


#[derive(Debug, Clone)]
pub enum FlexDirection {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl From<&Property<'_>> for FlexDirection {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::FlexDirection(value, _) => match value {
        LNFlexDirection::Row => FlexDirection::Row,
        LNFlexDirection::RowReverse => FlexDirection::RowReverse,
        LNFlexDirection::Column => FlexDirection::Column,
        LNFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
      },
      _ => FlexDirection::Row,
    }
  }
}

impl ToExpr for FlexDirection {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("FlexDirection".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          FlexDirection::Row => "Row",
          FlexDirection::RowReverse => "RowReverse",
          FlexDirection::Column => "Column",
          FlexDirection::ColumnReverse => "ColumnReverse",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }

  fn to_rn_expr(&self) -> Expr {
    generate_expr_lit_str!(match self {
      FlexDirection::Row => "row",
      FlexDirection::RowReverse => "row-reverse",
      FlexDirection::Column => "column",
      FlexDirection::ColumnReverse => "column-reverse",
    })
  }
}
