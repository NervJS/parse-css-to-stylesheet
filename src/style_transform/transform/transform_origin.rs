use lightningcss::{properties::Property, values::position::{PositionComponent::{self, Center, Side}, HorizontalPositionKeyword, VerticalPositionKeyword}};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, ObjectLit, Prop, KeyValueProp, PropName, Ident};

use crate::style_transform::{traits::ToExpr, utils::StringNumber};

use super::transform::parse_dimension_percentage;

#[derive(Debug, Clone)]
pub struct TransformOrigin {
  pub x: StringNumber,
  pub y: StringNumber,
}

impl TransformOrigin {
  pub fn new() -> Self {
    TransformOrigin {
      x: StringNumber::Number(0.0),
      y: StringNumber::Number(0.0),
    }
  }
}

impl ToExpr for TransformOrigin {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(stringify!("x").into(), DUMMY_SP)),
          value: Box::new(self.x.to_expr()),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(stringify!("y").into(), DUMMY_SP)),
          value: Box::new(self.y.to_expr()),
        }))),
      ],
    })
  }
}

impl From<&Property<'_>> for TransformOrigin {
  fn from(value: &Property<'_>) -> Self {
    let mut transform_origin = TransformOrigin::new();
    if let Property::TransformOrigin(position, _) = value {

      match &position.x {
        Center => {
          transform_origin.x = StringNumber::String("50%".to_string());
        }
        PositionComponent::Length(length) => {
          if let Some(x) = parse_dimension_percentage(&length) {
            transform_origin.x = x;
          }
        }
        Side { side, .. } => match &side {
          HorizontalPositionKeyword::Left => {
            transform_origin.x = StringNumber::String("0%".to_string());
          }
          HorizontalPositionKeyword::Right => {
            transform_origin.x = StringNumber::String("100%".to_string());
          }
        },
      }
      match &position.y {
        Center => {
          transform_origin.y = StringNumber::String("50%".to_string());
        }
        PositionComponent::Length(length) => {
          if let Some(y) = parse_dimension_percentage(&length) {
            transform_origin.y = y;
          }
        }
        Side { side, .. } => match &side {
          VerticalPositionKeyword::Top => {
            transform_origin.y = StringNumber::String("0%".to_string());
          }
          VerticalPositionKeyword::Bottom => {
            transform_origin.y = StringNumber::String("100%".to_string());
          }
        },
      }
    }
    transform_origin
  }
}
