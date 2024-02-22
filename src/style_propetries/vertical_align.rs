use lightningcss::properties::{Property, font};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberProp, MemberExpr};
use swc_ecma_utils::member_expr;

use crate::{generate_prop_name, generate_expr_lit_str, generate_invalid_expr};

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct VerticalAlign {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Baseline,
  Sub,
  Super,
  TextTop,
  TextBottom,
  Middle,
  Top,
  Bottom,
  Invalid
}

impl ToExpr for VerticalAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!("align"),
      {
        match self.value {
          EnumValue::Baseline | EnumValue::Sub | EnumValue::Super | EnumValue::TextTop | EnumValue::TextBottom | EnumValue::Invalid => generate_invalid_expr!(),
          EnumValue::Middle | EnumValue::Top | EnumValue::Bottom => {
            Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: match self.value {
                  EnumValue::Middle => "Center",
                  EnumValue::Top => "Top",
                  EnumValue::Bottom => "Bottom",
                  _ => ""
                }.into(),
                optional: false,
              }),
            })
            .into()
          }
        }
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_prop_name!("textAlignVertical"),
      match self.value {
        EnumValue::Baseline => generate_invalid_expr!(),
        EnumValue::Sub => generate_invalid_expr!(),
        EnumValue::Super => generate_invalid_expr!(),
        EnumValue::TextTop | EnumValue::Top => generate_expr_lit_str!("top"),
        EnumValue::TextBottom | EnumValue::Bottom => generate_expr_lit_str!("bottom"),
        EnumValue::Middle => generate_expr_lit_str!("center"),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }
}

impl From<(String, &Property<'_>)> for VerticalAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    VerticalAlign {
      id: prop.0,
      value: match prop.1 {
        Property::VerticalAlign(value) => {
          match value {
            font::VerticalAlign::Keyword(keyword) => {
              match keyword {
                font::VerticalAlignKeyword::Baseline => EnumValue::Baseline,
                font::VerticalAlignKeyword::Sub => EnumValue::Sub,
                font::VerticalAlignKeyword::Super => EnumValue::Super,
                font::VerticalAlignKeyword::TextTop => EnumValue::TextTop,
                font::VerticalAlignKeyword::TextBottom => EnumValue::TextBottom,
                font::VerticalAlignKeyword::Middle => EnumValue::Middle,
                font::VerticalAlignKeyword::Top => EnumValue::Top,
                font::VerticalAlignKeyword::Bottom => EnumValue::Bottom,
              }
            },
            font::VerticalAlign::Length(_) => EnumValue::Invalid,
        }
        }
        _ => EnumValue::Invalid,
      }
    }
  }
}
