use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Bool, ComputedPropName, Expr, ExprOrSpread, Ident, KeyValueProp, Lit, MemberExpr,
  MemberProp, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum LinearGradientDirection {
  Left,
  Right,
  Top,
  Bottom,
  LeftTop,
  LeftBottom,
  RightTop,
  RightBottom,
}

#[derive(Debug, Clone)]
pub struct LinearGradientItem {
  pub angle: Option<String>,
  pub color_stops: Vec<(String, String)>,
  pub derection: Option<LinearGradientDirection>,
  pub repeating: bool,
}

impl ToExpr for LinearGradientItem {
  fn to_expr(&self) -> Expr {
    let mut props = vec![];
    if let Some(angle) = &self.angle {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("angle".into(), DUMMY_SP)),
        value: Expr::Lit(Lit::Str(Str::from(angle.to_string()))).into(),
      }))));
    }
    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("colors".into(), DUMMY_SP)),
      value: Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: self
          .color_stops
          .iter()
          .map(|item| {
            Some(ExprOrSpread {
              spread: None,
              expr: Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: vec![
                  Some(Expr::Lit(Lit::Str(Str::from(item.0.to_string()))).into()),
                  Some(Expr::Lit(Lit::Str(Str::from(item.1.to_string()))).into()),
                ],
              })
              .into(),
            })
          })
          .collect::<Vec<_>>(),
      })
      .into(),
    }))));
    if let Some(derection) = &self.derection {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("direction".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new(
            "GradientDirection".into(),
            DUMMY_SP,
          ))),
          prop: MemberProp::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Expr::Lit(Lit::Str(Str::from(match derection {
              LinearGradientDirection::Left => "Left",
              LinearGradientDirection::Right => "Right",
              LinearGradientDirection::Top => "Top",
              LinearGradientDirection::Bottom => "Bottom",
              LinearGradientDirection::LeftTop => "LeftTop",
              LinearGradientDirection::LeftBottom => "LeftBottom",
              LinearGradientDirection::RightTop => "RightTop",
              LinearGradientDirection::RightBottom => "RightBottom",
            })))
            .into(),
          }),
        })
        .into(),
      }))));
    }
    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("repeating".into(), DUMMY_SP)),
      value: Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value: self.repeating,
      }))
      .into(),
    }))));
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}

#[derive(Debug, Clone)]
pub struct LinearGradient(pub Vec<LinearGradientItem>);

impl ToExpr for LinearGradient {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .map(|item| Some(item.to_expr().into()))
        .collect::<Vec<_>>(),
    })
  }
}
