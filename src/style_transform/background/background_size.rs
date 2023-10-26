use lightningcss::{
  properties::{background::BackgroundSize, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::length::LengthPercentageOrAuto,
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Expr, Ident, KeyValueProp, Lit, MemberExpr, MemberProp, ObjectLit, Prop, PropName,
  PropOrSpread, Str,
};

use crate::style_transform::traits::ToExpr;

pub fn parse_background_size_item(size_item: &BackgroundSize) -> Option<ImageSize> {
  match size_item {
    BackgroundSize::Contain => Some(ImageSize::Contain),
    BackgroundSize::Cover => Some(ImageSize::Cover),
    BackgroundSize::Explicit { width, height } => match width {
      LengthPercentageOrAuto::Auto => match height {
        LengthPercentageOrAuto::Auto => Some(ImageSize::Auto),
        _ => None,
      },
      LengthPercentageOrAuto::LengthPercentage(x) => match height {
        LengthPercentageOrAuto::LengthPercentage(y) => Some(ImageSize::ImageSizeWH(
          x.to_css_string(PrinterOptions::default()).unwrap(),
          y.to_css_string(PrinterOptions::default()).unwrap(),
        )),
        _ => None,
      },
    },
  }
}

pub fn parse_background_size(size: &SmallVec<[BackgroundSize; 1]>) -> BackgroundImageSize {
  let mut background_size = vec![];
  for item in size {
    let item_size = parse_background_size_item(item);
    if let Some(size) = item_size {
      background_size.push(size);
    }
  }

  BackgroundImageSize(background_size)
}

#[derive(Debug, Clone)]
pub enum ImageSize {
  Cover,
  Contain,
  Auto,
  ImageSizeWH(String, String),
}

#[derive(Debug, Clone)]
pub struct BackgroundImageSize(pub Vec<ImageSize>);

impl ToExpr for BackgroundImageSize {
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
            ImageSize::ImageSizeWH(width, height) => Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("width".into(), DUMMY_SP)),
                  value: Expr::Lit(Lit::Str(Str::from(width.to_string()))).into(),
                }))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("height".into(), DUMMY_SP)),
                  value: Expr::Lit(Lit::Str(Str::from(height.to_string()))).into(),
                }))),
              ]
              .into(),
            })
            .into(),
          })
        })
        .collect::<Vec<_>>(),
    })
  }
}

impl From<&Property<'_>> for BackgroundImageSize {
  fn from(value: &Property<'_>) -> Self {
    let mut background_image_size = BackgroundImageSize(vec![]);
    match value {
      Property::BackgroundSize(value) => {
        background_image_size = parse_background_size(&value);
      }
      _ => {}
    }
    background_image_size
  }
}
