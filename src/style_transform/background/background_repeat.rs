use lightningcss::properties::{
  background::{BackgroundRepeat, BackgroundRepeatKeyword},
  Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum ImageRepeatItem {
  XY,
  X,
  Y,
  NoRepeat,
}

impl From<&BackgroundRepeat> for ImageRepeatItem {
  fn from(value: &BackgroundRepeat) -> Self {
    if value.x == BackgroundRepeatKeyword::Repeat && value.y == BackgroundRepeatKeyword::Repeat {
      Self::XY
    } else if value.x == BackgroundRepeatKeyword::Repeat {
      Self::X
    } else if value.y == BackgroundRepeatKeyword::Repeat {
      Self::Y
    } else {
      Self::NoRepeat
    }
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
pub struct ImageRepeat(pub Vec<ImageRepeatItem>);

impl From<&Property<'_>> for ImageRepeat {
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
    ImageRepeat(image_repeat)
  }
}

impl ToExpr for ImageRepeat {
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
