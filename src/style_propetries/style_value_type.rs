
use crate::generate_expr_based_on_platform;

use super::{animation::Animation, transition::Transition, aspect_ratio::AspectRatio, background::Background, background_image::BackgroundImage, background_position::BackgroundPosition, background_repeat::BackgroundRepeat, background_size::BackgroundSize, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, box_shadow::BoxShadow, color::ColorProperty, display::Display, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, opacity::Opacity, overflow::Overflow, position::Position, size::SizeProperty, text_align::TextAlign, text_decoration::TextDecoration, text_overflow::TextOverflow, text_shadow::TextShadow, text_transform::TextTransform, traits::{ToExpr, ToStyleValue}, transform::Transform, transform_origin::TransformOrigin, unit::{Platform, PropertyTuple}, vertical_align::VerticalAlign, visibility::Visibility, word_break::WordBreak};


#[derive(Debug, Clone)]
pub enum StyleValueType {
  Normal(Normal),
  // Expr(Expr),
  NumberProperty(NumberProperty),
  ColorProperty(ColorProperty),
  LengthValueProperty(LengthValueProperty),
  SizeProperty(SizeProperty),
  MaxSizeProperty(MaxSizeProperty),
  MarginPadding(MarginPadding),
  FlexAlign(FlexAlign),
  AlignItems(ItemAlign),
  Flex(Flex),
  FlexBasis(FlexBasis),
  FlexDirection(FlexDirection),
  FlexWrap(FlexWrap),
  AspectRatio(AspectRatio),
  Display(Display),
  Gap(Gap),
  Overflow(Overflow),
  FontSize(FontSize),
  FontStyle(FontStyle),
  FontWeight(FontWeight),
  LineHeight(LineHeight),
  TextAlign(TextAlign),
  TextDecoration(TextDecoration),
  TextShadow(TextShadow),
  TextTransform(TextTransform),
  TextOverflow(TextOverflow),
  LetterSpacing(LetterSpacing),
  VerticalAlign(VerticalAlign),
  BorderColor(BorderColor),
  BorderWidth(BorderWidth),
  BorderRadius(BorderRadius),
  BorderStyle(BorderStyle),
  Border(Border),
  Transform(Transform),
  TransformOrigin(TransformOrigin),
  BackgroundRepeat(BackgroundRepeat),
  BackgroundPosition(BackgroundPosition),
  BackgroundSize(BackgroundSize),
  BackgroundImage(BackgroundImage),
  Background(Background),
  Animation(Animation),
  Transition(Transition),
  BoxShadow(BoxShadow),
  Position(Position),
  Visibility(Visibility),
  Opacity(Opacity),
  WordBreak(WordBreak)
}

impl ToStyleValue for StyleValueType {
  fn to_expr(&self, platform: Platform) -> PropertyTuple {
    match self {
      StyleValueType::Normal(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      // StyleValueType::Expr(value) => {
      //   generate_expr_based_on_platform!(platform, value)
      // },
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
      StyleValueType::MarginPadding(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::Flex(value) => {
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
      StyleValueType::FontStyle(value) => {
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
      StyleValueType::TextShadow(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::TextOverflow(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::TextTransform(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::LetterSpacing(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::VerticalAlign(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::BorderColor(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::BorderWidth(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::BorderRadius(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::BorderStyle(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Border(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Transform(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::TransformOrigin(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::BackgroundRepeat(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::BackgroundPosition(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::BackgroundSize(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::BackgroundImage(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Background(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Animation(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Transition(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::BoxShadow(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Position(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Visibility(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::Opacity(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
      StyleValueType::WordBreak(value) => {
        generate_expr_based_on_platform!(platform, value)
      }
    }
  }
}