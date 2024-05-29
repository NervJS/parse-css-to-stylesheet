use lightningcss::properties::{
  background::{BackgroundRepeat as LNBackgroundRepeat, BackgroundRepeatKeyword},
  Property,
};

use swc_core::ecma::ast::*;

use crate::{generate_expr_enum, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};

pub fn parse_background_repeat_item(value: &LNBackgroundRepeat) -> ImageRepeatItem {
  if value.x == BackgroundRepeatKeyword::Repeat && value.y == BackgroundRepeatKeyword::Repeat {
    ImageRepeatItem::XY
  } else if value.x == BackgroundRepeatKeyword::Repeat {
    ImageRepeatItem::X
  } else if value.y == BackgroundRepeatKeyword::Repeat {
    ImageRepeatItem::Y
  } else {
    ImageRepeatItem::NoRepeat
  }
}

#[derive(Debug, Clone)]
pub enum ImageRepeatItem {
  XY,
  X,
  Y,
  NoRepeat,
}

impl From<&LNBackgroundRepeat> for ImageRepeatItem {
  fn from(value: &LNBackgroundRepeat) -> Self {
    parse_background_repeat_item(value)
  }
}

impl ImageRepeatItem {
  fn to_expr(&self) -> Expr {
    match &self {
      Self::XY => generate_expr_enum!(style_property_enum::ArkUI_ImageRepeat::ARKUI_IMAGE_REPEAT_XY),
      Self::X => generate_expr_enum!(style_property_enum::ArkUI_ImageRepeat::ARKUI_IMAGE_REPEAT_X),
      Self::Y => generate_expr_enum!(style_property_enum::ArkUI_ImageRepeat::ARKUI_IMAGE_REPEAT_Y),
      Self::NoRepeat => generate_expr_enum!(style_property_enum::ArkUI_ImageRepeat::ARKUI_IMAGE_REPEAT_NONE),
    }
  }
}

#[derive(Debug, Clone)]
pub struct BackgroundRepeat {
  pub id: String,
  pub value: Vec<ImageRepeatItem>
}

impl From<(String, &Property<'_>)> for BackgroundRepeat {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut image_repeat = vec![];
    match value.1 {
      Property::BackgroundRepeat(value) => {
        value.into_iter().for_each(|item| {
          image_repeat.push(ImageRepeatItem::from(item));
        });
      }
      _ => {}
    };
    BackgroundRepeat {
      id: value.0.to_string(),
      value: image_repeat,
    }
  }
}

impl ToExpr for BackgroundRepeat {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::BackgroundRepeat,
      self.value.get(0).unwrap().to_expr(),
    )
  }
}
