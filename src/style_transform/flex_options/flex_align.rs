use lightningcss::properties::{
  align::{
    AlignContent as LNAlignContent, ContentDistribution, ContentPosition,
    JustifyContent as LNJustifyContent,
  },
  Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FlexAlign {
  Start,
  Center,
  End,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}

impl From<&str> for FlexAlign {
  fn from(value: &str) -> Self {
    match value {
      "flex-start" | "start" => FlexAlign::Start,
      "center" => FlexAlign::Center,
      "flex-end" | "end" => FlexAlign::End,
      "space-between" => FlexAlign::SpaceBetween,
      "space-around" => FlexAlign::SpaceAround,
      "space-evenly" => FlexAlign::SpaceEvenly,
      _ => FlexAlign::Start,
    }
  }
}

impl From<&Property<'_>> for FlexAlign {
  fn from(value: &Property<'_>) -> Self {
    match value {
      Property::JustifyContent(value, _) => match value {
        LNJustifyContent::ContentPosition { value, .. } => match value {
          ContentPosition::Start | ContentPosition::FlexStart => FlexAlign::Start,
          ContentPosition::Center => FlexAlign::Center,
          ContentPosition::End | ContentPosition::FlexEnd => FlexAlign::End,
        },
        LNJustifyContent::ContentDistribution(value) => match value {
          ContentDistribution::SpaceBetween => FlexAlign::SpaceBetween,
          ContentDistribution::SpaceAround => FlexAlign::SpaceAround,
          ContentDistribution::SpaceEvenly => FlexAlign::SpaceEvenly,
          _ => FlexAlign::Start,
        },
        _ => FlexAlign::Start,
      },
      Property::AlignContent(value, _) => match value {
        LNAlignContent::ContentPosition { value, .. } => match value {
          ContentPosition::Start | ContentPosition::FlexStart => FlexAlign::Start,
          ContentPosition::Center => FlexAlign::Center,
          ContentPosition::End | ContentPosition::FlexEnd => FlexAlign::End,
        },
        LNAlignContent::ContentDistribution(value) => match value {
          ContentDistribution::SpaceBetween => FlexAlign::SpaceBetween,
          ContentDistribution::SpaceAround => FlexAlign::SpaceAround,
          ContentDistribution::SpaceEvenly => FlexAlign::SpaceEvenly,
          _ => FlexAlign::Start,
        },
        _ => FlexAlign::Start,
      },
      _ => FlexAlign::Start,
    }
  }
}

impl ToExpr for FlexAlign {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("FlexAlign".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          FlexAlign::Start => "Start",
          FlexAlign::Center => "Center",
          FlexAlign::End => "End",
          FlexAlign::SpaceBetween => "SpaceBetween",
          FlexAlign::SpaceAround => "SpaceAround",
          FlexAlign::SpaceEvenly => "SpaceEvenly",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }
}
