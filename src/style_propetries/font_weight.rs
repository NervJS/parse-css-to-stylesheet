use lightningcss::properties::{Property, font};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{style_propetries::traits::ToExpr, generate_expr_lit_str, generate_expr_lit_num};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct FontWeight {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Number(f32),
  Bold,
  Bolder,
  Lighter,
  Normal
}

impl ToExpr for FontWeight {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      match &self.value {
        EnumValue::Number(num) => {
          generate_expr_lit_num!(*num as f64)
        }
        EnumValue::Bold | EnumValue::Bolder | EnumValue::Lighter | EnumValue::Normal => {
          Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new("FontWeight".into(), DUMMY_SP))),
            prop: MemberProp::Ident(Ident {
              span: DUMMY_SP,
              sym: match self.value {
                EnumValue::Bold => "Bold",
                EnumValue::Bolder => "Bolder",
                EnumValue::Lighter => "Lighter",
                EnumValue::Normal => "Normal",
                EnumValue::Number(_) => "",
              }
              .into(),
              optional: false,
            }),
          })
          .into()
        },
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id.to_string(),
      match &self.value {
        EnumValue::Bold => generate_expr_lit_str!("bold"),
        EnumValue::Bolder => generate_expr_lit_num!(900.0),
        EnumValue::Lighter => generate_expr_lit_num!(100.0),
        EnumValue::Normal => generate_expr_lit_str!("normal"),
        EnumValue::Number(num) => generate_expr_lit_num!(*num as f64)
      }
    )
  }
}

impl From<(String, &Property<'_>)> for FontWeight {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontWeight {
      id: prop.0,
      value: match prop.1 {
        Property::FontWeight(value) => {
          match value {
            font::FontWeight::Bolder => EnumValue::Bolder,
            font::FontWeight::Lighter => EnumValue::Lighter,
            font::FontWeight::Absolute(val) => {
              match val {
                font::AbsoluteFontWeight::Bold => {
                  EnumValue::Bold
                },
                font::AbsoluteFontWeight::Weight(num) => {
                  EnumValue::Number(*num)
                },
                font::AbsoluteFontWeight::Normal => {
                  EnumValue::Normal
                },
              }
            },
          }
        }
        _ => EnumValue::Normal
      }
    }
  }
}
