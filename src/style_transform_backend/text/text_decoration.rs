use lightningcss::properties::{Property, text};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberProp, MemberExpr};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum TextDecoration {
  Underline,
  LineThrough,
  Overline,
  None
}

impl ToExpr for TextDecoration {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("TextDecorationType".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          TextDecoration::Underline => "Underline",
          TextDecoration::LineThrough => "LineThrough",
          TextDecoration::Overline => "Overline",
          _ => "None",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }
}

impl From<&Property<'_>> for TextDecoration {
  fn from(value: &Property<'_>) -> Self {
    let mut decoration = TextDecoration::None;
    match value {
      Property::TextDecoration(value, _) => {
        match value.line {
          text::TextDecorationLine::LineThrough => {
            decoration = TextDecoration::LineThrough
          },
          text::TextDecorationLine::Overline => {
            decoration = TextDecoration::Overline
          },
          text::TextDecorationLine::Underline => {
            decoration = TextDecoration::Underline
          },
          _ => {
            decoration = TextDecoration::None
          }
        }
      }
      _ => {}
    };
    decoration
  }
}
