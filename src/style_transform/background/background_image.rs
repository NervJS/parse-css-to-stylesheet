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
  },
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Expr, Ident, KeyValueProp, Lit, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::style_transform::{traits::ToExpr, utils::StringNumber};

use super::{
  background_repeat::ImageRepeatItem,
  linear_gradient::{LinearGradientDirection, LinearGradientItem},
};

pub fn parse_background_image_item(
  image: &Image,
  repeat: &BackgroundRepeat,
) -> Option<BackgroundImageItem> {
  match image {
    Image::Url(url) => Some(BackgroundImageItem {
      image: BackgroundImageKind::String(url.url.to_string()),
      repeat: Some(ImageRepeatItem::from(repeat)),
    }),
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
                  .to_css_string(PrinterOptions::default())
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
        let repeating = if repeat.x == BackgroundRepeatKeyword::Repeat
          && repeat.y == BackgroundRepeatKeyword::Repeat
        {
          true
        } else {
          false
        };
        let direction = &gradient.direction;
        match direction {
          LineDirection::Angle(angle) => {
            let angle = match angle {
              Angle::Deg(deg) => Some(StringNumber::Number(*deg)),
              Angle::Rad(rad) => Some(StringNumber::Number(rad.to_degrees())),
              Angle::Turn(turn) => Some(StringNumber::Number(turn * 360.0)),
              Angle::Grad(grad) => Some(StringNumber::Number(grad * 0.9)),
            };
            Some(BackgroundImageItem {
              image: BackgroundImageKind::LinearGradient(LinearGradientItem {
                angle,
                color_stops,
                derection: None,
                repeating,
              }),
              repeat: None,
            })
          }
          LineDirection::Horizontal(horizontal) => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match horizontal {
                HorizontalPositionKeyword::Left => LinearGradientDirection::Left,
                HorizontalPositionKeyword::Right => LinearGradientDirection::Right,
              }),
              repeating,
            }),
            repeat: None,
          }),
          LineDirection::Vertical(vertical) => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match vertical {
                VerticalPositionKeyword::Top => LinearGradientDirection::Top,
                VerticalPositionKeyword::Bottom => LinearGradientDirection::Bottom,
              }),
              repeating,
            }),
            repeat: None,
          }),
          LineDirection::Corner {
            horizontal,
            vertical,
          } => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
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
              repeating,
            }),
            repeat: None,
          }),
        }
      } else {
        None
      }
    }
    _ => None,
  }
}

pub fn parse_background_image(
  image: &SmallVec<[Image; 1]>,
  repeat: Option<&SmallVec<[BackgroundRepeat; 1]>>,
) -> BackgroundImage {
  let mut background_image = vec![];
  for (index, item) in image.iter().enumerate() {
    if let Some(item) = parse_background_image_item(
      item,
      &repeat
        .map(|item| item[index].clone())
        .unwrap_or(BackgroundRepeat {
          x: BackgroundRepeatKeyword::NoRepeat,
          y: BackgroundRepeatKeyword::NoRepeat,
        }),
    ) {
      background_image.push(item);
    }
  }
  BackgroundImage(background_image)
}

#[derive(Debug, Clone)]
pub enum BackgroundImageKind {
  String(String),
  LinearGradient(LinearGradientItem),
}

#[derive(Debug, Clone)]
pub struct BackgroundImageItem {
  pub image: BackgroundImageKind,
  pub repeat: Option<ImageRepeatItem>,
}

#[derive(Debug, Clone)]
pub struct BackgroundImage(pub Vec<BackgroundImageItem>);

impl ToExpr for BackgroundImage {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .enumerate()
        .map(|(index, item)| match &item.image {
          BackgroundImageKind::String(src) => Some(
            Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("src".into(), DUMMY_SP)),
                  value: Expr::Lit(Lit::Str(Str::from(src.to_string()))).into(),
                }))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("repeat".into(), DUMMY_SP)),
                  value: if let Some(repeat) = &self.0[index].repeat {
                    repeat.to_expr().into()
                  } else {
                    Expr::Lit(Lit::Str(Str::from("NoRepeat"))).into()
                  },
                }))),
              ]
              .into(),
            })
            .into(),
          ),
          BackgroundImageKind::LinearGradient(linear_gradient) => {
            Some(linear_gradient.to_expr().into())
          }
        })
        .collect::<Vec<_>>(),
    })
  }
}

impl From<(&Property<'_>, Option<&Property<'_>>)> for BackgroundImage {
  fn from(value: (&Property<'_>, Option<&Property<'_>>)) -> Self {
    let (background_image, background_image_repeat) = value;
    let mut background_image_res = BackgroundImage(vec![]);
    match background_image {
      Property::BackgroundImage(value) => {
        background_image_res = parse_background_image(
          value,
          background_image_repeat
            .map(|item| match item {
              Property::BackgroundRepeat(value) => Some(value),
              _ => None,
            })
            .or_else(|| None)
            .unwrap(),
        );
      }
      _ => {}
    }
    background_image_res
  }
}
