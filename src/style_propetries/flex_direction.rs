use lightningcss::properties::{flex::FlexDirection as LNFlexDirection, Property};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::{generate_expr_lit_str, generate_prop_name};

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FlexDirection {
  pub id: String,
  pub value: EnumValue
}


#[derive(Debug, Clone)]
pub enum EnumValue {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl From<(String, &Property<'_>)> for FlexDirection {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexDirection {
      id: prop.0,
      value: match prop.1 {
        Property::FlexDirection(value, _) => match value {
          LNFlexDirection::Row => EnumValue::Row,
          LNFlexDirection::RowReverse => EnumValue::RowReverse,
          LNFlexDirection::Column => EnumValue::Column,
          LNFlexDirection::ColumnReverse => EnumValue::ColumnReverse,
        },
        _ => EnumValue::Row,
      }
    }
  }
}

impl ToExpr for FlexDirection {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("FlexDirection".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Row => "Row",
            EnumValue::RowReverse => "RowReverse",
            EnumValue::Column => "Column",
            EnumValue::ColumnReverse => "ColumnReverse",
          }
          .into(),
          optional: false,
        }),
      })
      .into()
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!(*self.id),
      generate_expr_lit_str!(match self.value {
        EnumValue::Row => "row",
        EnumValue::RowReverse => "row-reverse",
        EnumValue::Column => "column",
        EnumValue::ColumnReverse => "column-reverse",
      })
    )
  }
}
