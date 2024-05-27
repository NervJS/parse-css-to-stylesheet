use lightningcss::{
  properties::{background::BackgroundPosition as LNBackgroundPosition, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::position::{
    HorizontalPositionKeyword,
    PositionComponent::{self, Center, Side},
    VerticalPositionKeyword,
  },
};
use smallvec::SmallVec;
use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_enum, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::{generate_expr_with_css_input, Platform, PropertyTuple}};

pub fn parse_background_position_item(position: &LNBackgroundPosition) -> ImagePosition {
  match &position.x {
    Center => match &position.y {
      Center => ImagePosition::Center,
      Side { side, .. } => match side {
        VerticalPositionKeyword::Top => ImagePosition::Top,
        VerticalPositionKeyword::Bottom => ImagePosition::Bottom,
      },
      PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
        "50%".to_string(),
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
      ),
    },
    Side { side, .. } => match side {
      HorizontalPositionKeyword::Left => match &position.y {
        Center => ImagePosition::Start,
        Side { side, .. } => match side {
          VerticalPositionKeyword::Top => ImagePosition::TopStart,
          VerticalPositionKeyword::Bottom => ImagePosition::BottomStart,
        },
        PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
          "0".to_string(),
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ),
      },
      HorizontalPositionKeyword::Right => match &position.y {
        Center => ImagePosition::End,
        Side { side, .. } => match side {
          VerticalPositionKeyword::Top => ImagePosition::TopEnd,
          VerticalPositionKeyword::Bottom => ImagePosition::BottomEnd,
        },
        PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
          "100%".to_string(),
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ),
      },
    },
    PositionComponent::Length(length_percentage) => match &position.y {
      Center => ImagePosition::ImagePositionXY(
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
        "50%".to_string(),
      ),
      Side { side, .. } => match side {
        VerticalPositionKeyword::Top => ImagePosition::ImagePositionXY(
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
          "0".to_string(),
        ),
        VerticalPositionKeyword::Bottom => ImagePosition::ImagePositionXY(
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
          "100%".to_string(),
        ),
      },
      PositionComponent::Length(length_percentage_y) => ImagePosition::ImagePositionXY(
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
          length_percentage_y
          .to_css_string(PrinterOptions::default())
          .unwrap(),
      ),
    },
  }
}

pub fn parse_background_position(
  position: &SmallVec<[LNBackgroundPosition; 1]>,
) -> Vec<ImagePosition> {
  let mut background_position = vec![];
  for item in position {
    background_position.push(parse_background_position_item(item));
  }
  background_position
}

#[derive(Debug, Clone)]
pub enum ImagePosition {
  ImagePositionXY(String, String),
  TopStart,
  Top,
  TopEnd,
  Start,
  Center,
  End,
  BottomStart,
  Bottom,
  BottomEnd,
}

#[derive(Debug, Clone)]
pub struct BackgroundPosition {
  pub id: String,
  pub value: Vec<ImagePosition>
}

impl ToExpr for BackgroundPosition {
  fn to_expr(&self) -> PropertyTuple {
    let expr = match self.value.get(0).unwrap() {
      ImagePosition::ImagePositionXY(x, y) => Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("x".into(), DUMMY_SP)),
            value: generate_expr_with_css_input(x.to_string(), Platform::Harmony).into(),
          }))),
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("y".into(), DUMMY_SP)),
            value: generate_expr_with_css_input(y.to_string(), Platform::Harmony).into(),
          }))),
        ]
        .into(),
      })
      .into(),
      ImagePosition::TopStart => generate_expr_enum!(style_property_enum::Alignment::TopStart),
      ImagePosition::Top => generate_expr_enum!(style_property_enum::Alignment::Top),
      ImagePosition::TopEnd => generate_expr_enum!(style_property_enum::Alignment::TopEnd),
      ImagePosition::Start => generate_expr_enum!(style_property_enum::Alignment::Start),
      ImagePosition::Center => generate_expr_enum!(style_property_enum::Alignment::Center),
      ImagePosition::End => generate_expr_enum!(style_property_enum::Alignment::End),
      ImagePosition::BottomStart => generate_expr_enum!(style_property_enum::Alignment::BottomStart),
      ImagePosition::Bottom => generate_expr_enum!(style_property_enum::Alignment::Bottom),
      ImagePosition::BottomEnd => generate_expr_enum!(style_property_enum::Alignment::BottomEnd),
    };
    
    PropertyTuple::One(
      CSSPropertyType::BackgroundPosition,
      expr
    )
  }

}

impl From<(String, &Property<'_>)> for BackgroundPosition {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut background_image_position = vec![];
    match value.1 {
      Property::BackgroundPosition(value) => {
        background_image_position = parse_background_position(&value);
      }
      _ => {}
    }
    BackgroundPosition {
      id: value.0.to_string(),
      value: background_image_position,
    }
    
  }
}
