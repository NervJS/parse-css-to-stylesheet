use lightningcss::properties::{
  background::{BackgroundRepeat as LNBackgroundRepeat, BackgroundRepeatKeyword},
  Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

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

impl ToExpr for ImageRepeatItem {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("ImageRepeat".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          Self::XY => "XY".into(),
          Self::X => "X".into(),
          Self::Y => "Y".into(),
          Self::NoRepeat => "NoRepeat".into(),
        },
        optional: false,
      }),
    })
  }
}

#[derive(Debug, Clone)]
pub struct BackgroundRepeat(pub Vec<ImageRepeatItem>);

impl From<&Property<'_>> for BackgroundRepeat {
  fn from(value: &Property<'_>) -> Self {
    let mut image_repeat = vec![];
    match value {
      Property::BackgroundRepeat(value) => {
        value.into_iter().for_each(|item| {
          image_repeat.push(ImageRepeatItem::from(item));
        });
      }
      _ => {}
    }
    BackgroundRepeat(image_repeat)
  }
}

impl ToExpr for BackgroundRepeat {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .map(|item| Some(item.to_expr().into()))
        .collect::<Vec<_>>(),
    })
  }
}
