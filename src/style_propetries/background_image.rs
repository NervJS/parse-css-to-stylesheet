use lightningcss::{
  properties::Property, values::{
    angle::Angle,
    gradient::{ Gradient, GradientItem, LineDirection},
    image::Image,
    percentage::{DimensionPercentage, Percentage},
    position::{HorizontalPositionKeyword, VerticalPositionKeyword},
  }
};
use smallvec::SmallVec;

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_lit_color, generate_expr_lit_num, generate_invalid_expr};

use super::{graident_properties::{linear_gradient::{LinearGradientDirection, LinearGradientItem}, radial_gradient::{RadialGradientItem, RadialGradientPoint}}, style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

pub fn parse_background_image_item(image: &Image) -> Option<BackgroundImageKind> {
  match image {
    Image::Url(url) => Some(
      BackgroundImageKind::String(url.url.to_string())
    ),
    Image::Gradient(gradient) => {
      match &**gradient {
        Gradient::Linear(gradient) => {
          let mut color_stops = vec![];
          let mut now_percentage = 0.0;
          let colors_len = gradient.items.len() - 1;
          for (index, item) in gradient.items.clone().into_iter().enumerate()  {
            match item {
              GradientItem::ColorStop(color_stop) => {
                let item_pecentage = now_percentage + (((1.0 - now_percentage) / colors_len as f32) * index as f32);
                let color_stop_position = color_stop
                  .position
                  .clone()
                  .unwrap_or(DimensionPercentage::Percentage(Percentage(item_pecentage)));
                color_stops.push((
                  generate_expr_lit_color!(color_stop.color),
                  match &color_stop_position {
                    DimensionPercentage::Dimension(_) => generate_expr_lit_num!(0.0),
                    DimensionPercentage::Percentage(percentage) => {
                      now_percentage = percentage.0;
                      generate_expr_lit_num!(percentage.0 as f64)
                    },
                    DimensionPercentage::Calc(_) => generate_expr_lit_num!(0.0),
                  },
                ));
              }
              _ => {}
            };
          }
          let direction = &gradient.direction;
          match direction {
            LineDirection::Angle(angle) => {
              let angle = match angle {
                Angle::Deg(deg) => Some(*deg),
                Angle::Rad(rad) => Some(rad.to_degrees()),
                Angle::Turn(turn) => Some(turn * 360.0),
                Angle::Grad(grad) => Some(grad * 0.9),
              };
              Some(
                BackgroundImageKind::LinearGradient(LinearGradientItem {
                  angle,
                  color_stops,
                  derection: None,
                })
              )
            }
            LineDirection::Horizontal(horizontal) => Some(BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match horizontal {
                HorizontalPositionKeyword::Left => LinearGradientDirection::Left,
                HorizontalPositionKeyword::Right => LinearGradientDirection::Right,
              })
            })),
            LineDirection::Vertical(vertical) => Some(BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match vertical {
                VerticalPositionKeyword::Top => LinearGradientDirection::Top,
                VerticalPositionKeyword::Bottom => LinearGradientDirection::Bottom,
              }),
            })),
            LineDirection::Corner {
              horizontal,
              vertical,
            } => Some(BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match (horizontal, vertical) {
                (HorizontalPositionKeyword::Left, VerticalPositionKeyword::Top) => {
                  LinearGradientDirection::LeftTop
                }
                (HorizontalPositionKeyword::Left, VerticalPositionKeyword::Bottom) => {
                  LinearGradientDirection::LeftBottom
                }
                (HorizontalPositionKeyword::Right, VerticalPositionKeyword::Top) => {
                  LinearGradientDirection::RightTop
                }
                (HorizontalPositionKeyword::Right, VerticalPositionKeyword::Bottom) => {
                  LinearGradientDirection::RightBottom
                }
              }),
            })),
          }
        },
        Gradient::RepeatingLinear(_) => None,
        Gradient::Radial(gradient) => { 
          // Radial 华为的半径需要具体的单位
          let mut color_stops = vec![];
          let mut now_percentage = 0.0;
          let colors_len = gradient.items.len() - 1;
          for (index, item) in gradient.items.clone().into_iter().enumerate()  {
            match item {
              GradientItem::ColorStop(color_stop) => {
                let item_pecentage = now_percentage + (((1.0 - now_percentage) / colors_len as f32) * index as f32);
                let color_stop_position = color_stop
                  .position
                  .clone()
                  .unwrap_or(DimensionPercentage::Percentage(Percentage(item_pecentage)));
                color_stops.push((
                  generate_expr_lit_color!(color_stop.color),
                  match &color_stop_position {
                    DimensionPercentage::Dimension(_) => generate_expr_lit_num!(0.0),
                    DimensionPercentage::Percentage(percentage) => {
                      now_percentage = percentage.0;
                      generate_expr_lit_num!(percentage.0 as f64)
                    },
                    DimensionPercentage::Calc(_) => generate_expr_lit_num!(0.0),
                  },
                ));
              }
              _ => {}
            };
          }
          
         

          Some(BackgroundImageKind::RadialGradient(RadialGradientItem {
            color_stops,
            point: RadialGradientPoint { x: gradient.position.x.clone(), y: gradient.position.y.clone() },
            shape: gradient.shape.clone(),
          }))
        },
        Gradient::RepeatingRadial(_) => None,
        Gradient::Conic(_) => None,
        Gradient::RepeatingConic(_) => None,
        Gradient::WebKitGradient(_) => None,
      }
    }
    _ => None,
  }
}

pub fn parse_background_image(image: &SmallVec<[Image; 1]>) -> Vec<BackgroundImageKind> {
  let mut background_image = vec![];
  for (_, item) in image.iter().enumerate() {
    if let Some(item) = parse_background_image_item(item) {
      background_image.push(item);
    }
  }
  background_image
}

#[derive(Debug, Clone)]
pub enum BackgroundImageKind {
  String(String),
  LinearGradient(LinearGradientItem),
  RadialGradient(RadialGradientItem),
}

#[derive(Debug, Clone)]
pub struct BackgroundImage {
  pub id: String,
  pub value: Vec<BackgroundImageKind>
}

impl ToExpr for BackgroundImage {
  fn to_expr(&self) -> PropertyTuple {
    let expr = match self.value.get(0) {
      Some(BackgroundImageKind::String(src)) => {
        Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("src".into(), DUMMY_SP)),
              value: Expr::Lit(Lit::Str(Str::from(src.to_string()))).into(),
            })))
          ]
          .into(),
        })
      },
      Some(BackgroundImageKind::LinearGradient(linear_gradient)) => {
        linear_gradient.to_expr()
      },
      Some(BackgroundImageKind::RadialGradient(radial_gradient)) => {
        radial_gradient.to_expr()
      },
      _ => generate_invalid_expr!()
    };
    PropertyTuple::One(
      CSSPropertyType::BackgroundImage,
      expr
    )
  }

}

impl From<(String, &Property<'_>)> for BackgroundImage {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut background_image_res = vec![];
    match value.1 {
      Property::BackgroundImage(value) => {
        background_image_res = parse_background_image(value);
      }
      _ => {}
    }
    BackgroundImage {
      id: value.0,
      value: background_image_res
    }
  }
}