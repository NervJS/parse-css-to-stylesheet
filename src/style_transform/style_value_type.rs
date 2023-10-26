use swc_ecma_ast::Expr;

use super::{
  background::{background::Background, linear_gradient::LinearGradient},
  border_radius::BorderRadius,
  flex_options::{flex_options::FlexOptions, item_align::ItemAlign},
  flex_size::{flex_basis::FlexBasis, flex_grow::FlexGrow, flex_shrink::FlexShrink},
  margin_padding::MarginPadding,
  text_decoration::TextDecoration,
  traits::ToExpr,
  transform::{Matrices, Rotates, Scales, Translates},
};

#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(String),
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
}

impl ToExpr for StyleValueType {
  fn to_expr(&self) -> Expr {
    match self {
      StyleValueType::Normal(value) => value.to_string().into(),
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
    }
  }
}
