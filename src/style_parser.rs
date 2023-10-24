use std::{
  cell::RefCell, collections::HashMap, convert::Infallible, fmt::Display, hash::Hash, rc::Rc, vec,
};

use lightningcss::{
  declaration::DeclarationBlock,
  properties::{
    align::{
      AlignContent as LNAlignContent, AlignItems as LNAlignItems, AlignSelf as LNAlignSelf,
      BaselinePosition, ContentDistribution, ContentPosition, JustifyContent as LNJustifyContent,
      SelfPosition,
    },
    background::{
      Background as LNBackground, BackgroundPosition, BackgroundRepeat, BackgroundRepeatKeyword,
      BackgroundSize,
    },
    flex::{Flex, FlexDirection as LNFlexDirection, FlexWrap as LNFlexWrap},
    Property, PropertyId,
  },
  rules::CssRule,
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
  traits::ToCss,
  values::{
    gradient::{Gradient, GradientItem, LineDirection},
    image::Image,
    length::{LengthPercentageOrAuto, LengthValue},
    number::CSSNumber,
    percentage::DimensionPercentage,
    position::{
      HorizontalPositionKeyword,
      PositionComponent::{Center, Length, Side},
      VerticalPositionKeyword,
    },
  },
  visit_types,
  visitor::{Visit, VisitTypes, Visitor},
};
use smallvec::SmallVec;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ArrayLit, Bool, ComputedPropName, Expr, ExprOrSpread, Ident, KeyValueProp, Lit, MemberExpr,
  MemberProp, Number, ObjectLit, Prop, PropName, PropOrSpread, Str,
};

use crate::{document::JSXDocument, utils::to_camel_case, visitor::SpanKey};

fn parse_background_position_item(position: &BackgroundPosition) -> ImagePosition {
  match &position.x {
    Center => match &position.y {
      Center => ImagePosition::Center,
      Side { side, .. } => match side {
        VerticalPositionKeyword::Top => ImagePosition::Top,
        VerticalPositionKeyword::Bottom => ImagePosition::Bottom,
      },
      Length(length_percentage) => ImagePosition::ImagePositionXY(
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
        Length(length_percentage) => ImagePosition::ImagePositionXY(
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
        Length(length_percentage) => ImagePosition::ImagePositionXY(
          "100%".to_string(),
          length_percentage
            .to_css_string(PrinterOptions::default())
            .unwrap(),
        ),
      },
    },
    Length(length_percentage) => match &position.y {
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
      Length(length_percentage) => ImagePosition::ImagePositionXY(
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

fn parse_background_position(
  position: &SmallVec<[BackgroundPosition; 1]>,
) -> BackgroundImagePosition {
  let mut background_position = vec![];
  for item in position {
    background_position.push(parse_background_position_item(item));
  }
  BackgroundImagePosition(background_position)
}

fn parse_background_size_item(size_item: &BackgroundSize) -> Option<ImageSize> {
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

fn parse_background_size(size: &SmallVec<[BackgroundSize; 1]>) -> BackgroundImageSize {
  let mut background_size = vec![];
  for item in size {
    let item_size = parse_background_size_item(item);
    if let Some(size) = item_size {
      background_size.push(size);
    }
  }

  BackgroundImageSize(background_size)
}

fn parse_background_image_item(
  image: &Image,
  repeat: &BackgroundRepeat,
) -> Option<BackgroundImageItem> {
  match image {
    Image::Url(url) => Some(BackgroundImageItem {
      image: BackgroundImageKind::String(url.to_css_string(PrinterOptions::default()).unwrap()),
      repeat: Some(ImageRepeat::from(repeat.clone())),
    }),
    Image::Gradient(gradient) => {
      if let Gradient::Linear(gradient) = &**gradient {
        let mut color_stops = vec![];
        for item in &gradient.items {
          match item {
            GradientItem::ColorStop(color_stop) => {
              color_stops.push((
                color_stop
                  .color
                  .to_css_string(PrinterOptions::default())
                  .unwrap(),
                color_stop
                  .position
                  .clone()
                  .unwrap_or(DimensionPercentage::Dimension(LengthValue::Px(0.0)))
                  .to_css_string(PrinterOptions::default())
                  .unwrap(),
              ));
            }
            _ => {}
          };
        }
        let repeating = if repeat.x == BackgroundRepeatKeyword::Repeat
          && repeat.y == BackgroundRepeatKeyword::Repeat
        {
          true
        } else {
          false
        };
        let direction = &gradient.direction;
        match direction {
          LineDirection::Angle(angle) => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: Some(angle.to_css_string(PrinterOptions::default()).unwrap()),
              color_stops,
              derection: None,
              repeating,
            }),
            repeat: None,
          }),
          LineDirection::Horizontal(horizontal) => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match horizontal {
                HorizontalPositionKeyword::Left => LinearGradientDirection::Left,
                HorizontalPositionKeyword::Right => LinearGradientDirection::Right,
              }),
              repeating,
            }),
            repeat: None,
          }),
          LineDirection::Vertical(vertical) => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match vertical {
                VerticalPositionKeyword::Top => LinearGradientDirection::Top,
                VerticalPositionKeyword::Bottom => LinearGradientDirection::Bottom,
              }),
              repeating,
            }),
            repeat: None,
          }),
          LineDirection::Corner {
            horizontal,
            vertical,
          } => Some(BackgroundImageItem {
            image: BackgroundImageKind::LinearGradient(LinearGradientItem {
              angle: None,
              color_stops,
              derection: Some(match (horizontal, vertical) {
                (HorizontalPositionKeyword::Left, VerticalPositionKeyword::Top) => {
                  LinearGradientDirection::LeftTop
                }
                (HorizontalPositionKeyword::Left, VerticalPositionKeyword::Bottom) => {
                  LinearGradientDirection::LeftBottom
                }
                (HorizontalPositionKeyword::Right, VerticalPositionKeyword::Top) => {
                  LinearGradientDirection::RightTop
                }
                (HorizontalPositionKeyword::Right, VerticalPositionKeyword::Bottom) => {
                  LinearGradientDirection::RightBottom
                }
              }),
              repeating,
            }),
            repeat: None,
          }),
        }
      } else {
        None
      }
    }
    _ => None,
  }
}

fn parse_background_image(
  image: &SmallVec<[Image; 1]>,
  repeat: Option<&SmallVec<[BackgroundRepeat; 1]>>,
) -> BackgroundImage {
  let mut background_image = vec![];
  for (index, item) in image.iter().enumerate() {
    if let Some(item) = parse_background_image_item(
      item,
      &repeat
        .map(|item| item[index].clone())
        .unwrap_or(BackgroundRepeat {
          x: BackgroundRepeatKeyword::NoRepeat,
          y: BackgroundRepeatKeyword::NoRepeat,
        }),
    ) {
      background_image.push(item);
    }
  }
  BackgroundImage(background_image)
}

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
    background_color = Some(item.color.to_css_string(PrinterOptions::default()).unwrap());
  }
  Background {
    image: BackgroundImage(background_image),
    position: BackgroundImagePosition(background_position),
    size: BackgroundImageSize(background_size),
    color: BackgroundColor(background_color.unwrap_or("".to_string())),
  }
}

fn parse_flex_size(flex: &Flex) -> FlexSize {
  let mut flex_size = FlexSize {
    grow: None,
    shrink: None,
    basis: None,
  };
  flex_size.grow = Some(FlexGrow(flex.grow));
  flex_size.shrink = Some(FlexShrink(flex.shrink));
  match &flex.basis {
    LengthPercentageOrAuto::Auto => {
      flex_size.basis = Some(FlexBasis::String("auto".to_string()));
    }
    LengthPercentageOrAuto::LengthPercentage(value) => match value {
      DimensionPercentage::Dimension(value) => {
        flex_size.basis = Some(FlexBasis::Number(value.to_unit_value().0));
      }
      DimensionPercentage::Percentage(value) => {
        flex_size.basis = Some(FlexBasis::String(
          value.to_css_string(PrinterOptions::default()).unwrap(),
        ));
      }
      _ => {
        flex_size.basis = Some(FlexBasis::String("auto".to_string()));
      }
    },
  }
  flex_size
}

fn parse_flex_basis(flex_basis: &LengthPercentageOrAuto) -> FlexBasis {
  match flex_basis {
    LengthPercentageOrAuto::Auto => FlexBasis::String("auto".to_string()),
    LengthPercentageOrAuto::LengthPercentage(value) => match value {
      DimensionPercentage::Dimension(value) => FlexBasis::Number(value.to_unit_value().0),
      DimensionPercentage::Percentage(value) => {
        FlexBasis::String(value.to_css_string(PrinterOptions::default()).unwrap())
      }
      _ => FlexBasis::String("auto".to_string()),
    },
  }
}

pub type StyleValue = HashMap<String, StyleValueType>;

pub struct StyleData {
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleValue>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

pub trait ToExpr {
  fn to_expr(&self) -> Expr;
}

#[derive(Debug, Clone)]
pub struct TextDecoration {
  pub kind: String,
  pub color: String,
}

impl TextDecoration {
  pub fn change_color(&mut self, color: &str) {
    self.color = color.to_string();
  }
}

impl ToExpr for TextDecoration {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
          value: Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(Ident::new(
              "TextDecorationType".into(),
              DUMMY_SP,
            ))),
            prop: MemberProp::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Expr::Lit(Lit::Str(Str::from(to_camel_case(self.kind.as_str(), true)))).into(),
            }),
          })
          .into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("color".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.color.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for TextDecoration {
  fn from(value: &str) -> Self {
    TextDecoration {
      kind: if value == "" {
        "None".to_string()
      } else {
        value.to_string()
      },
      color: "black".to_string(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct BorderRadius {
  pub top_left: String,
  pub top_right: String,
  pub bottom_left: String,
  pub bottom_right: String,
}

impl BorderRadius {
  pub fn new() -> Self {
    BorderRadius {
      top_left: "0".to_string(),
      top_right: "0".to_string(),
      bottom_left: "0".to_string(),
      bottom_right: "0".to_string(),
    }
  }
  pub fn is_zero(&self) -> bool {
    self.top_left == "0"
      && self.top_right == "0"
      && self.bottom_left == "0"
      && self.bottom_right == "0"
  }

  pub fn set_top_left(&mut self, top_left: &str) {
    self.top_left = top_left.to_string();
  }

  pub fn set_top_right(&mut self, top_right: &str) {
    self.top_right = top_right.to_string();
  }

  pub fn set_bottom_left(&mut self, bottom_left: &str) {
    self.bottom_left = bottom_left.to_string();
  }

  pub fn set_bottom_right(&mut self, bottom_right: &str) {
    self.bottom_right = bottom_right.to_string();
  }
}

impl ToExpr for BorderRadius {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("topLeft".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top_left.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("topRight".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top_right.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottomLeft".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom_left.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottomRight".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom_right.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for BorderRadius {
  fn from(value: &str) -> Self {
    let border_radius_parsed = Property::parse_string(
      PropertyId::from("border-radius"),
      value,
      ParserOptions::default(),
    );
    let mut border_radius = BorderRadius::new();
    if let Ok(value) = border_radius_parsed {
      match value {
        Property::BorderRadius(value, _) => {
          border_radius.set_top_left(
            value
              .top_left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_top_right(
            value
              .top_right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_bottom_left(
            value
              .bottom_left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          border_radius.set_bottom_right(
            value
              .bottom_right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
        }
        _ => {}
      }
    }
    border_radius
  }
}

#[derive(Debug, Clone)]
pub struct MarginPadding {
  pub top: String,
  pub right: String,
  pub bottom: String,
  pub left: String,
}

impl MarginPadding {
  pub fn new() -> Self {
    MarginPadding {
      top: "0".to_string(),
      right: "0".to_string(),
      bottom: "0".to_string(),
      left: "0".to_string(),
    }
  }

  pub fn is_zero(&self) -> bool {
    self.top == "0" && self.right == "0" && self.bottom == "0" && self.left == "0"
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = top.to_string();
  }

  pub fn set_right(&mut self, right: &str) {
    self.right = right.to_string();
  }

  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = bottom.to_string();
  }

  pub fn set_left(&mut self, left: &str) {
    self.left = left.to_string();
  }
}

impl ToExpr for MarginPadding {
  fn to_expr(&self) -> Expr {
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.top.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.right.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.bottom.to_string()))).into(),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
          value: Expr::Lit(Lit::Str(Str::from(self.left.to_string()))).into(),
        }))),
      ]
      .into(),
    })
  }
}

impl From<&str> for MarginPadding {
  fn from(value: &str) -> Self {
    let margin_padding_parsed =
      Property::parse_string(PropertyId::from("Margin"), value, ParserOptions::default());
    let mut margin_padding = MarginPadding::new();
    if let Ok(value) = margin_padding_parsed {
      match value {
        Property::Margin(value) => {
          margin_padding.set_top(
            value
              .top
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin_padding.set_right(
            value
              .right
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin_padding.set_bottom(
            value
              .bottom
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
          margin_padding.set_left(
            value
              .left
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
          );
        }
        _ => {}
      }
    }
    margin_padding
  }
}

#[derive(Debug, Clone)]
pub enum LinearGradientDirection {
  Left,
  Right,
  Top,
  Bottom,
  LeftTop,
  LeftBottom,
  RightTop,
  RightBottom,
}

#[derive(Debug, Clone)]
pub struct LinearGradientItem {
  pub angle: Option<String>,
  pub color_stops: Vec<(String, String)>,
  pub derection: Option<LinearGradientDirection>,
  pub repeating: bool,
}

impl ToExpr for LinearGradientItem {
  fn to_expr(&self) -> Expr {
    let mut props = vec![];
    if let Some(angle) = &self.angle {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("angle".into(), DUMMY_SP)),
        value: Expr::Lit(Lit::Str(Str::from(angle.to_string()))).into(),
      }))));
    }
    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("colors".into(), DUMMY_SP)),
      value: Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: self
          .color_stops
          .iter()
          .map(|item| {
            Some(ExprOrSpread {
              spread: None,
              expr: Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: vec![
                  Some(Expr::Lit(Lit::Str(Str::from(item.0.to_string()))).into()),
                  Some(Expr::Lit(Lit::Str(Str::from(item.1.to_string()))).into()),
                ],
              })
              .into(),
            })
          })
          .collect::<Vec<_>>(),
      })
      .into(),
    }))));
    if let Some(derection) = &self.derection {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("direction".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new(
            "GradientDirection".into(),
            DUMMY_SP,
          ))),
          prop: MemberProp::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Expr::Lit(Lit::Str(Str::from(match derection {
              LinearGradientDirection::Left => "Left",
              LinearGradientDirection::Right => "Right",
              LinearGradientDirection::Top => "Top",
              LinearGradientDirection::Bottom => "Bottom",
              LinearGradientDirection::LeftTop => "LeftTop",
              LinearGradientDirection::LeftBottom => "LeftBottom",
              LinearGradientDirection::RightTop => "RightTop",
              LinearGradientDirection::RightBottom => "RightBottom",
            })))
            .into(),
          }),
        })
        .into(),
      }))));
    }
    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new("repeating".into(), DUMMY_SP)),
      value: Expr::Lit(Lit::Bool(Bool {
        span: DUMMY_SP,
        value: self.repeating,
      }))
      .into(),
    }))));
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}

#[derive(Debug, Clone)]
pub struct LinearGradient(pub Vec<LinearGradientItem>);

impl ToExpr for LinearGradient {
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

#[derive(Debug, Clone)]
pub struct BackgroundColor(pub String);

impl ToExpr for BackgroundColor {
  fn to_expr(&self) -> Expr {
    Expr::Lit(Lit::Str(Str::from(self.0.to_string()))).into()
  }
}

pub struct BackgroundImageStr {
  pub src: String,
  pub repeat: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BackgroundImage(pub Vec<BackgroundImageItem>);

impl ToExpr for BackgroundImage {
  fn to_expr(&self) -> Expr {
    Expr::Array(ArrayLit {
      span: DUMMY_SP,
      elems: self
        .0
        .iter()
        .map(|item| match &item.image {
          BackgroundImageKind::String(src) => Some(
            Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("src".into(), DUMMY_SP)),
                  value: Expr::Lit(Lit::Str(Str::from(src.to_string()))).into(),
                }))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("repeat".into(), DUMMY_SP)),
                  value: Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident::new("ImageRepeat".into(), DUMMY_SP))),
                    prop: MemberProp::Ident(Ident {
                      span: DUMMY_SP,
                      sym: if let Some(repeat) = &self.0[0].repeat {
                        match repeat {
                          ImageRepeat::XY => "XY",
                          ImageRepeat::X => "X",
                          ImageRepeat::Y => "Y",
                          ImageRepeat::NoRepeat => "NoRepeat",
                        }
                        .into()
                      } else {
                        "NoRepeat".into()
                      },
                      optional: false,
                    }),
                  })
                  .into(),
                }))),
              ]
              .into(),
            })
            .into(),
          ),
          BackgroundImageKind::LinearGradient(linear_gradient) => {
            Some(linear_gradient.to_expr().into())
          }
        })
        .collect::<Vec<_>>(),
    })
  }
}

impl From<&BackgroundImageStr> for BackgroundImage {
  fn from(value: &BackgroundImageStr) -> Self {
    let repeat_str = value.repeat.clone().unwrap_or("no-repeat".to_string());
    let background_image_parsed = Property::parse_string(
      PropertyId::from("background-image"),
      &value.src,
      ParserOptions::default(),
    );
    let background_image_repeat_parsed = Property::parse_string(
      PropertyId::from("background-repeat"),
      &repeat_str,
      ParserOptions::default(),
    );
    let mut background_image = BackgroundImage(vec![]);
    if let Ok(value) = background_image_parsed {
      match value {
        Property::BackgroundImage(value) => {
          background_image = parse_background_image(
            &value,
            if let Ok(repeat_parsed) = &background_image_repeat_parsed {
              match repeat_parsed {
                Property::BackgroundRepeat(value) => Some(value),
                _ => None,
              }
            } else {
              None
            },
          );
        }
        _ => {}
      }
    }
    background_image
  }
}

#[derive(Debug, Clone)]
pub enum BackgroundImageKind {
  String(String),
  LinearGradient(LinearGradientItem),
}

#[derive(Debug, Clone)]
pub struct BackgroundImageItem {
  pub image: BackgroundImageKind,
  pub repeat: Option<ImageRepeat>,
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

impl From<&str> for BackgroundImageSize {
  fn from(value: &str) -> Self {
    let background_image_size_parsed = Property::parse_string(
      PropertyId::from("background-size"),
      value,
      ParserOptions::default(),
    );
    let mut background_image_size = BackgroundImageSize(vec![]);
    if let Ok(value) = background_image_size_parsed {
      match value {
        Property::BackgroundSize(value) => {
          background_image_size = parse_background_size(&value);
        }
        _ => {}
      }
    }
    background_image_size
  }
}

#[derive(Debug, Clone)]
pub enum ImageRepeat {
  XY,
  X,
  Y,
  NoRepeat,
}

impl From<&str> for ImageRepeat {
  fn from(value: &str) -> Self {
    match value {
      "repeat" => ImageRepeat::XY,
      "repeat-x" => ImageRepeat::X,
      "repeat-y" => ImageRepeat::Y,
      _ => ImageRepeat::NoRepeat,
    }
  }
}

impl From<BackgroundRepeat> for ImageRepeat {
  fn from(value: BackgroundRepeat) -> Self {
    if value.x == BackgroundRepeatKeyword::Repeat && value.y == BackgroundRepeatKeyword::Repeat {
      ImageRepeat::XY
    } else if value.x == BackgroundRepeatKeyword::Repeat {
      ImageRepeat::X
    } else if value.y == BackgroundRepeatKeyword::Repeat {
      ImageRepeat::Y
    } else {
      ImageRepeat::NoRepeat
    }
  }
}

#[derive(Debug, Clone)]
pub enum ImageSize {
  Cover,
  Contain,
  Auto,
  ImageSizeWH(String, String),
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
pub struct BackgroundImagePosition(pub Vec<ImagePosition>);

impl ToExpr for BackgroundImagePosition {
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
                  value: Expr::Lit(Lit::Str(Str::from(x.to_string()))).into(),
                }))),
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("y".into(), DUMMY_SP)),
                  value: Expr::Lit(Lit::Str(Str::from(y.to_string()))).into(),
                }))),
              ]
              .into(),
            })
            .into(),
            ImagePosition::TopStart => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "TopStart".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Top => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Top".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::TopEnd => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "TopEnd".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Start => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Start".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Center => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Center".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::End => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "End".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::BottomStart => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "BottomStart".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::Bottom => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
              prop: MemberProp::Ident(Ident {
                span: DUMMY_SP,
                sym: "Bottom".into(),
                optional: false,
              }),
            })
            .into(),
            ImagePosition::BottomEnd => Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(Ident::new("ImagePosition".into(), DUMMY_SP))),
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

impl From<&str> for BackgroundImagePosition {
  fn from(value: &str) -> Self {
    let background_image_position_parsed = Property::parse_string(
      PropertyId::from("background-position"),
      value,
      ParserOptions::default(),
    );
    let mut background_image_position = BackgroundImagePosition(vec![]);
    if let Ok(value) = background_image_position_parsed {
      match value {
        Property::BackgroundPosition(value) => {
          background_image_position = parse_background_position(&value);
        }
        _ => {}
      }
    }
    background_image_position
  }
}

#[derive(Debug, Clone)]
pub struct Background {
  pub image: BackgroundImage,
  pub color: BackgroundColor,
  pub size: BackgroundImageSize,
  pub position: BackgroundImagePosition,
}

impl From<&str> for Background {
  fn from(value: &str) -> Self {
    let background_parsed = Property::parse_string(
      PropertyId::from("background"),
      value,
      ParserOptions::default(),
    );
    let mut background = Background {
      image: BackgroundImage(vec![]),
      color: BackgroundColor("".to_string()),
      size: BackgroundImageSize(vec![]),
      position: BackgroundImagePosition(vec![]),
    };
    if let Ok(property) = background_parsed {
      if let Property::Background(value) = property {
        background = parse_background(&value);
      }
    }

    background
  }
}

impl From<&SmallVec<[LNBackground<'_>; 1]>> for Background {
  fn from(value: &SmallVec<[LNBackground<'_>; 1]>) -> Self {
    parse_background(value)
  }
}

#[derive(Debug, Clone)]
pub enum FlexDirection {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl From<&str> for FlexDirection {
  fn from(value: &str) -> Self {
    match value {
      "row" => FlexDirection::Row,
      "row-reverse" => FlexDirection::RowReverse,
      "column" => FlexDirection::Column,
      "column-reverse" => FlexDirection::ColumnReverse,
      _ => FlexDirection::Row,
    }
  }
}

impl From<&LNFlexDirection> for FlexDirection {
  fn from(value: &LNFlexDirection) -> Self {
    match value {
      LNFlexDirection::Row => FlexDirection::Row,
      LNFlexDirection::RowReverse => FlexDirection::RowReverse,
      LNFlexDirection::Column => FlexDirection::Column,
      LNFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
    }
  }
}

#[derive(Debug, Clone)]
pub enum FlexWrap {
  Wrap,
  WrapReverse,
  NoWrap,
}

impl From<&str> for FlexWrap {
  fn from(value: &str) -> Self {
    match value {
      "wrap" => FlexWrap::Wrap,
      "wrap-reverse" => FlexWrap::WrapReverse,
      "nowrap" => FlexWrap::NoWrap,
      _ => FlexWrap::NoWrap,
    }
  }
}

impl From<&LNFlexWrap> for FlexWrap {
  fn from(value: &LNFlexWrap) -> Self {
    match value {
      LNFlexWrap::Wrap => FlexWrap::Wrap,
      LNFlexWrap::WrapReverse => FlexWrap::WrapReverse,
      LNFlexWrap::NoWrap => FlexWrap::NoWrap,
    }
  }
}

#[derive(Debug, Clone)]
pub enum FlexAlign {
  Start,
  Center,
  End,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}

impl From<&str> for FlexAlign {
  fn from(value: &str) -> Self {
    match value {
      "flex-start" | "start" => FlexAlign::Start,
      "center" => FlexAlign::Center,
      "flex-end" | "end" => FlexAlign::End,
      "space-between" => FlexAlign::SpaceBetween,
      "space-around" => FlexAlign::SpaceAround,
      "space-evenly" => FlexAlign::SpaceEvenly,
      _ => FlexAlign::Start,
    }
  }
}

impl From<&LNJustifyContent> for FlexAlign {
  fn from(value: &LNJustifyContent) -> Self {
    match value {
      LNJustifyContent::ContentPosition { value, .. } => match value {
        ContentPosition::Start | ContentPosition::FlexStart => FlexAlign::Start,
        ContentPosition::Center => FlexAlign::Center,
        ContentPosition::End | ContentPosition::FlexEnd => FlexAlign::End,
      },
      LNJustifyContent::ContentDistribution(value) => match value {
        ContentDistribution::SpaceBetween => FlexAlign::SpaceBetween,
        ContentDistribution::SpaceAround => FlexAlign::SpaceAround,
        ContentDistribution::SpaceEvenly => FlexAlign::SpaceEvenly,
        _ => FlexAlign::Start,
      },
      _ => FlexAlign::Start,
    }
  }
}

impl From<&LNAlignContent> for FlexAlign {
  fn from(value: &LNAlignContent) -> Self {
    match value {
      LNAlignContent::ContentPosition { value, .. } => match value {
        ContentPosition::Start | ContentPosition::FlexStart => FlexAlign::Start,
        ContentPosition::Center => FlexAlign::Center,
        ContentPosition::End | ContentPosition::FlexEnd => FlexAlign::End,
      },
      LNAlignContent::ContentDistribution(value) => match value {
        ContentDistribution::SpaceBetween => FlexAlign::SpaceBetween,
        ContentDistribution::SpaceAround => FlexAlign::SpaceAround,
        ContentDistribution::SpaceEvenly => FlexAlign::SpaceEvenly,
        _ => FlexAlign::Start,
      },
      _ => FlexAlign::Start,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemAlign {
  Auto,
  Start,
  Center,
  End,
  Stretch,
  Baseline,
  Ignore,
}

impl ToExpr for ItemAlign {
  fn to_expr(&self) -> Expr {
    Expr::Member(MemberExpr {
      span: DUMMY_SP,
      obj: Box::new(Expr::Ident(Ident::new("ItemAlign".into(), DUMMY_SP))),
      prop: MemberProp::Ident(Ident {
        span: DUMMY_SP,
        sym: match self {
          ItemAlign::Auto => "Auto",
          ItemAlign::Start => "Start",
          ItemAlign::Center => "Center",
          ItemAlign::End => "End",
          ItemAlign::Stretch => "Stretch",
          ItemAlign::Baseline => "Baseline",
          ItemAlign::Ignore => "",
        }
        .into(),
        optional: false,
      }),
    })
    .into()
  }
}

impl From<&str> for ItemAlign {
  fn from(value: &str) -> Self {
    match value {
      "auto" => ItemAlign::Auto,
      "flex-start" | "start" => ItemAlign::Start,
      "center" => ItemAlign::Center,
      "flex-end" | "end" => ItemAlign::End,
      "stretch" => ItemAlign::Stretch,
      "baseline" => ItemAlign::Baseline,
      _ => ItemAlign::Auto,
    }
  }
}

impl From<&LNAlignItems> for ItemAlign {
  fn from(value: &LNAlignItems) -> Self {
    match value {
      LNAlignItems::Stretch => ItemAlign::Stretch,
      LNAlignItems::SelfPosition { value, .. } => match value {
        SelfPosition::Start => ItemAlign::Start,
        SelfPosition::Center => ItemAlign::Center,
        SelfPosition::End => ItemAlign::End,
        _ => ItemAlign::Ignore,
      },
      LNAlignItems::BaselinePosition(value) => match value {
        BaselinePosition::Last => ItemAlign::Ignore,
        _ => ItemAlign::Baseline,
      },
      _ => ItemAlign::Auto,
    }
  }
}

impl From<&LNAlignSelf> for ItemAlign {
  fn from(value: &LNAlignSelf) -> Self {
    match value {
      LNAlignSelf::Auto => ItemAlign::Auto,
      LNAlignSelf::SelfPosition { value, .. } => match value {
        SelfPosition::Start => ItemAlign::Start,
        SelfPosition::Center => ItemAlign::Center,
        SelfPosition::End => ItemAlign::End,
        _ => ItemAlign::Ignore,
      },
      LNAlignSelf::BaselinePosition(value) => match value {
        BaselinePosition::Last => ItemAlign::Ignore,
        _ => ItemAlign::Baseline,
      },
      _ => ItemAlign::Auto,
    }
  }
}

#[derive(Debug, Clone)]
pub struct FlexOptions {
  pub direction: Option<FlexDirection>,
  pub wrap: Option<FlexWrap>,
  pub justify_content: Option<FlexAlign>,
  pub align_items: Option<ItemAlign>,
  pub align_content: Option<FlexAlign>,
}

impl FlexOptions {
  pub fn new() -> Self {
    FlexOptions {
      direction: None,
      wrap: None,
      justify_content: None,
      align_items: None,
      align_content: None,
    }
  }
}

impl ToExpr for FlexOptions {
  fn to_expr(&self) -> Expr {
    let mut props = vec![];
    if let Some(direction) = &self.direction {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("direction".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FlexDirection".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match direction {
              FlexDirection::Row => "Row",
              FlexDirection::RowReverse => "RowReverse",
              FlexDirection::Column => "Column",
              FlexDirection::ColumnReverse => "ColumnReverse",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }
    if let Some(wrap) = &self.wrap {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("wrap".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FlexWrap".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match wrap {
              FlexWrap::Wrap => "Wrap",
              FlexWrap::WrapReverse => "WrapReverse",
              FlexWrap::NoWrap => "NoWrap",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }
    if let Some(justify_content) = &self.justify_content {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("justifyContent".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FlexAlign".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match justify_content {
              FlexAlign::Start => "Start",
              FlexAlign::Center => "Center",
              FlexAlign::End => "End",
              FlexAlign::SpaceBetween => "SpaceBetween",
              FlexAlign::SpaceAround => "SpaceAround",
              FlexAlign::SpaceEvenly => "SpaceEvenly",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }
    if let Some(align_items) = &self.align_items {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("alignItems".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("ItemAlign".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match align_items {
              ItemAlign::Auto => "Auto",
              ItemAlign::Start => "Start",
              ItemAlign::Center => "Center",
              ItemAlign::End => "End",
              ItemAlign::Stretch => "Stretch",
              ItemAlign::Baseline => "Baseline",
              ItemAlign::Ignore => "",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }
    if let Some(align_content) = &self.align_content {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("alignContent".into(), DUMMY_SP)),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("FlexAlign".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match align_content {
              FlexAlign::Start => "Start",
              FlexAlign::Center => "Center",
              FlexAlign::End => "End",
              FlexAlign::SpaceBetween => "SpaceBetween",
              FlexAlign::SpaceAround => "SpaceAround",
              FlexAlign::SpaceEvenly => "SpaceEvenly",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}

#[derive(Debug, Clone)]
pub struct FlexGrow(pub CSSNumber);

impl ToExpr for FlexGrow {
  fn to_expr(&self) -> Expr {
    Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value: self.0 as f64,
      raw: None,
    }))
    .into()
  }
}

impl From<&str> for FlexGrow {
  fn from(value: &str) -> Self {
    let flex_grow_parsed = Property::parse_string(
      PropertyId::from("flex-grow"),
      value,
      ParserOptions::default(),
    );
    let mut flex_grow = FlexGrow(0.0);
    if let Ok(value) = flex_grow_parsed {
      match value {
        Property::FlexGrow(value, _) => {
          flex_grow = FlexGrow(value);
        }
        _ => {}
      }
    }
    flex_grow
  }
}

#[derive(Debug, Clone)]
pub struct FlexShrink(pub CSSNumber);

impl ToExpr for FlexShrink {
  fn to_expr(&self) -> Expr {
    Expr::Lit(Lit::Num(Number {
      span: DUMMY_SP,
      value: self.0 as f64,
      raw: None,
    }))
    .into()
  }
}

impl From<&str> for FlexShrink {
  fn from(value: &str) -> Self {
    let flex_shrink_parsed = Property::parse_string(
      PropertyId::from("flex-shrink"),
      value,
      ParserOptions::default(),
    );
    let mut flex_shrink = FlexShrink(0.0);
    if let Ok(value) = flex_shrink_parsed {
      match value {
        Property::FlexShrink(value, _) => {
          flex_shrink = FlexShrink(value);
        }
        _ => {}
      }
    }
    flex_shrink
  }
}

#[derive(Debug, Clone)]
pub enum FlexBasis {
  String(String),
  Number(CSSNumber),
}

impl ToExpr for FlexBasis {
  fn to_expr(&self) -> Expr {
    match self {
      FlexBasis::String(value) => Expr::Lit(Lit::Str(Str::from(value.to_string()))).into(),
      FlexBasis::Number(value) => Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: *value as f64,
        raw: None,
      }))
      .into(),
    }
  }
}

impl From<&str> for FlexBasis {
  fn from(value: &str) -> Self {
    let flex_basis_parsed = Property::parse_string(
      PropertyId::from("flex-basis"),
      value,
      ParserOptions::default(),
    );
    let mut flex_basis = FlexBasis::String("auto".to_string());
    if let Ok(value) = flex_basis_parsed {
      match value {
        Property::FlexBasis(value, _) => {
          flex_basis = parse_flex_basis(&value);
        }
        _ => {}
      }
    }
    flex_basis
  }
}

#[derive(Debug, Clone)]
pub struct FlexSize {
  pub grow: Option<FlexGrow>,
  pub shrink: Option<FlexShrink>,
  pub basis: Option<FlexBasis>,
}

impl FlexSize {
  pub fn new() -> Self {
    FlexSize {
      grow: None,
      shrink: None,
      basis: None,
    }
  }
}

impl From<&str> for FlexSize {
  fn from(value: &str) -> Self {
    let flex_size_parsed =
      Property::parse_string(PropertyId::from("flex"), value, ParserOptions::default());
    if let Ok(flex_size_parsed) = flex_size_parsed {
      match flex_size_parsed {
        Property::Flex(flex, _) => parse_flex_size(&flex),
        _ => FlexSize::new(),
      }
    } else {
      FlexSize::new()
    }
  }
}

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(String),
  TextDecoration(TextDecoration),
  BorderRadius(BorderRadius),
  MarginPadding(MarginPadding),
  BackgroundImage(BackgroundImage),
  BackgroundColor(BackgroundColor),
  BackgroundImageSize(BackgroundImageSize),
  BackgroundImagePosition(BackgroundImagePosition),
  LinearGradient(LinearGradient),
  FlexOptions(FlexOptions),
  AlignSelf(ItemAlign),
  FlexGrow(FlexGrow),
  FlexShrink(FlexShrink),
  FlexBasis(FlexBasis),
}

impl Display for StyleValueType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StyleValueType::Normal(value) => write!(f, "{}", value),
      StyleValueType::TextDecoration(value) => {
        write!(f, "{}", to_camel_case(value.kind.as_str(), true))
      }
      StyleValueType::BorderRadius(value) => {
        write!(
          f,
          "{} {} {} {}",
          value.top_left, value.top_right, value.bottom_left, value.bottom_right
        )
      }
      StyleValueType::MarginPadding(value) => {
        write!(
          f,
          "{} {} {} {}",
          value.top, value.right, value.bottom, value.left
        )
      }
      StyleValueType::BackgroundImage(value) => {
        let mut image = "".to_string();
        for item in value.0.iter() {
          if let BackgroundImageKind::String(src) = &item.image {
            image.push_str(src.as_str());
            image.push_str(" ");
            if let Some(repeat) = &item.repeat {
              match repeat {
                ImageRepeat::XY => {
                  image.push_str("repeat");
                }
                ImageRepeat::X => {
                  image.push_str("repeat-x");
                }
                ImageRepeat::Y => {
                  image.push_str("repeat-y");
                }
                ImageRepeat::NoRepeat => {
                  image.push_str("no-repeat");
                }
              }
              image.push_str(" ");
            }
          }
        }
        write!(f, "{}", image)
      }
      StyleValueType::BackgroundColor(value) => write!(f, "{}", value.0),
      StyleValueType::BackgroundImageSize(value) => {
        let mut size = "".to_string();
        for item in value.0.iter() {
          match item {
            ImageSize::Cover => {
              size.push_str("cover");
            }
            ImageSize::Contain => {
              size.push_str("contain");
            }
            ImageSize::Auto => {
              size.push_str("auto");
            }
            ImageSize::ImageSizeWH(width, height) => {
              size.push_str(width.as_str());
              size.push_str(" ");
              size.push_str(height.as_str());
            }
          }
          size.push_str(" ");
        }
        write!(f, "{}", size)
      }
      StyleValueType::BackgroundImagePosition(value) => {
        let mut position = "".to_string();
        for item in value.0.iter() {
          match item {
            ImagePosition::ImagePositionXY(x, y) => {
              position.push_str(x.as_str());
              position.push_str(" ");
              position.push_str(y.as_str());
            }
            ImagePosition::TopStart => {
              position.push_str("top left");
            }
            ImagePosition::Top => {
              position.push_str("top center");
            }
            ImagePosition::TopEnd => {
              position.push_str("top right");
            }
            ImagePosition::Start => {
              position.push_str("center left");
            }
            ImagePosition::Center => {
              position.push_str("center center");
            }
            ImagePosition::End => {
              position.push_str("center right");
            }
            ImagePosition::BottomStart => {
              position.push_str("bottom left");
            }
            ImagePosition::Bottom => {
              position.push_str("bottom center");
            }
            ImagePosition::BottomEnd => {
              position.push_str("bottom right");
            }
          }
          position.push_str(" ");
        }
        write!(f, "{}", position)
      }
      StyleValueType::LinearGradient(linear_gradient) => {
        let mut linear_gradient_str = "".to_string();
        for item in linear_gradient.0.iter() {
          if let Some(angle) = &item.angle {
            linear_gradient_str.push_str(angle.as_str());
            linear_gradient_str.push_str(" ");
          }
          for (index, color_stop) in item.color_stops.iter().enumerate() {
            linear_gradient_str.push_str(color_stop.0.as_str());
            linear_gradient_str.push_str(" ");
            linear_gradient_str.push_str(color_stop.1.as_str());
            if index != item.color_stops.len() - 1 {
              linear_gradient_str.push_str(", ");
            }
          }
          linear_gradient_str.push_str(" ");
        }
        write!(f, "{}", linear_gradient_str)
      }
      StyleValueType::FlexOptions(flex_options) => {
        let mut flex_options_str = "".to_string();
        if let Some(direction) = &flex_options.direction {
          flex_options_str.push_str(match direction {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
          });
          flex_options_str.push_str(" ");
        }
        if let Some(wrap) = &flex_options.wrap {
          flex_options_str.push_str(match wrap {
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
            FlexWrap::NoWrap => "nowrap",
          });
          flex_options_str.push_str(" ");
        }
        if let Some(justify_content) = &flex_options.justify_content {
          flex_options_str.push_str(match justify_content {
            FlexAlign::Start => "start",
            FlexAlign::Center => "center",
            FlexAlign::End => "end",
            FlexAlign::SpaceBetween => "space-between",
            FlexAlign::SpaceAround => "space-around",
            FlexAlign::SpaceEvenly => "space-evenly",
          });
          flex_options_str.push_str(" ");
        }
        if let Some(align_items) = &flex_options.align_items {
          flex_options_str.push_str(match align_items {
            ItemAlign::Auto => "auto",
            ItemAlign::Start => "start",
            ItemAlign::Center => "center",
            ItemAlign::End => "end",
            ItemAlign::Stretch => "stretch",
            ItemAlign::Baseline => "baseline",
            ItemAlign::Ignore => "",
          });
          flex_options_str.push_str(" ");
        }
        if let Some(align_content) = &flex_options.align_content {
          flex_options_str.push_str(match align_content {
            FlexAlign::Start => "start",
            FlexAlign::Center => "center",
            FlexAlign::End => "end",
            FlexAlign::SpaceBetween => "space-between",
            FlexAlign::SpaceAround => "space-around",
            FlexAlign::SpaceEvenly => "space-evenly",
          });
          flex_options_str.push_str(" ");
        }
        write!(f, "{}", flex_options_str)
      }
      StyleValueType::AlignSelf(align_self) => {
        write!(
          f,
          "{}",
          match align_self {
            ItemAlign::Auto => "auto",
            ItemAlign::Start => "start",
            ItemAlign::Center => "center",
            ItemAlign::End => "end",
            ItemAlign::Stretch => "stretch",
            ItemAlign::Baseline => "baseline",
            ItemAlign::Ignore => "",
          }
        )
      }
      StyleValueType::FlexGrow(flex_grow) => {
        write!(f, "{}", flex_grow.0)
      }
      StyleValueType::FlexShrink(flex_shrink) => {
        write!(f, "{}", flex_shrink.0)
      }
      StyleValueType::FlexBasis(flex_basis) => match flex_basis {
        FlexBasis::String(value) => write!(f, "{}", value),
        FlexBasis::Number(value) => write!(f, "{}", value),
      },
    }
  }
}

impl ToExpr for StyleValueType {
  fn to_expr(&self) -> Expr {
    match self {
      StyleValueType::Normal(value) => value.to_string().into(),
      StyleValueType::TextDecoration(text_decoration) => text_decoration.to_expr().into(),
      StyleValueType::BorderRadius(border_radius) => border_radius.to_expr().into(),
      StyleValueType::MarginPadding(margin_padding) => margin_padding.to_expr().into(),
      StyleValueType::BackgroundImage(background_image) => background_image.to_expr().into(),
      StyleValueType::BackgroundImageSize(background_size) => background_size.to_expr().into(),
      StyleValueType::BackgroundImagePosition(background_position) => {
        background_position.to_expr().into()
      }
      StyleValueType::BackgroundColor(background_color) => background_color.to_expr().into(),
      StyleValueType::LinearGradient(linear_gradient) => linear_gradient.to_expr().into(),
      StyleValueType::FlexOptions(flex_options) => flex_options.to_expr().into(),
      StyleValueType::AlignSelf(align_self) => align_self.to_expr().into(),
      StyleValueType::FlexGrow(flex_grow) => flex_grow.to_expr().into(),
      StyleValueType::FlexShrink(flex_shrink) => flex_shrink.to_expr().into(),
      StyleValueType::FlexBasis(flex_basis) => flex_basis.to_expr().into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

pub struct StyleVisitor<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    document: &'i JSXDocument,
    all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  ) -> Self {
    StyleVisitor {
      all_style,
      document,
    }
  }
}

impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().replace(".", "");
          let mut all_style = self.all_style.borrow_mut();
          let declarations: &mut Vec<StyleDeclaration<'_>> =
            all_style.entry(selector.clone()).or_insert(vec![]);
          declarations.push(StyleDeclaration {
            specificity: style.selectors.0.get(index).unwrap().specificity(),
            declaration: style.declarations.clone(),
          });
        }
      }
      _ => {}
    }
    Ok(())
  }
}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(HashMap::new())),
      document,
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(self.document, Rc::clone(&self.all_style));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  fn calc_style_record<T: Hash + Eq + Clone>(
    &self,
    style_record: &mut HashMap<T, Vec<StyleDeclaration<'i>>>,
  ) -> HashMap<T, StyleDeclaration<'i>> {
    let mut final_style_record = HashMap::new();
    for (id, declarations) in style_record.iter_mut() {
      declarations.sort_by(|a, b| a.specificity.cmp(&b.specificity));
      let mut final_properties: Vec<Property<'i>> = Vec::new();
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let declarations = &declaration.declarations;
        for declaration in declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let important_declarations = &declaration.important_declarations;
        for declaration in important_declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      final_style_record.insert(
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: vec![],
          },
        },
      );
    }
    final_style_record
  }

  fn parse_style_style(&self, style_value: &mut StyleDeclaration<'i>) -> StyleValue {
    let properties = style_value
      .declaration
      .declarations
      .iter()
      .map(|property| {
        (
          to_camel_case(
            property
              .property_id()
              .to_css_string(PrinterOptions::default())
              .unwrap()
              .as_str(),
            false,
          ),
          property,
        )
      })
      .collect::<HashMap<_, _>>();
    let mut final_properties = HashMap::new();

    let mut text_decoration = None;
    let mut color = None;
    let mut flex_options = FlexOptions::new();

    for (id, value) in properties.iter() {
      match id.as_str() {
        "margin" => {
          let margin = match value {
            Property::Margin(value) => {
              let mut margin = MarginPadding::new();
              margin.set_top(
                value
                  .top
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              margin.set_right(
                value
                  .right
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              margin.set_bottom(
                value
                  .bottom
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              margin.set_left(
                value
                  .left
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              margin
            }
            _ => MarginPadding::new(),
          };
          if margin.is_zero() {
            final_properties.remove("margin");
          } else {
            final_properties.insert("margin".to_string(), StyleValueType::MarginPadding(margin));
          }
        }
        "marginLeft" => {
          let margin = final_properties
            .entry("margin".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(margin) = margin {
            margin.set_left(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "marginRight" => {
          let margin = final_properties
            .entry("margin".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(margin) = margin {
            margin.set_right(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "marginTop" => {
          let margin = final_properties
            .entry("margin".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(margin) = margin {
            margin.set_top(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "marginBottom" => {
          let margin = final_properties
            .entry("margin".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(margin) = margin {
            margin.set_bottom(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "padding" => {
          let padding = match value {
            Property::Padding(value) => {
              let mut padding = MarginPadding::new();
              padding.set_top(
                value
                  .top
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              padding.set_right(
                value
                  .right
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              padding.set_bottom(
                value
                  .bottom
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              padding.set_left(
                value
                  .left
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              padding
            }
            _ => MarginPadding::new(),
          };
          if padding.is_zero() {
            final_properties.remove("padding");
          } else {
            final_properties.insert(
              "padding".to_string(),
              StyleValueType::MarginPadding(padding),
            );
          }
        }
        "paddingLeft" => {
          let padding = final_properties
            .entry("padding".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(padding) = padding {
            padding.set_left(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "paddingRight" => {
          let padding = final_properties
            .entry("padding".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(padding) = padding {
            padding.set_right(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "paddingTop" => {
          let padding = final_properties
            .entry("padding".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(padding) = padding {
            padding.set_top(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "paddingBottom" => {
          let padding = final_properties
            .entry("padding".to_string())
            .or_insert(StyleValueType::MarginPadding(MarginPadding::new()));
          if let StyleValueType::MarginPadding(padding) = padding {
            padding.set_bottom(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "borderRadius" => {
          let border_radius = match value {
            Property::BorderRadius(value, _) => {
              let mut border_radius = BorderRadius::new();
              border_radius.set_top_left(
                value
                  .top_left
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              border_radius.set_top_right(
                value
                  .top_right
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              border_radius.set_bottom_left(
                value
                  .bottom_left
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              border_radius.set_bottom_right(
                value
                  .bottom_right
                  .to_css_string(PrinterOptions::default())
                  .unwrap()
                  .as_str(),
              );
              border_radius
            }
            _ => BorderRadius::new(),
          };
          if border_radius.is_zero() {
            final_properties.remove("borderRadius");
          } else {
            final_properties.insert(
              "borderRadius".to_string(),
              StyleValueType::BorderRadius(border_radius),
            );
          }
        }
        "borderRadiusTopLeft" => {
          let border_radius = final_properties
            .entry("borderRadius".to_string())
            .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
          if let StyleValueType::BorderRadius(border_radius) = border_radius {
            border_radius.set_top_left(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "borderRadiusTopRight" => {
          let border_radius = final_properties
            .entry("borderRadius".to_string())
            .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
          if let StyleValueType::BorderRadius(border_radius) = border_radius {
            border_radius.set_top_right(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "borderRadiusBottomLeft" => {
          let border_radius = final_properties
            .entry("borderRadius".to_string())
            .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
          if let StyleValueType::BorderRadius(border_radius) = border_radius {
            border_radius.set_bottom_left(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "borderRadiusBottomRight" => {
          let border_radius = final_properties
            .entry("borderRadius".to_string())
            .or_insert(StyleValueType::BorderRadius(BorderRadius::new()));
          if let StyleValueType::BorderRadius(border_radius) = border_radius {
            border_radius.set_bottom_right(
              value
                .value_to_css_string(PrinterOptions::default())
                .unwrap()
                .as_str(),
            )
          }
        }
        "textDecoration" => text_decoration = Some((*value).clone()),
        "color" => {
          color = Some((*value).clone());
          final_properties.insert(
            id.to_string(),
            StyleValueType::Normal(
              value
                .value_to_css_string(PrinterOptions {
                  minify: false,
                  targets: Targets {
                    include: Features::HexAlphaColors,
                    ..Targets::default()
                  },
                  ..PrinterOptions::default()
                })
                .unwrap(),
            ),
          );
        }
        "background" => match value {
          Property::Background(value) => {
            let background = parse_background(value);
            let mut images = vec![];
            let mut linear_gradient = vec![];
            for item in background.image.0.iter() {
              if let BackgroundImageKind::String(_) = &item.image {
                images.push(item.clone());
              } else if let BackgroundImageKind::LinearGradient(gradient) = &item.image {
                linear_gradient.push(gradient.clone());
              }
            }
            if images.len() > 0 {
              final_properties.insert(
                "backgroundImage".to_string(),
                StyleValueType::BackgroundImage(BackgroundImage(images)),
              );
            }
            if linear_gradient.len() > 0 {
              final_properties.insert(
                "linearGradient".to_string(),
                StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
              );
            }
            final_properties.insert(
              "backgroundImagePosition".to_string(),
              StyleValueType::BackgroundImagePosition(background.position),
            );
            final_properties.insert(
              "backgroundImageSize".to_string(),
              StyleValueType::BackgroundImageSize(background.size),
            );
            final_properties.insert(
              "backgroundColor".to_string(),
              StyleValueType::BackgroundColor(background.color),
            );
          }
          _ => {}
        },
        "backgroundColor" => match value {
          Property::BackgroundColor(value) => {
            final_properties.insert(
              id.to_string(),
              StyleValueType::BackgroundColor(BackgroundColor(
                value
                  .to_css_string(PrinterOptions {
                    minify: false,
                    targets: Targets {
                      include: Features::HexAlphaColors,
                      ..Targets::default()
                    },
                    ..PrinterOptions::default()
                  })
                  .unwrap(),
              )),
            );
          }
          _ => {}
        },
        "backgroundImage" => match value {
          Property::BackgroundImage(value) => {
            let mut repeat = None;
            if let Some(value) = properties.get("backgroundRepeat") {
              if let Property::BackgroundRepeat(repeat_value) = value {
                repeat = Some(repeat_value);
              }
            }
            let background_image = parse_background_image(value, repeat);
            let mut images = vec![];
            let mut linear_gradient = vec![];
            for item in background_image.0.iter() {
              if let BackgroundImageKind::String(_) = &item.image {
                images.push(item.clone());
              } else if let BackgroundImageKind::LinearGradient(gradient) = &item.image {
                linear_gradient.push(gradient.clone());
              }
            }
            if images.len() > 0 {
              final_properties.insert(
                id.to_string(),
                StyleValueType::BackgroundImage(BackgroundImage(images)),
              );
            }
            if linear_gradient.len() > 0 {
              final_properties.insert(
                "linearGradient".to_string(),
                StyleValueType::LinearGradient(LinearGradient(linear_gradient)),
              );
            }
          }
          _ => {}
        },
        "backgroundPosition" => match value {
          Property::BackgroundPosition(value) => {
            let background_position = parse_background_position(value);
            if background_position.0.len() > 0 {
              final_properties.insert(
                "backgroundImagePosition".to_string(),
                StyleValueType::BackgroundImagePosition(background_position),
              );
            }
          }
          _ => {}
        },
        "backgroundSize" => match value {
          Property::BackgroundSize(value) => {
            let background_size = parse_background_size(value);
            if background_size.0.len() > 0 {
              final_properties.insert(
                "backgroundImageSize".to_string(),
                StyleValueType::BackgroundImageSize(background_size),
              );
            }
          }
          _ => {}
        },
        "backgroundRepeat" => {}
        "flexDirection" => {
          flex_options.direction = match value {
            Property::FlexDirection(value, _) => Some(FlexDirection::from(value)),
            _ => None,
          }
        }
        "flexWrap" => {
          flex_options.wrap = match value {
            Property::FlexWrap(value, _) => Some(FlexWrap::from(value)),
            _ => None,
          }
        }
        "justifyContent" => {
          flex_options.justify_content = match value {
            Property::JustifyContent(value, _) => Some(FlexAlign::from(value)),
            _ => None,
          }
        }
        "alignItems" => {
          flex_options.align_items = match value {
            Property::AlignItems(value, _) => {
              let value = ItemAlign::from(value);
              if value == ItemAlign::Ignore {
                None
              } else {
                Some(value)
              }
            }
            _ => None,
          }
        }
        "alignContent" => {
          flex_options.align_content = match value {
            Property::AlignContent(value, _) => Some(FlexAlign::from(value)),
            _ => None,
          }
        }
        "flex" => {
          let mut flex_grow = None;
          let mut flex_shrink = None;
          let mut flex_basis = None;
          if let Property::Flex(value, _) = value {
            let flex_size = parse_flex_size(value);
            flex_grow = flex_size.grow;
            flex_shrink = flex_size.shrink;
            flex_basis = flex_size.basis;
          }
          if let Some(flex_grow) = flex_grow {
            final_properties.insert("flexGrow".to_string(), StyleValueType::FlexGrow(flex_grow));
          }
          if let Some(flex_shrink) = flex_shrink {
            final_properties.insert(
              "flexShrink".to_string(),
              StyleValueType::FlexShrink(flex_shrink),
            );
          }
          if let Some(flex_basis) = flex_basis {
            final_properties.insert(
              "flexBasis".to_string(),
              StyleValueType::FlexBasis(flex_basis),
            );
          }
        }
        "flexGrow" => {
          let mut flex_grow = None;
          if let Property::FlexGrow(value, _) = value {
            flex_grow = Some(*value);
          }
          if let Some(flex_grow) = flex_grow {
            final_properties.insert(
              id.to_string(),
              StyleValueType::FlexGrow(FlexGrow(flex_grow)),
            );
          }
        }
        "flexShrink" => {
          let mut flex_shrink = None;
          if let Property::FlexShrink(value, _) = value {
            flex_shrink = Some(*value);
          }
          if let Some(flex_shrink) = flex_shrink {
            final_properties.insert(
              id.to_string(),
              StyleValueType::FlexShrink(FlexShrink(flex_shrink)),
            );
          }
        }
        "flexBasis" => {
          let mut flex_basis = None;
          if let Property::FlexBasis(value, _) = value {
            flex_basis = Some(parse_flex_basis(value));
          }
          if let Some(flex_basis) = flex_basis {
            final_properties.insert(id.to_string(), StyleValueType::FlexBasis(flex_basis));
          }
        }
        "alignSelf" => {
          let mut align_self = None;
          if let Property::AlignSelf(value, _) = value {
            align_self = Some(ItemAlign::from(value));
          }
          if let Some(align_self) = align_self {
            if align_self != ItemAlign::Ignore {
              final_properties.insert(id.to_string(), StyleValueType::AlignSelf(align_self));
            }
          }
        }
        _ => {
          final_properties.insert(
            id.to_string(),
            StyleValueType::Normal(
              value
                .value_to_css_string(PrinterOptions {
                  minify: false,
                  targets: Targets {
                    include: Features::HexAlphaColors,
                    ..Targets::default()
                  },
                  ..PrinterOptions::default()
                })
                .unwrap(),
            ),
          );
        }
      }
    }
    if let Some(text_decoration) = text_decoration {
      match &text_decoration {
        Property::TextDecoration(decoration, _) => {
          final_properties.insert(
            "textDecoration".to_string(),
            StyleValueType::TextDecoration(TextDecoration {
              kind: decoration
                .line
                .to_css_string(PrinterOptions::default())
                .unwrap(),
              color: match color {
                Some(color) => to_camel_case(
                  color
                    .value_to_css_string(PrinterOptions::default())
                    .unwrap()
                    .as_str(),
                  true,
                ),
                None => "black".to_string(),
              },
            }),
          );
        }
        _ => {}
      }
    }

    final_properties.insert(
      "flexOptions".to_string(),
      StyleValueType::FlexOptions(flex_options),
    );

    final_properties
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let mut final_all_style = self.calc_style_record(&mut all_style);

    let mut final_all_style = final_all_style
      .iter_mut()
      .map(|(selector, style_value)| (selector.to_owned(), self.parse_style_style(style_value)))
      .collect::<HashMap<_, _>>();

    for (selector, style_value) in final_all_style.iter_mut() {
      let elements = self.document.select(selector);
      for element in elements {
        let declarations = style_record.entry(element.span).or_insert(vec![]);
        declarations.push(style_value.clone());
      }
    }
    let final_style_record = style_record
      .iter_mut()
      .map(|(selector, style_value)| {
        (
          *selector,
          style_value
            .iter_mut()
            .reduce(|a, b| {
              a.extend(b.drain());
              a
            })
            .unwrap()
            .to_owned(),
        )
      })
      .collect::<HashMap<_, _>>();
    StyleData {
      style_record: Rc::new(RefCell::new(final_style_record)),
      all_style: Rc::new(RefCell::new(final_all_style)),
    }
  }
}
