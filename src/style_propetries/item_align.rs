use lightningcss::properties::{
  align::AlignItems as LNAlignItems, align::AlignSelf as LNAlignSelf, align::BaselinePosition,
  align::SelfPosition, Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::{generate_expr_lit_str, generate_prop_name};

use super::{traits::ToExpr, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct ItemAlign {
  pub id: String,
  pub value: EnumValue
}


#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
  Auto,
  Start,
  Center,
  End,
  Stretch,
  Baseline,
  Ignore,
}

impl From<(String, &Property<'_>)> for ItemAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    ItemAlign {
      id: prop.0,
      value: match prop.1 {
        Property::AlignItems(value, _) => match value {
          LNAlignItems::Stretch => EnumValue::Stretch,
          LNAlignItems::SelfPosition { value, .. } => match value {
            SelfPosition::Start | SelfPosition::FlexStart => EnumValue::Start,
            SelfPosition::Center => EnumValue::Center,
            SelfPosition::End | SelfPosition::FlexEnd => EnumValue::End,
            _ => EnumValue::Ignore,
          },
          LNAlignItems::BaselinePosition(value) => match value {
            BaselinePosition::Last => EnumValue::Ignore,
            _ => EnumValue::Baseline,
          },
          _ => EnumValue::Auto,
        },
        Property::AlignSelf(value, _) => match value {
          LNAlignSelf::Auto => EnumValue::Auto,
          LNAlignSelf::SelfPosition { value, .. } => match value {
            SelfPosition::Start | SelfPosition::FlexStart => EnumValue::Start,
            SelfPosition::Center => EnumValue::Center,
            SelfPosition::End | SelfPosition::FlexEnd => EnumValue::End,
            _ => EnumValue::Ignore,
          },
          LNAlignSelf::Stretch => EnumValue::Stretch,
          LNAlignSelf::BaselinePosition(value) => match value {
            BaselinePosition::Last => EnumValue::Ignore,
            _ => EnumValue::Baseline,
          },
          _ => EnumValue::Auto,
        },
        _ => EnumValue::Auto,
      }
    }
  }
}

impl ToExpr for ItemAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("ItemAlign".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Auto => "Auto",
            EnumValue::Start => "Start",
            EnumValue::Center => "Center",
            EnumValue::End => "End",
            EnumValue::Stretch => "Stretch",
            EnumValue::Baseline => "Baseline",
            EnumValue::Ignore => "",
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
      self.id.to_string(),
      generate_expr_lit_str!(
        match self.value {
          EnumValue::Auto => "auto",
          EnumValue::Start => "flex-start",
          EnumValue::Center => "center",
          EnumValue::End => "flex-end",
          EnumValue::Stretch => "stretch",
          EnumValue::Baseline => "baseline",
          _ => "",
        }
      )
    )
    
  }

}
