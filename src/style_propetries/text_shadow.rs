use lightningcss::{properties::Property, values::{length::Length, color::CssColor}, traits::ToCss};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use crate::{style_propetries::traits::ToExpr, generate_prop_name, generate_expr_by_length, generate_string_by_css_color};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct TextShadow {
  pub id: String,
  pub offset_x: Option<Length>,
  pub offset_y: Option<Length>,
  pub blur_radius: Option<Length>,
  pub color: Option<CssColor>
}

impl TextShadow {
  pub fn new(id: String) -> Self {
    Self {
      id,
      offset_x: None,
      offset_y: None,
      blur_radius: None,
      color: None
    }
  }

  pub fn set_offset_x(&mut self, offset_x: Length) {
    self.offset_x = Some(offset_x);
  }

  pub fn set_offset_y(&mut self, offset_y: Length) {
    self.offset_y = Some(offset_y);
  }

  pub fn set_blur_radius(&mut self, blur_radius: Length) {
    self.blur_radius = Some(blur_radius);
  }

  pub fn set_color(&mut self, color: CssColor) {
    self.color = Some(color);
  }
}

impl ToExpr for TextShadow {
    fn to_expr(&self) -> PropertyTuple {
      PropertyTuple::One(
        "textShadow".to_string(),
        Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("radius"),
                value: Box::new(generate_expr_by_length!(self.offset_x.as_ref().unwrap(), Platform::ReactNative)),
              }
            ))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("color"),
                value: Box::new(generate_string_by_css_color!(self.color.as_ref().unwrap())),
              }
            ))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("offsetX"),
                value: Box::new(generate_expr_by_length!(self.offset_x.as_ref().unwrap(), Platform::ReactNative)),
              }
            ))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("offsetY"),
                value: Box::new(generate_expr_by_length!(self.offset_y.as_ref().unwrap(), Platform::ReactNative)),
              }
            ))),
          ]
        })
      )
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      PropertyTuple::Array(
        vec![
          ("textShadowOffset".to_string(), Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
              PropOrSpread::Prop(Box::new(Prop::KeyValue(
                KeyValueProp {
                  key: generate_prop_name!("width"),
                  value: Box::new(generate_expr_by_length!(self.offset_x.as_ref().unwrap(), Platform::ReactNative)),
                }
              ))),
              PropOrSpread::Prop(Box::new(Prop::KeyValue(
                KeyValueProp {
                  key: generate_prop_name!("height"),
                  value: Box::new(generate_expr_by_length!(self.offset_y.as_ref().unwrap(), Platform::ReactNative)),
                }
              ))),
            ],
          })),
          ("textShadowColor".to_string(), generate_string_by_css_color!(self.color.as_ref().unwrap())),
          ("textShadowRadius".to_string(), generate_expr_by_length!(self.blur_radius.as_ref().unwrap(), Platform::ReactNative)),
        ]
      )
    }
}

impl From<(String, &Property<'_>)> for TextShadow {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let text_shadow = TextShadow::new(prop.0);
    match prop.1 {
      Property::TextShadow(value) => {
        value.into_iter().fold(text_shadow, |mut acc, val| {
          acc.set_offset_x(val.x_offset.clone());
          acc.set_offset_y(val.y_offset.clone());
          acc.set_blur_radius(val.blur.clone());
          acc.set_color(val.color.clone());
          acc
        })
      }
      _ => text_shadow
    }
  }
}
