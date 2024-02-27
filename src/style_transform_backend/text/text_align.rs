use lightningcss::properties::{Property, text};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum TextAlign {
  Start,
  Center,
  End
}

impl ToExpr for TextAlign {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("TextAlign".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          TextAlign::Start => "Start",
          TextAlign::Center => "Center",
          TextAlign::End => "End",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }
}

impl From<&Property<'_>> for TextAlign {
  fn from(value: &Property<'_>) -> Self {
    let mut text_align = TextAlign::Start;
    match value {
      Property::TextAlign(value) => {
        match value {
          text::TextAlign::Left | text::TextAlign::Start => {
            text_align = TextAlign::Start
          },
          text::TextAlign::Right | text::TextAlign::End => {
            text_align = TextAlign::End
          },
          text::TextAlign::Center => {
            text_align = TextAlign::Center
          },
          _ => {}
        }
      }
      _ => {}
    }
    text_align
  }
}
