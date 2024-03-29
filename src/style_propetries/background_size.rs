use lightningcss::{
  properties::{background::BackgroundSize as LNBackgroundSize, Property},
  values::length::LengthPercentageOrAuto,
};
use smallvec::SmallVec;

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::{generate_expr_by_length_percentage_or_auto, generate_invalid_expr};

use super::{traits::ToExpr, unit::{Platform, PropertyTuple}};

pub fn parse_background_size_item(size_item: &LNBackgroundSize) -> Option<ImageSize> {
  match size_item {
    LNBackgroundSize::Contain => Some(ImageSize::Contain),
    LNBackgroundSize::Cover => Some(ImageSize::Cover),
    LNBackgroundSize::Explicit { width, height } => {
      Some(ImageSize::ImageSizeWH(
        width.clone(),
        height.clone()
      ))
      // match width {
      //   LengthPercentageOrAuto::Auto => match height {
      //     LengthPercentageOrAuto::Auto => Some(ImageSize::Auto),
      //     _ => None,
      //   },
      //   LengthPercentageOrAuto::LengthPercentage(x) => {
      //     let x_str = x.to_css_string(PrinterOptions::default()).unwrap();
      //     match height {
      //       LengthPercentageOrAuto::LengthPercentage(y) => {
      //         let y_str = y.to_css_string(PrinterOptions::default()).unwrap();
      //         Some(ImageSize::ImageSizeWH(x_str, Some(y_str)))
      //       },
      //       LengthPercentageOrAuto::Auto => {
      //           Some(ImageSize::ImageSizeWH(x_str, None))
      //       }
      //     }
      //   },
      // }
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
  ImageSizeWH(LengthPercentageOrAuto, LengthPercentageOrAuto),
}

#[derive(Debug, Clone)]
pub struct BackgroundSize {
  pub id: String,
  pub value: Vec<ImageSize>
}

impl ToExpr for BackgroundSize {
  fn to_expr(&self) -> PropertyTuple {
   let expr = match self.value.get(0) {
      Some(image_size) => {
        match image_size {
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
          ImageSize::ImageSizeWH(width, height) => {
            
            let props = vec![
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new("width".into(), DUMMY_SP)),
                value: Box::new(generate_expr_by_length_percentage_or_auto!(width, Platform::Harmony))
              }))),
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new("height".into(), DUMMY_SP)),
                value: Box::new(generate_expr_by_length_percentage_or_auto!(height, Platform::Harmony))
              }))),
            ];
          
            Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: props.into()
            }).into()
          }
        }
      },
      _ => {
        generate_invalid_expr!()
      }
    };
    PropertyTuple::One(
      "backgroundSize".to_string(),
      expr
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      "backgroundSize".to_string(),
      generate_invalid_expr!()
    )
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
      value: background_image_size
    }
  }
}
