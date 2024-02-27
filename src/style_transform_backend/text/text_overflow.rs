use lightningcss::properties::{Property, overflow};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp, ObjectLit, PropOrSpread, Prop, KeyValueProp, PropName};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum TextOverflow {
  Clip,
  Ellipsis,
  None
}

impl ToExpr for TextOverflow {
  fn to_expr(&self) -> Expr {

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("overflow".into(), DUMMY_SP)),
          value: Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new("TextOverflow".into(), DUMMY_SP))),
            prop: MemberProp::Ident(Ident {
              span: DUMMY_SP,
              sym: match self {
                TextOverflow::Clip => "Clip",
                TextOverflow::Ellipsis => "Ellipsis",
                TextOverflow::None => "None",
              }
              .into(),
              optional: false,
            }),
          }).into()
        }))),
      ],
    })
  }
}

impl From<&Property<'_>> for TextOverflow {
  fn from(value: &Property<'_>) -> Self {
    let mut text_overflows = TextOverflow::None;
    match value {
      Property::TextOverflow(value, _) => {
        match value {
          overflow::TextOverflow::Clip => {
            text_overflows = TextOverflow::Clip
          },
          overflow::TextOverflow::Ellipsis => {
            text_overflows = TextOverflow::Ellipsis
          }
        }
      }
      _ => {}
    }
    text_overflows
  }
}
