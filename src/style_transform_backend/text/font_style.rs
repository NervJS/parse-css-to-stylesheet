use lightningcss::properties::{Property, font};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberProp, MemberExpr};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum FontStyle {
  Normal,
  Italic
}

impl ToExpr for FontStyle {
  fn to_expr(&self) -> Expr {
    let font_weight: Expr;
    match self {
      FontStyle::Italic | FontStyle::Normal => {
        font_weight = Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FontStyle".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match self {
              FontStyle::Italic => "Italic",
              FontStyle::Normal => "Normal",
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

impl From<&Property<'_>> for FontStyle {
  fn from(value: &Property<'_>) -> Self {
    let mut font_weight = FontStyle::Normal;
    match value {
      Property::FontStyle(value) => {
        match value {
          font::FontStyle::Italic => {
            font_weight = FontStyle::Italic
          },
          font::FontStyle::Normal => {
            font_weight = FontStyle::Normal
          },
          font::FontStyle::Oblique(_) => {
            
          }
        }
      }
      _ => {}
    }
    font_weight
  }
}
