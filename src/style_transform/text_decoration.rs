use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ComputedPropName, Expr, Ident, KeyValueProp, Lit, MemberExpr, MemberProp, ObjectLit, Prop,
  PropName, PropOrSpread, Str,
};

use crate::utils::to_camel_case;

use super::traits::ToExpr;

#[derive(Debug, Clone)]
pub struct TextDecoration {
  pub kind: String,
  pub color: String,
}

impl TextDecoration {
  pub fn new() -> Self {
    TextDecoration {
      kind: "none".into(),
      color: "black".to_string(),
    }
  }
}

impl Default for TextDecoration {
  fn default() -> Self {
    TextDecoration::new()
  }
}

impl ToExpr for TextDecoration {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
          value: Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(
              "TextDecorationType".into(),
              DUMMY_SP,
            ))),
            prop: MemberProp::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Expr::Lit(Lit::Str(Str::from(to_camel_case(self.kind.as_str(), true)))).into(),
            }),
          })
          .into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("color".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.color.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<(&Property<'_>, Option<&Property<'_>>)> for TextDecoration {
  fn from(value: (&Property<'_>, Option<&Property<'_>>)) -> Self {
    let (text_decoration, color) = value;
    match &text_decoration {
      Property::TextDecoration(decoration, _) => TextDecoration {
        kind: decoration
          .line
          .to_css_string(PrinterOptions::default())
          .unwrap(),
        color: color
          .map(|color| {
            color
              .value_to_css_string(PrinterOptions {
                minify: false,
                targets: Targets {
                  include: Features::HexAlphaColors,
                  ..Targets::default()
                },
                ..PrinterOptions::default()
              })
              .unwrap()
          })
          .unwrap_or("black".to_string()),
      },
      _ => TextDecoration::default(),
    }
  }
}
