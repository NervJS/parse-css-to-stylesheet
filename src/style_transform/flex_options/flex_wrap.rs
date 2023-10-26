use lightningcss::properties::{flex::FlexWrap as LNFlexWrap, Property};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FlexWrap {
  Wrap,
  WrapReverse,
  NoWrap,
}

impl From<&str> for FlexWrap {
  fn from(value: &str) -> Self {
    match value {
      "wrap" => FlexWrap::Wrap,
      "wrap-reverse" => FlexWrap::WrapReverse,
      "nowrap" => FlexWrap::NoWrap,
      _ => FlexWrap::NoWrap,
    }
  }
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
}
