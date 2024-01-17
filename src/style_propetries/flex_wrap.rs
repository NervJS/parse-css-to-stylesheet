use lightningcss::properties::{flex::FlexWrap as LNFlexWrap, Property};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::{style_propetries::traits::ToExpr, generate_expr_lit_str};

#[derive(Debug, Clone)]
pub enum FlexWrap {
  Wrap,
  WrapReverse,
  NoWrap,
}

impl From<&Property<'_>> for FlexWrap {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::FlexWrap(value, _) => match value {
        LNFlexWrap::Wrap => FlexWrap::Wrap,
        LNFlexWrap::WrapReverse => FlexWrap::WrapReverse,
        LNFlexWrap::NoWrap => FlexWrap::NoWrap,
      },
      _ => FlexWrap::NoWrap,
    }
  }
}

impl ToExpr for FlexWrap {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("FlexWrap".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          FlexWrap::Wrap => "Wrap",
          FlexWrap::WrapReverse => "WrapReverse",
          FlexWrap::NoWrap => "NoWrap",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }

  fn to_rn_expr(&self) -> Expr {
    generate_expr_lit_str!(match self {
      FlexWrap::Wrap => "wrap",
      FlexWrap::WrapReverse => "wrap-reverse",
      FlexWrap::NoWrap => "nowrap",
    })
  }
}
