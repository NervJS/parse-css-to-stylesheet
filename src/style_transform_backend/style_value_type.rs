use swc_ecma_ast::Expr;

use crate::utils::{convert_px_to_units, fix_rgba};

use super::{
  background::{linear_gradient::LinearGradient, background_position::BackgroundPosition, background_size::BackgroundSize, background_repeat::BackgroundRepeat, background_image::BackgroundImage, background_color::BackgroundColor},
  flex_options::{item_align::ItemAlign, flex_direction::FlexDirection, flex_wrap::FlexWrap, flex_align::FlexAlign},
  flex_size::{flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink},
  border::border_style::BorderStyleType,
  traits::ToExpr,
  transform::{transform::Transform, transform_origin::TransformOrigin},
  text::{line_height::LineHeight, letter_spacing::LetterSpacing, text_align::TextAlign, text_overflow::TextOverflow, font_weight::FontWeight, font_style::FontStyle, text_decoration::TextDecoration},
};

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Length(String),
  Normal(String),
  Color(String),
  // Number(f32),
  Px(String),
  BorderStyleType(BorderStyleType),
  TextDecoration(TextDecoration),
  // Background(Background),
  BackgroundPosition(BackgroundPosition),
  BackgroundSize(BackgroundSize),
  BackgroundRepeat(BackgroundRepeat),
  BackgroundImage(BackgroundImage),
  BackgroundColor(BackgroundColor),
  LinearGradient(LinearGradient),
  FlexDirection(FlexDirection),
  FlexWrap(FlexWrap),
  JustifyContent(FlexAlign),
  ItemAlign(ItemAlign),
  AlignContent(FlexAlign),
  AlignSelf(ItemAlign),
  FlexGrow(FlexGrow),
  FlexShrink(FlexShrink),
  FlexBasis(FlexBasis),
  Transform(Transform),
  TransformOrigin(TransformOrigin),
  LineHeight(LineHeight),
  LetterSpacing(LetterSpacing),
  TextAlign(TextAlign),
  TextOverflow(TextOverflow),
  FontWeight(FontWeight),
  FontStyle(FontStyle)
}

impl ToExpr for StyleValueType {
  fn to_expr(&self) -> Expr {
    match self {
      StyleValueType::Length(value) => convert_px_to_units(value.to_string()).into(),
      StyleValueType::Normal(value) => {
        if let Ok(number) = value.parse::<f64>() {
          return number.into()
        }
        value.to_string().into()
      },
      StyleValueType::Color(value) => fix_rgba(&value).into(),
      // StyleValueType::Number(num) => (*num as f64).into(),
      StyleValueType::Px(value) => convert_px_to_units(value.to_string()).into(),
      StyleValueType::BorderStyleType(border_style_type) => border_style_type.to_expr().into(),
      StyleValueType::TextDecoration(text_decoration) => text_decoration.to_expr().into(),
      StyleValueType::BackgroundPosition(background_position) => background_position.to_expr().into(),
      StyleValueType::BackgroundSize(background_size) => background_size.to_expr().into(),
      StyleValueType::BackgroundRepeat(background_repeat) => background_repeat.to_expr().into(),
      StyleValueType::BackgroundColor(background_color) => background_color.to_expr().into(),
      StyleValueType::BackgroundImage(background_image) => background_image.to_expr().into(),
      StyleValueType::LinearGradient(linear_gradient) => linear_gradient.to_expr().into(),
      StyleValueType::FlexDirection(flex_direction) => flex_direction.to_expr().into(),
      StyleValueType::FlexWrap(flex_wrap) => flex_wrap.to_expr().into(),
      StyleValueType::JustifyContent(justify_content) => justify_content.to_expr().into(),
      StyleValueType::ItemAlign(item_align) => item_align.to_expr().into(),
      StyleValueType::AlignContent(align_content) => align_content.to_expr().into(),
      StyleValueType::AlignSelf(align_self) => align_self.to_expr().into(),
      StyleValueType::FlexGrow(flex_grow) => flex_grow.to_expr().into(),
      StyleValueType::FlexShrink(flex_shrink) => flex_shrink.to_expr().into(),
      StyleValueType::FlexBasis(flex_basis) => flex_basis.to_expr().into(),
      StyleValueType::Transform(transform) => transform.to_expr().into(),
      StyleValueType::TransformOrigin(transform_origin) => transform_origin.to_expr().into(),
      StyleValueType::LineHeight(line_height) => line_height.to_expr().into(),
      StyleValueType::LetterSpacing(letter_spacing) => letter_spacing.to_expr().into(),
      StyleValueType::TextAlign(text_align) => text_align.to_expr().into(),
      StyleValueType::TextOverflow(text_overflow) => text_overflow.to_expr().into(),
      StyleValueType::FontWeight(font_weight) => font_weight.to_expr().into(),
      StyleValueType::FontStyle(font_style) => font_style.to_expr().into(),
    }
  }
}
