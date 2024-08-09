use lightningcss::{
  properties::{background::BackgroundSize as LNBackgroundSize, Property},
  values::length::LengthPercentageOrAuto,
};
use smallvec::SmallVec;

use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::{
  generate_expr_by_length_percentage_or_auto, generate_expr_enum, generate_invalid_expr,
  style_propetries::style_property_enum,
};

use super::{
  style_property_type::CSSPropertyType,
  traits::ToExpr,
  unit::{Platform, PropertyTuple},
};

pub fn parse_background_size_item(size_item: &LNBackgroundSize) -> Option<ImageSize> {
  match size_item {
    LNBackgroundSize::Contain => Some(ImageSize::Contain),
    LNBackgroundSize::Cover => Some(ImageSize::Cover),
    LNBackgroundSize::Explicit { width, height } => {
      Some(ImageSize::ImageSizeWH(width.clone(), height.clone()))
    }
  }
}

pub fn parse_background_size(size: &SmallVec<[LNBackgroundSize; 1]>) -> Vec<ImageSize> {
  let mut background_size = vec![];
  for item in size {
    let item_size = parse_background_size_item(item);
    if let Some(size) = item_size {
      background_size.push(size);
    }
  }

  background_size
}

#[derive(Debug, Clone)]
pub enum ImageSize {
  Cover,
  Contain,
  Auto,
  ImageSizeWH(LengthPercentageOrAuto, LengthPercentageOrAuto),
}

#[derive(Debug, Clone)]
pub struct BackgroundSize {
  pub id: String,
  pub value: Vec<ImageSize>,
}

impl ToExpr for BackgroundSize {
  fn to_expr(&self) -> PropertyTuple {
    let expr = match self.value.get(0) {
      Some(image_size) => match image_size {
        ImageSize::Cover => {
          generate_expr_enum!(style_property_enum::ArkUI_ImageSize::ARKUI_IMAGE_SIZE_COVER)
        }
        ImageSize::Contain => {
          generate_expr_enum!(style_property_enum::ArkUI_ImageSize::ARKUI_IMAGE_SIZE_CONTAIN)
        }
        ImageSize::Auto => {
          generate_expr_enum!(style_property_enum::ArkUI_ImageSize::ARKUI_IMAGE_SIZE_AUTO)
        }
        ImageSize::ImageSizeWH(width, height) => {
          let props = vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("width".into(), DUMMY_SP)),
              value: Box::new(generate_expr_by_length_percentage_or_auto!(
                width,
                Platform::Harmony
              )),
            }))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("height".into(), DUMMY_SP)),
              value: Box::new(generate_expr_by_length_percentage_or_auto!(
                height,
                Platform::Harmony
              )),
            }))),
          ];

          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: props.into(),
          })
          .into()
        }
      },
      _ => {
        generate_invalid_expr!()
      }
    };
    PropertyTuple::One(CSSPropertyType::BackgroundSize, expr)
  }
}

impl From<(String, &Property<'_>)> for BackgroundSize {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut background_image_size = vec![];
    match value.1 {
      Property::BackgroundSize(value) => {
        background_image_size = parse_background_size(&value);
      }
      _ => {}
    }
    BackgroundSize {
      id: value.0,
      value: background_image_size,
    }
  }
}
