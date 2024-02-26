use lightningcss::{
  properties::{
    background::{BackgroundRepeat, BackgroundRepeatKeyword},
    Property,
  },
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{
    angle::Angle,
    gradient::{Gradient, GradientItem, LineDirection},
    image::Image,
    length::LengthValue,
    percentage::DimensionPercentage,
    position::{HorizontalPositionKeyword, VerticalPositionKeyword},
  }, targets::{Targets, Features},
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::generate_invalid_expr;

use super::{linear_gradient::{LinearGradientDirection, LinearGradientItem}, traits::ToExpr, unit::PropertyTuple};

pub fn parse_background_image_item(image: &Image) -> Option<BackgroundImageKind> {
  match image {
    Image::Url(url) => Some(
      BackgroundImageKind::String(url.url.to_string())
    ),
    Image::Gradient(gradient) => {
      if let Gradient::Linear(gradient) = &**gradient {
        let mut color_stops = vec![];
        for item in &gradient.items {
          match item {
            GradientItem::ColorStop(color_stop) => {
              let color_stop_position = color_stop
                .position
                .clone()
                .unwrap_or(DimensionPercentage::Dimension(LengthValue::Px(0.0)));
              color_stops.push((
                color_stop
                  .color
                  .to_css_string(PrinterOptions {
                    minify: false,
                    targets: Targets {
                      include: Features::HexAlphaColors,
                      ..Targets::default()
                    },
                    ..PrinterOptions::default()
                  })
                  .unwrap(),
                match &color_stop_position {
                  DimensionPercentage::Dimension(length) => {
                    length.to_css_string(PrinterOptions::default()).unwrap()
                  }
                  DimensionPercentage::Percentage(percentage) => percentage.0.to_string(),
                  _ => color_stop_position
                    .to_css_string(PrinterOptions::default())
                    .unwrap(),
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
      } else {
        None
      }
    }
    _ => None,
  }
}

pub fn parse_background_image(image: &SmallVec<[Image; 1]>) -> Vec<BackgroundImageKind> {
  let mut background_image = vec![];
  for (index, item) in image.iter().enumerate() {
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
}

#[derive(Debug, Clone)]
pub struct BackgroundImage {
  pub id: String,
  pub value: Vec<BackgroundImageKind>
}

impl ToExpr for BackgroundImage {
  fn to_expr(&self) -> PropertyTuple {
    let expr = match self.value.get(0).unwrap() {
      BackgroundImageKind::String(src) => {
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
      BackgroundImageKind::LinearGradient(linear_gradient) => {
        linear_gradient.to_expr()
      }
    };
    PropertyTuple::One(
      "backgroundImage".to_string(),
      expr
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      "backgroundImage".to_string(),
      generate_invalid_expr!()
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