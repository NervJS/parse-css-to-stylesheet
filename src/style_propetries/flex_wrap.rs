use lightningcss::properties::{flex::FlexWrap as LNFlexWrap, Property};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::{style_propetries::traits::ToExpr, generate_expr_lit_str, generate_ident};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct FlexWrap {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Wrap,
  WrapReverse,
  NoWrap,
}

impl From<(String, &Property<'_>)> for FlexWrap {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexWrap {
      id: prop.0,
      value: match prop.1 {
        Property::FlexWrap(value, _) => match value {
          LNFlexWrap::Wrap => EnumValue::Wrap,
          LNFlexWrap::WrapReverse => EnumValue::WrapReverse,
          LNFlexWrap::NoWrap => EnumValue::NoWrap,
        },
        _ => EnumValue::NoWrap,
      }
    }
  }
}

impl ToExpr for FlexWrap {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_ident!(&self.id),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("FlexWrap".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Wrap => "Wrap",
            EnumValue::WrapReverse => "WrapReverse",
            EnumValue::NoWrap => "NoWrap",
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
      generate_ident!(&self.id),
      match &self.value {
        EnumValue::Wrap => generate_expr_lit_str!("wrap"),
        EnumValue::WrapReverse => generate_expr_lit_str!("wrap-reverse"),
        EnumValue::NoWrap => generate_expr_lit_str!("nowrap"),
      }
    )
  }
}
