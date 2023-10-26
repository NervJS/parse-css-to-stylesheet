use lightningcss::{
  properties::{background::Background as LNBackground, Property},
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
  values::color::CssColor,
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

use crate::style_transform::traits::ToExpr;

use super::{
  background_color::BackgroundColor,
  background_image::{parse_background_image_item, BackgroundImage},
  background_position::{parse_background_position_item, BackgroundImagePosition},
  background_size::{parse_background_size_item, BackgroundImageSize},
};

fn parse_background(background: &SmallVec<[LNBackground<'_>; 1]>) -> Background {
  let mut background_image = vec![];
  let mut background_position = vec![];
  let mut background_size = vec![];
  let mut background_color = None;
  for item in background.iter() {
    if let Some(image) = parse_background_image_item(&item.image, &item.repeat) {
      background_image.push(image);
    }
    background_position.push(parse_background_position_item(&item.position));
    if let Some(size) = parse_background_size_item(&item.size) {
      background_size.push(size);
    }
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
  Background {
    image: BackgroundImage(background_image),
    position: BackgroundImagePosition(background_position),
    size: BackgroundImageSize(background_size),
    color: BackgroundColor(background_color.unwrap_or("".to_string())),
  }
}

#[derive(Debug, Clone)]
pub struct Background {
  pub image: BackgroundImage,
  pub color: BackgroundColor,
  pub size: BackgroundImageSize,
  pub position: BackgroundImagePosition,
}

impl Background {
  pub fn new() -> Self {
    Background {
      image: BackgroundImage(vec![]),
      color: BackgroundColor("".to_string()),
      size: BackgroundImageSize(vec![]),
      position: BackgroundImagePosition(vec![]),
    }
  }
}

impl ToExpr for Background {
  fn to_expr(&self) -> Expr {
    let mut arr = vec![];
    if self.image.0.len() > 0 {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("image".into(), DUMMY_SP)),
        value: self.image.to_expr().into(),
      }))))
    }
    if self.color.to_string() != "" {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("color".into(), DUMMY_SP)),
        value: self.color.to_expr().into(),
      }))))
    }
    if self.size.0.len() > 0 {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("size".into(), DUMMY_SP)),
        value: self.size.to_expr().into(),
      }))))
    }
    if self.position.0.len() > 0 {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("position".into(), DUMMY_SP)),
        value: self.position.to_expr().into(),
      }))))
    }
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })
  }
}

impl From<&Property<'_>> for Background {
  fn from(value: &Property<'_>) -> Self {
    let mut background = Background {
      image: BackgroundImage(vec![]),
      color: BackgroundColor("".to_string()),
      size: BackgroundImageSize(vec![]),
      position: BackgroundImagePosition(vec![]),
    };
    if let Property::Background(value) = value {
      background = parse_background(&value);
    }
    background
  }
}
