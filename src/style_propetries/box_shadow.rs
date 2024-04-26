use lightningcss::{properties::Property, values::{length::Length, color::CssColor}, traits::ToCss};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use crate::{generate_expr_by_length, generate_expr_lit_bool, generate_prop_name, generate_string_by_css_color, style_propetries::traits::ToExpr};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct BoxShadow {
  pub id: String,
  pub offset_x: Option<Length>,
  pub offset_y: Option<Length>,
  pub blur_radius: Option<Length>,
  pub color: Option<CssColor>,
  pub inset: Option<bool>
}

impl BoxShadow {
  pub fn new(id: String) -> Self {
    Self {
      id,
      offset_x: None,
      offset_y: None,
      blur_radius: None,
      color: None,
      inset: None
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

  pub fn set_inset(&mut self, inset: bool) {
    self.inset = Some(inset);
  }
}

impl ToExpr for BoxShadow {
    fn to_expr(&self) -> PropertyTuple {
      PropertyTuple::One(
        "boxShadow".to_string(),
        Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("radius"),
                value: Box::new(generate_expr_by_length!(self.offset_x.as_ref().unwrap(), Platform::Harmony)),
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
                value: Box::new(generate_expr_by_length!(self.offset_x.as_ref().unwrap(), Platform::Harmony)),
              }
            ))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("offsetY"),
                value: Box::new(generate_expr_by_length!(self.offset_y.as_ref().unwrap(), Platform::Harmony)),
              }
            ))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(
              KeyValueProp {
                key: generate_prop_name!("fill"),
                value: Box::new(generate_expr_lit_bool!(self.inset.unwrap())),
              }
            ))),
          ]
        })
      )
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      PropertyTuple::Array(
        vec![
          ("BoxShadowOffset".to_string(), Expr::Object(ObjectLit {
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
          ("BoxShadowColor".to_string(), generate_string_by_css_color!(self.color.as_ref().unwrap())),
          ("BoxShadowRadius".to_string(), generate_expr_by_length!(self.blur_radius.as_ref().unwrap(), Platform::ReactNative)),
        ]
      )
    }
}

impl From<(String, &Property<'_>)> for BoxShadow {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let box_shadow = BoxShadow::new(prop.0);
    match prop.1 {
      Property::BoxShadow(value, _) => {
        value.into_iter().fold(box_shadow, |mut acc, val| {
          acc.set_offset_x(val.x_offset.clone());
          acc.set_offset_y(val.y_offset.clone());
          acc.set_blur_radius(val.blur.clone());
          acc.set_color(val.color.clone());
          acc.set_inset(val.inset.clone());
          acc
        })
      }
      _ => box_shadow
    }
  }
}
