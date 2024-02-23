use lightningcss::properties::{Property, font};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberProp, MemberExpr};

use crate::generate_expr_lit_str;

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FontStyle {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Normal,
  Italic
}

impl ToExpr for FontStyle {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(Ident::new("FontStyle".into(), DUMMY_SP))),
        prop: MemberProp::Ident(Ident {
          span: DUMMY_SP,
          sym: match self.value {
            EnumValue::Italic => "Italic",
            EnumValue::Normal => "Normal",
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
        EnumValue::Italic => "italic",
        EnumValue::Normal => "normal",
      })
    )
  }
}

impl From<(String, &Property<'_>)> for FontStyle {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontStyle {
      id: prop.0,
      value: match prop.1 {
        Property::FontStyle(value) => {
          match value {
            font::FontStyle::Italic => {
              EnumValue::Italic
            },
            font::FontStyle::Normal => {
              EnumValue::Normal
            },
            font::FontStyle::Oblique(_) => {
              EnumValue::Normal
            }
          }
        }
        _ => EnumValue::Normal
      }
    }
  }
}
