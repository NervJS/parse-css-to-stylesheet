use lightningcss::{properties::Property, values::{length::LengthPercentage, position::{HorizontalPositionKeyword, PositionComponent::{self, Center, Side}, VerticalPositionKeyword}}};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, ObjectLit, Prop, KeyValueProp, PropName, Ident};

use crate::{generate_expr_by_length_percentage, generate_expr_lit_str, generate_tpl_expr, style_propetries::traits::ToExpr};

use super::unit::{Platform, PropertyTuple};


#[derive(Debug, Clone)]
pub struct TransformOrigin {
  pub id: String,
  pub x: EnumValue,
  pub y: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  String(String),
  Length(LengthPercentage)
}

impl TransformOrigin {
  pub fn new(id: String) -> Self {
    TransformOrigin {
      id: id,
      x: EnumValue::String("0%".to_string()),
      y: EnumValue::String("0%".to_string()),
    }
  }
}

impl ToExpr for TransformOrigin {
  fn to_expr(&self) -> PropertyTuple {
    let expr = Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(stringify!("x").into(), DUMMY_SP)),
          value: Box::new(
            match &self.x {
              EnumValue::String(value) => generate_expr_lit_str!(value.to_string()),
              EnumValue::Length(value) => generate_expr_by_length_percentage!(value, Platform::Harmony)
            }
          ),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(stringify!("y").into(), DUMMY_SP)),
          value: Box::new(
            match &self.y {
              EnumValue::String(value) => generate_expr_lit_str!(value.to_string()),
              EnumValue::Length(value) => generate_expr_by_length_percentage!(value, Platform::Harmony)
            }
          ),
        }))),
      ],
    });

    PropertyTuple::One(
      "transformOrigin".to_string(),
      expr
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    let expr = generate_tpl_expr!(vec![
      match &self.x {
        EnumValue::String(value) => generate_expr_lit_str!(value.to_string()),
        EnumValue::Length(value) => generate_expr_by_length_percentage!(value, Platform::ReactNative)
      },
      match &self.y {
        EnumValue::String(value) => generate_expr_lit_str!(value.to_string()),
        EnumValue::Length(value) => generate_expr_by_length_percentage!(value, Platform::ReactNative)
      }
    ]);
    PropertyTuple::One(
      "transformOrigin".to_string(),
      expr
    )
  }
}

impl From<(String, &Property<'_>)> for TransformOrigin {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut transform_origin = TransformOrigin::new(prop.0);
    if let Property::TransformOrigin(position, _) = prop.1 {

      match &position.x {
        Center => {
          transform_origin.x = EnumValue::String("50%".to_string());
        }
        PositionComponent::Length(length) => {
          transform_origin.x = EnumValue::Length(length.to_owned())
        }
        Side { side, .. } => match &side {
          HorizontalPositionKeyword::Left => {
            transform_origin.x = EnumValue::String("0%".to_string());
          }
          HorizontalPositionKeyword::Right => {
            transform_origin.x = EnumValue::String("100%".to_string());
          }
        },
      }
      match &position.y {
        Center => {
          transform_origin.y = EnumValue::String("50%".to_string());
        }
        PositionComponent::Length(length) => {
          transform_origin.y = EnumValue::Length(length.to_owned())
        }
        Side { side, .. } => match &side {
          VerticalPositionKeyword::Top => {
            transform_origin.y = EnumValue::String("0%".to_string());
          }
          VerticalPositionKeyword::Bottom => {
            transform_origin.y = EnumValue::String("100%".to_string());
          }
        },
      }
    }
    transform_origin
  }
}
