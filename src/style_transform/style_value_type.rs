use swc_ecma_ast::Expr;

use super::{
  background::{background::Background, linear_gradient::LinearGradient},
  flex_options::{flex_options::FlexOptions, item_align::ItemAlign},
  flex_size::{flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink},
  border::{border_color::BorderColor, border_width::BorderWidth, border_radius::BorderRadius, border_style::BorderStyle},
  margin_padding::MarginPadding,
  constraint_size::ConstraintSize,
  text_decoration::TextDecoration,
  traits::ToExpr,
  transform::{Matrices, Rotates, Scales, Translates},
  text::{line_height::LineHeight, letter_spacing::LetterSpacing, text_align::TextAlign, text_overflow::TextOverflow, font_weight::FontWeight},
};

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(String),
  // Number(f32),
  TextDecoration(TextDecoration),
  BorderRadius(BorderRadius),
  MarginPadding(MarginPadding),
  Background(Background),
  LinearGradient(LinearGradient),
  FlexOptions(FlexOptions),
  AlignSelf(ItemAlign),
  FlexGrow(FlexGrow),
  FlexShrink(FlexShrink),
  FlexBasis(FlexBasis),
  Translates(Translates),
  Rotates(Rotates),
  Scales(Scales),
  Matrices(Matrices),
  ConstraintSize(ConstraintSize),
  BorderColor(BorderColor),
  BorderWidth(BorderWidth),
  BorderStyle(BorderStyle),
  LineHeight(LineHeight),
  LetterSpacing(LetterSpacing),
  TextAlign(TextAlign),
  TextOverflow(TextOverflow),
  FontWeight(FontWeight)
}

impl ToExpr for StyleValueType {
  fn to_expr(&self) -> Expr {
    match self {
      StyleValueType::Normal(value) => value.to_string().into(),
      // StyleValueType::Number(num) => (*num as f64).into(),
      StyleValueType::TextDecoration(text_decoration) => text_decoration.to_expr().into(),
      StyleValueType::BorderRadius(border_radius) => border_radius.to_expr().into(),
      StyleValueType::MarginPadding(margin_padding) => margin_padding.to_expr().into(),
      StyleValueType::Background(background) => background.to_expr().into(),
      StyleValueType::LinearGradient(linear_gradient) => linear_gradient.to_expr().into(),
      StyleValueType::FlexOptions(flex_options) => flex_options.to_expr().into(),
      StyleValueType::AlignSelf(align_self) => align_self.to_expr().into(),
      StyleValueType::FlexGrow(flex_grow) => flex_grow.to_expr().into(),
      StyleValueType::FlexShrink(flex_shrink) => flex_shrink.to_expr().into(),
      StyleValueType::FlexBasis(flex_basis) => flex_basis.to_expr().into(),
      StyleValueType::Translates(translates) => translates.to_expr().into(),
      StyleValueType::Rotates(rotates) => rotates.to_expr().into(),
      StyleValueType::Scales(scales) => scales.to_expr().into(),
      StyleValueType::Matrices(matrices) => matrices.to_expr().into(),
      StyleValueType::ConstraintSize(constraint_size) => constraint_size.to_expr().into(),
      StyleValueType::BorderColor(border_color) => border_color.to_expr().into(),
      StyleValueType::BorderWidth(border_width) => border_width.to_expr().into(),
      StyleValueType::BorderStyle(border_style) => border_style.to_expr().into(),
      StyleValueType::LineHeight(line_height) => line_height.to_expr().into(),
      StyleValueType::LetterSpacing(letter_spacing) => letter_spacing.to_expr().into(),
      StyleValueType::TextAlign(text_align) => text_align.to_expr().into(),
      StyleValueType::TextOverflow(text_overflow) => text_overflow.to_expr().into(),
      StyleValueType::FontWeight(font_weight) => font_weight.to_expr().into(),
    }
  }
}
