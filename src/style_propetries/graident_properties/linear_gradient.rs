use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::style_propetries::style_property_enum;
use crate::{generate_expr_enum, generate_expr_lit_num};

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
  pub color_stops: Vec<(Expr, Expr)>,
  pub derection: Option<LinearGradientDirection>,
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
                  Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(item.0.clone()),
                  }),
                  Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(item.1.clone()),
                  }),
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
        value: generate_expr_enum!(
          match derection {
            LinearGradientDirection::Left => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT,
            LinearGradientDirection::Right => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT,
            LinearGradientDirection::Top => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_TOP,
            LinearGradientDirection::Bottom => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_BOTTOM,
            LinearGradientDirection::LeftTop => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT_TOP,
            LinearGradientDirection::LeftBottom => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT_BOTTOM,
            LinearGradientDirection::RightTop => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT_TOP,
            LinearGradientDirection::RightBottom => style_property_enum::ArkUI_LinearGradientDirection::ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT_BOTTOM,
          }
        ).into()
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
