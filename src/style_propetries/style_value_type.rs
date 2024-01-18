use swc_ecma_ast::Expr;

use crate::generate_expr_based_on_platform;

use super::{traits::{ToExpr, ToStyleValue}, flex_align::FlexAlign, item_align::ItemAlign, aspect_ratio::AspactRatio, display::Display, flex_basis::FlexBasis, unit::{Platform, PropertyTuple}, normal::Normal, flex_direction::FlexDirection, flex_wrap::FlexWrap, gap::Gap, length_value::LengthValueProperty, size::SizeProperty, max_size::MaxSizeProperty, overflow::Overflow, number::NumberProperty, color::ColorProperty, font_size::FontSize, font_weight::FontWeight, line_height::LineHeight, text_align::TextAlign, text_decoration::TextDecoration};


#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(Normal),
  NumberProperty(NumberProperty),
  ColorProperty(ColorProperty),
  LengthValueProperty(LengthValueProperty),
  SizeProperty(SizeProperty),
  MaxSizeProperty(MaxSizeProperty),
  FlexAlign(FlexAlign),
  AlignItems(ItemAlign),
  FlexBasis(FlexBasis),
  FlexDirection(FlexDirection),
  FlexWrap(FlexWrap),
  AspectRatio(AspactRatio),
  Display(Display),
  Gap(Gap),
  Overflow(Overflow),
  FontSize(FontSize),
  FontWeight(FontWeight),
  LineHeight(LineHeight),
  TextAlign(TextAlign),
  TextDecoration(TextDecoration),
}

impl ToStyleValue for StyleValueType {
  fn to_expr(&self, platform: Platform) -> PropertyTuple {
    match self {
      StyleValueType::Normal(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::NumberProperty(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::ColorProperty(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::LengthValueProperty(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::SizeProperty(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::MaxSizeProperty(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FlexAlign(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::AlignItems(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FlexBasis(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FlexDirection(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FlexWrap(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::AspectRatio(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::Display(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::Gap(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::Overflow(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FontSize(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FontWeight(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::LineHeight(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::TextAlign(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::TextDecoration(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
    }
  }
}