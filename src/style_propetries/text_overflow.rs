use lightningcss::properties::{Property, overflow};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp, ObjectLit, PropOrSpread, Prop, KeyValueProp, PropName};

use crate::{generate_invalid_expr, style_propetries::traits::ToExpr};

use super::unit::PropertyTuple;

#[derive(Debug, Clone)]
pub enum TextOverflow {
  Clip,
  Ellipsis,
  None
}

impl ToExpr for TextOverflow {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      "textOverflow".to_string(),
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
    )
  }
  
  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      "overflow".to_string(),
      generate_invalid_expr!()
    )
  }
}

impl From<(String, &Property<'_>)> for TextOverflow {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut text_overflows = TextOverflow::None;
    match value.1 {
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
