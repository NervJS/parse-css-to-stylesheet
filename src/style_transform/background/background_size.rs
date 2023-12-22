use lightningcss::{
  properties::{background::BackgroundSize as LNBackgroundSize, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::length::LengthPercentageOrAuto,
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Expr, Ident, KeyValueProp, MemberExpr, MemberProp, ObjectLit, Prop, PropName,
  PropOrSpread,
};

use crate::{style_transform::traits::ToExpr, utils::convert_px_to_units};

pub fn parse_background_size_item(size_item: &LNBackgroundSize) -> Option<ImageSize> {
  match size_item {
    LNBackgroundSize::Contain => Some(ImageSize::Contain),
    LNBackgroundSize::Cover => Some(ImageSize::Cover),
    LNBackgroundSize::Explicit { width, height } => {
      match width {
        LengthPercentageOrAuto::Auto => match height {
          LengthPercentageOrAuto::Auto => Some(ImageSize::Auto),
          _ => None,
        },
        LengthPercentageOrAuto::LengthPercentage(x) => {
          let x_str = x.to_css_string(PrinterOptions::default()).unwrap();
          match height {
            LengthPercentageOrAuto::LengthPercentage(y) => {
              let y_str = y.to_css_string(PrinterOptions::default()).unwrap();
              Some(ImageSize::ImageSizeWH(x_str, Some(y_str)))
            },
            LengthPercentageOrAuto::Auto => {
                Some(ImageSize::ImageSizeWH(x_str, None))
            }
          }
        },
      }
    }
  }
}

pub fn parse_background_size(size: &SmallVec<[LNBackgroundSize; 1]>) -> BackgroundSize {
  let mut background_size = vec![];
  for item in size {
    let item_size = parse_background_size_item(item);
    if let Some(size) = item_size {
      background_size.push(size);
    }
  }

  BackgroundSize(background_size)
}

#[derive(Debug, Clone)]
pub enum ImageSize {
  Cover,
  Contain,
  Auto,
  ImageSizeWH(String, Option<String>),
}

#[derive(Debug, Clone)]
pub struct BackgroundSize(pub Vec<ImageSize>);

impl ToExpr for BackgroundSize {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .map(|item| {
          Some(match item {
            ImageSize::Cover => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImageSize".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Cover".into(),
                optional: false,
              }),
            })
            .into(),
            ImageSize::Contain => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImageSize".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Contain".into(),
                optional: false,
              }),
            })
            .into(),
            ImageSize::Auto => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImageSize".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Auto".into(),
                optional: false,
              }),
            })
            .into(),
            ImageSize::ImageSizeWH(width, height) => {
              let width_str = width.to_string();
              let height_str = height.as_ref().map(|h| h.to_string());
            
              let mut props = vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("width".into(), DUMMY_SP)),
                  value: convert_px_to_units(width_str).into(),
                })))
              ];
            
              if let Some(height_str) = height_str {
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("height".into(), DUMMY_SP)),
                  value: convert_px_to_units(height_str).into(),
                }))))
              }
            
              Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: props.into()
              }).into()
            }
          })
        })
        .collect::<Vec<_>>(),
    })
  }
}

impl From<&Property<'_>> for BackgroundSize {
  fn from(value: &Property<'_>) -> Self {
    let mut background_image_size = BackgroundSize(vec![]);
    match value {
      Property::BackgroundSize(value) => {
        background_image_size = parse_background_size(&value);
      }
      _ => {}
    }
    background_image_size
  }
}
