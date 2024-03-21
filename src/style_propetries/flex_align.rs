use lightningcss::properties::{
  align::{
    AlignContent as LNAlignContent, ContentDistribution, ContentPosition,
    JustifyContent as LNJustifyContent,
  },
  Property,
};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::generate_expr_lit_str;

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FlexAlign {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Start,
  Center,
  End,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}


impl From<(String, &Property<'_>)> for FlexAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexAlign {
      id: prop.0,
      value: match prop.1 {
        Property::JustifyContent(value, _) => match value {
          LNJustifyContent::ContentPosition { value, .. } => match value {
            ContentPosition::Start | ContentPosition::FlexStart => EnumValue::Start,
            ContentPosition::Center => EnumValue::Center,
            ContentPosition::End | ContentPosition::FlexEnd => EnumValue::End,
          },
          LNJustifyContent::ContentDistribution(value) => match value {
            ContentDistribution::SpaceBetween => EnumValue::SpaceBetween,
            ContentDistribution::SpaceAround => EnumValue::SpaceAround,
            ContentDistribution::SpaceEvenly => EnumValue::SpaceEvenly,
            _ => EnumValue::Start,
          },
          _ => EnumValue::Start,
        },
        Property::AlignContent(value, _) => match value {
          LNAlignContent::ContentPosition { value, .. } => match value {
            ContentPosition::Start | ContentPosition::FlexStart => EnumValue::Start,
            ContentPosition::Center => EnumValue::Center,
            ContentPosition::End | ContentPosition::FlexEnd => EnumValue::End,
          },
          LNAlignContent::ContentDistribution(value) => match value {
            ContentDistribution::SpaceBetween => EnumValue::SpaceBetween,
            ContentDistribution::SpaceAround => EnumValue::SpaceAround,
            ContentDistribution::SpaceEvenly => EnumValue::SpaceEvenly,
            _ => EnumValue::Start,
          },
          _ => EnumValue::Start,
        },
        _ => EnumValue::Start,
      }
    }
  }
}

// 转换成鸿蒙样式
impl ToExpr for FlexAlign {

  // 转换成鸿蒙样式
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One (
      self.id.to_string(),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("FlexAlign".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Start => "Start",
            EnumValue::Center => "Center",
            EnumValue::End => "End",
            EnumValue::SpaceBetween => "SpaceBetween",
            EnumValue::SpaceAround => "SpaceAround",
            EnumValue::SpaceEvenly => "SpaceEvenly",
          }
          .into(),
          optional: false,
        }),
      })
      .into(),
    )
  }

  // 转换成RN样式
  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One (
      self.id.to_string(),
      match &self.value {
        EnumValue::Start => generate_expr_lit_str!("flex-start"),
        EnumValue::Center => generate_expr_lit_str!("center"),
        EnumValue::End => generate_expr_lit_str!("flex-end"),
        EnumValue::SpaceBetween => generate_expr_lit_str!("space-between"),
        EnumValue::SpaceAround => generate_expr_lit_str!("space-around"),
        EnumValue::SpaceEvenly => generate_expr_lit_str!("space-evenly"),
      }
    )
  }
}
