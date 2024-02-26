use lightningcss::properties::{
  background::{BackgroundRepeat as LNBackgroundRepeat, BackgroundRepeatKeyword},
  Property,
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr, Ident, MemberExpr, MemberProp};

use crate::{generate_invalid_expr, style_propetries::traits::ToExpr};

use super::unit::PropertyTuple;

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
      "backgroundRepeat".to_string(),
      self.value.get(0).unwrap().to_expr(),
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    // RN暂不支持该属性
    PropertyTuple::One(
      "backgroundRepeat".to_string(),
      generate_invalid_expr!(),
    )
  }
}
