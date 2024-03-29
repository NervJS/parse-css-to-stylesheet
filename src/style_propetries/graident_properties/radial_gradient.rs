
use lightningcss::values::{gradient::{Circle, Ellipse, EndingShape}, position::{HorizontalPositionKeyword, PositionComponent, VerticalPositionKeyword}};
use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_by_length, generate_expr_by_length_percentage, generate_expr_lit_num, generate_expr_lit_str, generate_invalid_expr, style_propetries::unit::Platform};


#[derive(Debug, Clone)]
pub struct RadialGradientPoint {
  pub x: PositionComponent<HorizontalPositionKeyword>,
  pub y: PositionComponent<VerticalPositionKeyword>,
}

#[derive(Debug, Clone)]
pub struct RadialGradientItem {
  pub color_stops: Vec<(Expr, Expr)>,
  pub point: RadialGradientPoint,
  pub shape: EndingShape,
}

impl RadialGradientItem {
  pub fn to_expr(&self) -> Expr {
    let mut props = vec![];
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
                    expr: Box::new(item.0.clone())
                  }),
                  Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(item.1.clone())
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
    
    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("center".into(), DUMMY_SP)),
      value: Box::new(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: vec![
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(match &self.point.x {
              PositionComponent::Center => generate_expr_lit_str!("50%"),
              PositionComponent::Length(length_value) => generate_expr_by_length_percentage!(length_value, Platform::Harmony),
              PositionComponent::Side { side, offset: _ } => {
                match side {
                  HorizontalPositionKeyword::Left => generate_expr_lit_num!(0),
                  HorizontalPositionKeyword::Right => generate_expr_lit_str!("100%")
                }
              },
            }),
          }),
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(match &self.point.y {
              PositionComponent::Center => generate_expr_lit_str!("50%"),
              PositionComponent::Length(length_value) => generate_expr_by_length_percentage!(length_value, Platform::Harmony),
              PositionComponent::Side { side, offset: _ } => {
                match side {
                  VerticalPositionKeyword::Top => generate_expr_lit_num!(0),
                  VerticalPositionKeyword::Bottom => generate_expr_lit_str!("100%")
                }
              },
            }),
          }),
        ]
      }))
    }))));

    match &self.shape {
      EndingShape::Circle(circle) => {
        match circle {
          Circle::Radius(radius) => {
            props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("radius".into(), DUMMY_SP)),
              value: Box::new(generate_expr_by_length!(radius, Platform::Harmony)),
            }))));
          },
          Circle::Extent(_) => {
            // Harmony不支持extent
          },
        }
      },
      EndingShape::Ellipse(ellipse) => {
        // Harmony不支持椭圆形状
        match ellipse {
          // 暂时会把x作为半径
          Ellipse::Size { x, y: _ } => {
            props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("radius".into(), DUMMY_SP)),
              value: Box::new(generate_expr_by_length_percentage!(x, Platform::Harmony)),
            }))));
          },
          Ellipse::Extent(_) => {
            // Harmony不支持extent
          }
        }
      },
    };

    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("repeating".into(), DUMMY_SP)),
      value: Box::new(Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value: false,
      }))),
    }))));

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}

#[derive(Debug, Clone)]
pub struct RadialGradient(pub Vec<RadialGradientItem>);




