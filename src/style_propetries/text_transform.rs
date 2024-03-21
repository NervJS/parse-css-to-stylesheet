use lightningcss::properties::{Property, text};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::generate_expr_lit_str;

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct TextTransform {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  None,
  Uppercase,
  Lowercase,
  Capitalize
}

impl ToExpr for TextTransform {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      "textCase".to_string(),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("TextCase".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::None => "Normal",
            EnumValue::Lowercase => "Lowercase",
            EnumValue::Uppercase => "UpperCase",
            EnumValue::Capitalize => "Normal",
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
      generate_expr_lit_str!(match self.value {
        EnumValue::None => "none",
        EnumValue::Uppercase => "uppercase",
        EnumValue::Lowercase => "lowercase",
        EnumValue::Capitalize => "capitalize",
      })
    )
  }
}

impl From<(String, &Property<'_>)> for TextTransform {
  fn from(prop: (String, &Property<'_>)) -> Self {
    TextTransform {
      id: prop.0,
      value: match prop.1 {
        Property::TextTransform(value) => {
          match value.case {
            text::TextTransformCase::None => EnumValue::None,
            text::TextTransformCase::Uppercase => EnumValue::Uppercase,
            text::TextTransformCase::Lowercase => EnumValue::Lowercase,
            text::TextTransformCase::Capitalize => EnumValue::Capitalize
        }
        }
        _ => EnumValue::None,
      }
    }
  }
}
