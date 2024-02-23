use lightningcss::properties::{Property, text};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::{style_propetries::traits::ToExpr, generate_expr_lit_str};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct TextAlign {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Start,
  Center,
  End,
  Justify
}

impl ToExpr for TextAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("TextAlign".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Start => "Start",
            EnumValue::Center => "Center",
            EnumValue::End => "End",
            EnumValue::Justify => "Start",
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
      match &self.value {
        EnumValue::Start => generate_expr_lit_str!("left"),
        EnumValue::Center => generate_expr_lit_str!("center"),
        EnumValue::End => generate_expr_lit_str!("right"),
        EnumValue::Justify => generate_expr_lit_str!("justify"),
      }
    )
  }
}

impl From<(String, &Property<'_>)> for TextAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    TextAlign {
      id: prop.0,
      value: match prop.1 {
        Property::TextAlign(value) => {
          match value {
            text::TextAlign::Left | text::TextAlign::Start => EnumValue::Start,
            text::TextAlign::Right | text::TextAlign::End => EnumValue::End,
            text::TextAlign::Center => EnumValue::Center,
            text::TextAlign::Justify => EnumValue::Justify,
            _ => EnumValue::Start
          }
        }
        _ => EnumValue::Start
      }
    }
  }
}
