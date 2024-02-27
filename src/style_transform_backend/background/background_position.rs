use lightningcss::{
  properties::{background::BackgroundPosition as LNBackgroundPosition, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::position::{
    HorizontalPositionKeyword,
    PositionComponent::{self, Center, Side},
    VerticalPositionKeyword,
  },
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Expr, Ident, KeyValueProp, MemberExpr, MemberProp, ObjectLit, Prop, PropName,
  PropOrSpread, 
};

use crate::{style_transform::traits::ToExpr, utils::convert_px_to_units};

pub fn parse_background_position_item(position: &LNBackgroundPosition) -> ImagePosition {
  match &position.x {
    Center => match &position.y {
      Center => ImagePosition::Center,
      Side { side, .. } => match side {
        VerticalPositionKeyword::Top => ImagePosition::Top,
        VerticalPositionKeyword::Bottom => ImagePosition::Bottom,
      },
      PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
        "50%".to_string(),
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
      ),
    },
    Side { side, .. } => match side {
      HorizontalPositionKeyword::Left => match &position.y {
        Center => ImagePosition::Start,
        Side { side, .. } => match side {
          VerticalPositionKeyword::Top => ImagePosition::TopStart,
          VerticalPositionKeyword::Bottom => ImagePosition::BottomStart,
        },
        PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
          "0".to_string(),
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ),
      },
      HorizontalPositionKeyword::Right => match &position.y {
        Center => ImagePosition::End,
        Side { side, .. } => match side {
          VerticalPositionKeyword::Top => ImagePosition::TopEnd,
          VerticalPositionKeyword::Bottom => ImagePosition::BottomEnd,
        },
        PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
          "100%".to_string(),
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ),
      },
    },
    PositionComponent::Length(length_percentage) => match &position.y {
      Center => ImagePosition::ImagePositionXY(
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
        "50%".to_string(),
      ),
      Side { side, .. } => match side {
        VerticalPositionKeyword::Top => ImagePosition::ImagePositionXY(
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
          "0".to_string(),
        ),
        VerticalPositionKeyword::Bottom => ImagePosition::ImagePositionXY(
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
          "100%".to_string(),
        ),
      },
      PositionComponent::Length(length_percentage) => ImagePosition::ImagePositionXY(
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
        length_percentage
          .to_css_string(PrinterOptions::default())
          .unwrap(),
      ),
    },
  }
}

pub fn parse_background_position(
  position: &SmallVec<[LNBackgroundPosition; 1]>,
) -> BackgroundPosition {
  let mut background_position = vec![];
  for item in position {
    background_position.push(parse_background_position_item(item));
  }
  BackgroundPosition(background_position)
}

#[derive(Debug, Clone)]
pub enum ImagePosition {
  ImagePositionXY(String, String),
  TopStart,
  Top,
  TopEnd,
  Start,
  Center,
  End,
  BottomStart,
  Bottom,
  BottomEnd,
}

#[derive(Debug, Clone)]
pub struct BackgroundPosition(pub Vec<ImagePosition>);

impl ToExpr for BackgroundPosition {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .map(|item| {
          Some(match item {
            ImagePosition::ImagePositionXY(x, y) => Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("x".into(), DUMMY_SP)),
                  value: convert_px_to_units(x.to_string()).into(),
                }))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("y".into(), DUMMY_SP)),
                  value: convert_px_to_units(y.to_string()).into(),
                }))),
              ]
              .into(),
            })
            .into(),
            ImagePosition::TopStart => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "TopStart".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Top => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Top".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::TopEnd => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "TopEnd".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Start => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Start".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Center => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Center".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::End => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "End".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::BottomStart => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "BottomStart".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Bottom => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Bottom".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::BottomEnd => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("Alignment".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "BottomEnd".into(),
                optional: false,
              }),
            })
            .into(),
          })
        })
        .collect::<Vec<_>>(),
    })
  }
}

impl From<&Property<'_>> for BackgroundPosition {
  fn from(value: &Property<'_>) -> Self {
    let mut background_image_position = BackgroundPosition(vec![]);
    match value {
      Property::BackgroundPosition(value) => {
        background_image_position = parse_background_position(&value);
      }
      _ => {}
    }
    background_image_position
  }
}
