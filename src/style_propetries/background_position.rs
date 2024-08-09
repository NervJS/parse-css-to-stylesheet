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
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::{
  generate_expr_enum,
  style_propetries::{style_property_enum, traits::ToExpr},
};

use super::{
  style_property_type::CSSPropertyType,
  unit::{generate_expr_with_css_input, Platform, PropertyTuple},
};

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
  pub value: Vec<ImagePosition>,
}

impl ToExpr for BackgroundPosition {
  fn to_expr(&self) -> PropertyTuple {
    let property_tuple = match self.value.get(0).unwrap() {
      ImagePosition::ImagePositionXY(x, y) => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input(x.to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input(y.to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::TopStart => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::Top => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::TopEnd => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::Start => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::Center => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::End => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::BottomStart => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("0%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::Bottom => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("50%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
      ]),
      ImagePosition::BottomEnd => PropertyTuple::Array(vec![
        (
          CSSPropertyType::BackgroundPositionX,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
        (
          CSSPropertyType::BackgroundPositionY,
          generate_expr_with_css_input("100%".to_string(), Platform::Harmony).into(),
        ),
      ]),
    };

    property_tuple

    // PropertyTuple::One(
    //   CSSPropertyType::BackgroundPosition,
    //   expr
    // )
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
