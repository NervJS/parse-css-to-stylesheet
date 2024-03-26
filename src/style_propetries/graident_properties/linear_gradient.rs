
use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_lit_num, utils::fix_rgba};


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
  pub angle: Option<f32>,
  pub color_stops: Vec<(String, String)>,
  pub derection: Option<LinearGradientDirection>
}

impl LinearGradientItem {
  pub fn to_expr(&self) -> Expr {
    let mut props = vec![];
    if let Some(angle) = &self.angle {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("angle".into(), DUMMY_SP)),
        value: generate_expr_lit_num!(*angle as f64).into(),
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
                  Some(Expr::Lit(Lit::Str(Str::from(fix_rgba(item.0.clone())))).into()),
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
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}

#[derive(Debug, Clone)]
pub struct LinearGradient(pub Vec<LinearGradientItem>);




