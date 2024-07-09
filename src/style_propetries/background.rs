use lightningcss::{
  properties::{background::Background as LNBackground, Property},
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
  values::color::CssColor,
};
use smallvec::SmallVec;

use crate::{generate_expr_lit_color, generate_expr_lit_str, generate_invalid_expr};

use super::{
  background_image::{parse_background_image_item, BackgroundImage}, background_position::{parse_background_position_item, BackgroundPosition}, background_repeat::{parse_background_repeat_item, BackgroundRepeat}, background_size::{parse_background_size_item, BackgroundSize}, style_property_type::CSSPropertyType, traits::ToExpr, unit::{convert_color_keywords_to_hex, PropertyTuple}
};

fn parse_background(background: &SmallVec<[LNBackground<'_>; 1]>) -> Background {
  let mut background_image = vec![];
  let mut background_position = vec![];
  let mut background_size = vec![];
  let mut background_color = None;
  let mut background_repeat = vec![];

  for item in background.iter() {
    if let Some(image) = parse_background_image_item(&item.image) {
      background_image.push(image);
    }
    background_position.push(parse_background_position_item(&item.position));
    if let Some(size) = parse_background_size_item(&item.size) {
      background_size.push(size);
    }
    background_repeat.push(parse_background_repeat_item(&item.repeat));
    if item.color != CssColor::default() {
      background_color = Some(item.color.clone());
    }
  }
  let mut bg = Background::new();
  if background_image.len() > 0 {
    bg.image = Some(BackgroundImage {
      id: "background_image".to_string(),
      value: background_image,
    });
  }
  if background_position.len() > 0 {
    bg.position = Some(BackgroundPosition {
      id: "background_position".to_string(),
      value: background_position,
    });
  }
  if background_size.len() > 0 {
    bg.size = Some(BackgroundSize {
      id: "background_size".to_string(),
      value: background_size,
    });
  }
  if background_repeat.len() > 0 {
    bg.repeat = Some(BackgroundRepeat {
      id: "background_repeat".to_string(),
      value: background_repeat,
    });
  }
  if background_color.is_some() {
    bg.color = background_color;
  }
  bg
}

#[derive(Debug, Clone)]
pub struct Background {
  pub image: Option<BackgroundImage>,
  pub size: Option<BackgroundSize>,
  pub position: Option<BackgroundPosition>,
  pub repeat: Option<BackgroundRepeat>,
  pub color: Option<CssColor>
}

impl Background {
  pub fn new() -> Self {
    Background {
      image: None,
      size: None,
      position: None,
      repeat: None,
      color: None
    }
  }
}

impl From<(String, &Property<'_>)> for Background {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut background = Background::new();
    if let Property::Background(value) = value.1 {
      background = parse_background(&value);
    }
    background
  }
}


impl ToExpr for Background {

  fn to_expr(&self) -> PropertyTuple {
    let mut props = vec![];
    if let Some(image) = &self.image {
      match image.to_expr() {
        PropertyTuple::One(_, val) => {
          props.push((CSSPropertyType::BackgroundImage, val));

          // 只有存在image的时候才加
          if let Some(size) = &self.size {
            match size.to_expr() {
              PropertyTuple::One(_, val) => props.push((CSSPropertyType::BackgroundSize, val)),
              _ => {}
            }
          }
          if let Some(position) = &self.position {
            match position.to_expr() {
              PropertyTuple::One(_, val) => props.push((CSSPropertyType::BackgroundPosition, val)),
              _ => {}
            }
          }
          if let Some(repeat) = &self.repeat {
            match repeat.to_expr() {
              PropertyTuple::One(_, val) => props.push((CSSPropertyType::BackgroundRepeat, val)),
              _ => {}
            }
          }
        },
        _ => {}
      }
    }
    
    if let Some(color) = &self.color {
      props.push((CSSPropertyType::BackgroundColor, generate_expr_lit_color!(color)));
    }
    PropertyTuple::Array(props)
  }

}