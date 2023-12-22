use lightningcss::{
  properties::{background::Background as LNBackground, Property},
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
  values::color::CssColor,
};
use smallvec::SmallVec;

use super::{
  background_image::{parse_background_image_item, BackgroundImage},
  background_position::{parse_background_position_item, BackgroundPosition},
  background_size::{parse_background_size_item, BackgroundSize}, background_color::BackgroundColor, background_repeat::{parse_background_repeat_item, BackgroundRepeat},
};

fn parse_background(background: &SmallVec<[LNBackground<'_>; 1]>) -> Background {
  let mut background_image = vec![];
  let mut background_position = vec![];
  let mut background_size = vec![];
  let mut background_color = None;
  let mut background_repeat = vec![];

  for item in background.iter() {
    if let Some(image) = parse_background_image_item(&item.image, &item.repeat) {
      background_image.push(image);
    }
    background_position.push(parse_background_position_item(&item.position));
    if let Some(size) = parse_background_size_item(&item.size) {
      background_size.push(size);
    }
    background_repeat.push(parse_background_repeat_item(&item.repeat));
    if item.color != CssColor::default() {
      background_color = Some(
        item
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
      );
    }
  }
  let mut bg = Background::new();
  if background_image.len() > 0 {
    bg.image = Some(BackgroundImage(background_image));
  }
  if background_position.len() > 0 {
    bg.position = Some(BackgroundPosition(background_position));
  }
  if background_size.len() > 0 {
    bg.size = Some(BackgroundSize(background_size));
  }
  if background_repeat.len() > 0 {
    bg.repeat = Some(BackgroundRepeat(background_repeat));
  }
  if let Some(color) = background_color {
    bg.color = Some(BackgroundColor(color));
  }
  bg
}

#[derive(Debug, Clone)]
pub struct Background {
  pub image: Option<BackgroundImage>,
  pub color: Option<BackgroundColor>,
  pub size: Option<BackgroundSize>,
  pub position: Option<BackgroundPosition>,
  pub repeat: Option<BackgroundRepeat>
}

impl Background {
  pub fn new() -> Self {
    Background {
      image: None,
      color: None,
      size: None,
      position: None,
      repeat: None
    }
  }
}

impl From<&Property<'_>> for Background {
  fn from(value: &Property<'_>) -> Self {
    let mut background = Background::new();
    if let Property::Background(value) = value {
      background = parse_background(&value);
    }
    background
  }
}
