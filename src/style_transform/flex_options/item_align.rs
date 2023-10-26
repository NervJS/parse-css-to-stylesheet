use lightningcss::properties::{
  align::AlignItems as LNAlignItems, align::AlignSelf as LNAlignSelf, align::BaselinePosition,
  align::SelfPosition, Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemAlign {
  Auto,
  Start,
  Center,
  End,
  Stretch,
  Baseline,
  Ignore,
}

impl From<&str> for ItemAlign {
  fn from(value: &str) -> Self {
    match value {
      "auto" => ItemAlign::Auto,
      "flex-start" | "start" => ItemAlign::Start,
      "center" => ItemAlign::Center,
      "flex-end" | "end" => ItemAlign::End,
      "stretch" => ItemAlign::Stretch,
      "baseline" => ItemAlign::Baseline,
      _ => ItemAlign::Auto,
    }
  }
}

impl From<&Property<'_>> for ItemAlign {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::AlignItems(value, _) => match value {
        LNAlignItems::Stretch => ItemAlign::Stretch,
        LNAlignItems::SelfPosition { value, .. } => match value {
          SelfPosition::Start => ItemAlign::Start,
          SelfPosition::Center => ItemAlign::Center,
          SelfPosition::End => ItemAlign::End,
          _ => ItemAlign::Ignore,
        },
        LNAlignItems::BaselinePosition(value) => match value {
          BaselinePosition::Last => ItemAlign::Ignore,
          _ => ItemAlign::Baseline,
        },
        _ => ItemAlign::Auto,
      },
      Property::AlignSelf(value, _) => match value {
        LNAlignSelf::Auto => ItemAlign::Auto,
        LNAlignSelf::SelfPosition { value, .. } => match value {
          SelfPosition::Start => ItemAlign::Start,
          SelfPosition::Center => ItemAlign::Center,
          SelfPosition::End => ItemAlign::End,
          _ => ItemAlign::Ignore,
        },
        LNAlignSelf::BaselinePosition(value) => match value {
          BaselinePosition::Last => ItemAlign::Ignore,
          _ => ItemAlign::Baseline,
        },
        _ => ItemAlign::Auto,
      },
      _ => ItemAlign::Auto,
    }
  }
}

impl ToExpr for ItemAlign {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("ItemAlign".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          ItemAlign::Auto => "Auto",
          ItemAlign::Start => "Start",
          ItemAlign::Center => "Center",
          ItemAlign::End => "End",
          ItemAlign::Stretch => "Stretch",
          ItemAlign::Baseline => "Baseline",
          ItemAlign::Ignore => "",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }
}
