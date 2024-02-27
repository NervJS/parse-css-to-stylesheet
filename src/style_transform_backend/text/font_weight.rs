use lightningcss::properties::{Property, font};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number, Ident, MemberProp, MemberExpr};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FontWeight {
  Number(f32),
  Bold,
  Bolder,
  Lighter,
  Normal
}

impl ToExpr for FontWeight {
  fn to_expr(&self) -> Expr {
    let font_weight: Expr;
    match self {
      FontWeight::Number(num) => {
        font_weight = Expr::Lit(Lit::Num(Number::from(*num as f64)));
      }
      FontWeight::Bold | FontWeight::Bolder | FontWeight::Lighter | FontWeight::Normal => {
        font_weight = Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FontWeight".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match self {
              FontWeight::Bold => "Bold",
              FontWeight::Bolder => "Bolder",
              FontWeight::Lighter => "Lighter",
              FontWeight::Normal => "Normal",
              FontWeight::Number(_) => "",
            }
            .into(),
            optional: false,
          }),
        })
        .into()
      },
    }
    font_weight
  }
}

impl From<&Property<'_>> for FontWeight {
  fn from(value: &Property<'_>) -> Self {
    let mut font_weight = FontWeight::Normal;
    match value {
      Property::FontWeight(value) => {
        match value {
          font::FontWeight::Bolder => {
            font_weight = FontWeight::Bolder
          },
          font::FontWeight::Lighter => {
            font_weight = FontWeight::Lighter
          },
          font::FontWeight::Absolute(val) => {
            match val {
              font::AbsoluteFontWeight::Bold => {
                font_weight = FontWeight::Bold
              },
              font::AbsoluteFontWeight::Weight(num) => {
                font_weight = FontWeight::Number(*num)
              },
              font::AbsoluteFontWeight::Normal => {
                font_weight = FontWeight::Normal
              },
            }
          },
        }
      }
      _ => {}
    }
    font_weight
  }
}
