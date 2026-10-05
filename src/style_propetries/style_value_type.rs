use crate::generate_expr_based_on_platform;

#[derive(Debug, Clone)]
pub struct CssVariable {
  pub id: String,
  pub value: String
}

use super::{
  animation::Animation, animation_multi::AnimationMulti, aspect_ratio::AspectRatio, background::Background, background_image::BackgroundImage, background_position::BackgroundPosition, background_repeat::BackgroundRepeat, background_size::BackgroundSize, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, box_orient::BoxOrient, box_shadow::BoxShadow, color::ColorProperty, display::Display, expr::Expr, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, grid_auto_flow::GridAutoFlow, grid_placement::GridPlacement, grid_template::GridTemplate, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, opacity::Opacity, overflow::Overflow, pointer_events::PointerEvents, position::Position, size::SizeProperty, text_align::TextAlign, text_decoration::TextDecoration, text_overflow::TextOverflow, text_shadow::TextShadow, text_transform::TextTransform, traits::{ToExpr, ToStyleValue}, transform::Transform, transform_origin::TransformOrigin, transition::Transition, unit::{Platform, PropertyTuple}, variable::Variable, vertical_align::VerticalAlign, visibility::Visibility, white_space::WhiteSpace, word_break::WordBreak
};

// StyleValueType 派发变体表：variant 名 → 包裹的 payload 类型。
// 语义：每个 variant 都是用 payload 类型包裹一个 From 产物。
// 用途：生成 StyleValueType enum 和 ToStyleValue::to_expr match（去 55 臂重复）。
// 注：Normal/Expr/Variable 不在此表，它们不是 From 派生的，保留手写。
macro_rules! for_each_style_value_type {
  ($m:ident) => {
    $m! {
      // 顺序 = 原来在 StyleValueType 中出现的顺序（to_expr 顺序无关，仅列表对齐）
      NumberProperty(NumberProperty);
      ColorProperty(ColorProperty);
      LengthValueProperty(LengthValueProperty);
      SizeProperty(SizeProperty);
      MaxSizeProperty(MaxSizeProperty);
      MarginPadding(MarginPadding);
      FlexAlign(FlexAlign);
      AlignItems(ItemAlign);
      Flex(Flex);
      FlexBasis(FlexBasis);
      FlexDirection(FlexDirection);
      FlexWrap(FlexWrap);
      AspectRatio(AspectRatio);
      Display(Display);
      Gap(Gap);
      Overflow(Overflow);
      FontSize(FontSize);
      FontStyle(FontStyle);
      FontWeight(FontWeight);
      LineHeight(LineHeight);
      TextAlign(TextAlign);
      TextDecoration(TextDecoration);
      TextShadow(TextShadow);
      TextTransform(TextTransform);
      TextOverflow(TextOverflow);
      LetterSpacing(LetterSpacing);
      VerticalAlign(VerticalAlign);
      BorderColor(BorderColor);
      BorderWidth(BorderWidth);
      BorderRadius(BorderRadius);
      BorderStyle(BorderStyle);
      Border(Border);
      Transform(Transform);
      TransformOrigin(TransformOrigin);
      BackgroundRepeat(BackgroundRepeat);
      BackgroundPosition(BackgroundPosition);
      BackgroundSize(BackgroundSize);
      BackgroundImage(BackgroundImage);
      Background(Background);
      Animation(Animation);
      AnimationMulti(AnimationMulti);
      Transition(Transition);
      BoxShadow(BoxShadow);
      Position(Position);
      Visibility(Visibility);
      Opacity(Opacity);
      WordBreak(WordBreak);
      WhiteSpace(WhiteSpace);
      BoxOrient(BoxOrient);
      PointerEvents(PointerEvents);
      GridTemplate(GridTemplate);
      GridPlacement(GridPlacement);
      GridAutoFlow(GridAutoFlow);
    }
  };
}

// ---------------------------------------------------------------------------
// 生成 StyleValueType enum（含手写 Normal/Expr/Variable 变体）
// 展开后产生：
//   pub enum StyleValueType {
//     Normal(Normal),
//     Expr(Expr),
//     Variable(Variable),
//     NumberProperty(NumberProperty),
//     ... 52 个来自表
//   }
// ---------------------------------------------------------------------------
macro_rules! __define_style_value_type {
  ($($variant:ident($payload:ty);)*) => {
    #[derive(Debug, Clone)]
    pub enum StyleValueType {
      Normal(Normal),
      Expr(Expr),
      Variable(Variable),
      $($variant($payload),)*
    }
  };
}

for_each_style_value_type!(__define_style_value_type);

// ---------------------------------------------------------------------------
// 生成 ToStyleValue::to_expr 派发 match
// 展开后产生：
//   StyleValueType::NumberProperty(value) => generate_expr_based_on_platform!(platform, value),
//   ... 等等
// ---------------------------------------------------------------------------
macro_rules! __dispatch_style_value_type_to_expr {
  ($($variant:ident($payload:ty);)*) => {
    impl ToStyleValue for StyleValueType {
      fn to_expr(&self, platform: Platform) -> PropertyTuple {
        match self {
          StyleValueType::Normal(value) => generate_expr_based_on_platform!(platform, value),
          StyleValueType::Expr(value) => value.to_expr_for_platform(platform),
          StyleValueType::Variable(value) => generate_expr_based_on_platform!(platform, value),
          $(
            StyleValueType::$variant(value) => generate_expr_based_on_platform!(platform, value),
          )*
        }
      }
    }
  };
}

for_each_style_value_type!(__dispatch_style_value_type_to_expr);
