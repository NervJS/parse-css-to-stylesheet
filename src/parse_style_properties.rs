
use lightningcss::{properties::{custom::TokenOrValue, Property}, stylesheet::PrinterOptions, traits::ToCss};
use swc_core::{common::DUMMY_SP, ecma::{ast::{self}, utils::quote_ident}};
use swc_core::ecma::ast::*;
use crate::style_propetries::style_value_type::{Variable};
use crate::{generate_expr_lit_str, style_parser::KeyFrameItem, style_propetries::{animation::Animation, animation_multi::AnimationMulti, aspect_ratio::AspectRatio, background::Background, background_image::BackgroundImage, background_position::BackgroundPosition, background_repeat::BackgroundRepeat, background_size::BackgroundSize, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, box_shadow::BoxShadow, color::ColorProperty, display::Display, expr::Expr, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, opacity::Opacity, overflow::Overflow, position::Position, size::SizeProperty, style_property_type::{string_to_css_property_type, CSSPropertyType}, style_value_type::StyleValueType, text_align::TextAlign, text_decoration::TextDecoration, text_overflow::TextOverflow, text_shadow::TextShadow, text_transform::TextTransform, transform::Transform, transform_origin::TransformOrigin, transition::Transition, unit::{generate_expr_by_length_value, Platform}, vertical_align::VerticalAlign, visibility::Visibility, white_space::WhiteSpace, word_break::WordBreak}, utils::lowercase_first};

#[derive(Debug, Clone)]
pub struct DeclsAndVars {
  pub decls:Vec<StyleValueType>,
  pub has_var: bool
}

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> DeclsAndVars{
  let mut final_properties = vec![];
  let mut has_env = false;
  for (id, value)  in properties.iter() {

    let mut is_env: bool = false;
    match value {
      Property::Unparsed(unparsed) => {
        unparsed.value.0.iter().for_each(|item| {
          match item {
            TokenOrValue::Env(env) => {
              is_env = true;
              let env_result = value.value_to_css_string(PrinterOptions::default());
              if (env_result.is_ok()) {
                final_properties.push(StyleValueType::Expr(Expr::new(string_to_css_property_type(id), generate_expr_lit_str!(env_result.unwrap().to_string()))));
              }
            },
            _ => {}
          }
        });
      },
      Property::Custom(custom) => {
        if (id.as_str().starts_with("--")) {
          final_properties.push(StyleValueType::Variable(Variable {
            id: custom.name.to_css_string(PrinterOptions::default()).unwrap(),
            value: value.value_to_css_string(PrinterOptions::default()).unwrap(),
          }));
        }
      }
      _ => {}
    };
    if is_env {
      has_env = true;
      continue;
    }

    let mut property_name = id.as_str();

    // 移除部分厂商前缀: Webkit, Moz, 并且把首字母小写
    if property_name.starts_with("Webkit") {
      property_name = &property_name[6..];
    } else if property_name.starts_with("Moz") {
      property_name = &property_name[3..];
    }

    // 将property_name首字母小写
    let mut property_name = property_name.to_string();
    lowercase_first(&mut property_name);


    match property_name.as_str() {
          // 基础样式
          "alignContent" => {
            final_properties.push(StyleValueType::FlexAlign(FlexAlign::from((id.to_string(), value))));
          }
          "justifyContent" => {
            final_properties.push(StyleValueType::FlexAlign(FlexAlign::from((id.to_string(), value))));
          }
          "alignItems" => {
            final_properties.push(StyleValueType::AlignItems(ItemAlign::from((id.to_string(), value))));
          }
          "alignSelf" => {
            final_properties.push(StyleValueType::AlignItems(ItemAlign::from((id.to_string(), value))));
          }
          "flex" => {
            final_properties.push(StyleValueType::Flex(Flex::from((id.to_string(), value))));
          }
          "flexBasis" => {
            final_properties.push(StyleValueType::FlexBasis(FlexBasis::from((id.to_string(), value))));
          }
          "flexDirection" => {
            final_properties.push(StyleValueType::FlexDirection(FlexDirection::from((id.to_string(), value))));
          }
          "flexGrow" => {
            final_properties.push(StyleValueType::NumberProperty(NumberProperty::from((id.to_string(), value))));
          }
          "flexShrink" => {
            final_properties.push(StyleValueType::NumberProperty(NumberProperty::from((id.to_string(), value))));
          }
          "flexWrap" => {
            final_properties.push(StyleValueType::FlexWrap(FlexWrap::from((id.to_string(), value))));
          }
          "aspectRatio" => {
            final_properties.push(StyleValueType::AspectRatio(AspectRatio::from((id.to_string(), value))));
          }
          "display" => {
            final_properties.push(StyleValueType::Display(Display::from((id.to_string(), value))));
          }
          "gap" | "columnGap" | "rowGap" => {
            final_properties.push(StyleValueType::Gap(Gap::from((id.to_string(), value))));
          }
          "margin" | "padding"  => {
            final_properties.push(StyleValueType::MarginPadding(MarginPadding::from((id.to_string(), value))));
          }
          "marginTop" | "marginBottom" | "marginLeft" | "marginRight" | "paddingTop" | "paddingBottom" | "paddingLeft" | "paddingRight" | "top" | "bottom" | "left" | "right" => {
            final_properties.push(StyleValueType::LengthValueProperty(LengthValueProperty::from((id.to_string(), value))));
          }
          "maxHeight" | "maxWidth" => {
            final_properties.push(StyleValueType::MaxSizeProperty(MaxSizeProperty::from((id.to_string(), value))));
          }
          "height" | "width" | "minHeight" | "minWidth" => {
            final_properties.push(StyleValueType::SizeProperty(SizeProperty::from((id.to_string(), value))));
          }
          "overflow" => {
            final_properties.push(StyleValueType::Overflow(Overflow::from((id.to_string(), value))));
          }
          "color" | "backgroundColor" => {
            final_properties.push(StyleValueType::ColorProperty(ColorProperty::from((id.to_string(), value))));
          }
          // 文本样式
          "fontSize" => {
            final_properties.push(StyleValueType::FontSize(FontSize::from((id.to_string(), value))));
          }
          "fontStyle" => {
            final_properties.push(StyleValueType::FontStyle(FontStyle::from((id.to_string(), value))));
          }
          "fontWeight" => {
            final_properties.push(StyleValueType::FontWeight(FontWeight::from((id.to_string(), value))));
          }
          "lineHeight" => {
            final_properties.push(StyleValueType::LineHeight(LineHeight::from((id.to_string(), value))));
          }
          "textAlign" => {
            final_properties.push(StyleValueType::TextAlign(TextAlign::from((id.to_string(), value))));
          }
          "textDecoration" => {
            // textDecorationLine、textDecorationColor、textDecorationStyle
            final_properties.push(StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
          }
          "textDecorationLine" => {
            final_properties.push(StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
          }
          "textDecorationColor" => {
            final_properties.push(StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
          }
          "textDecorationStyle" => {
            final_properties.push(StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
          }
          "textShadow" => {
            final_properties.push(StyleValueType::TextShadow(TextShadow::from((id.to_string(), value))));
          }
          "textTransform" => {
            final_properties.push(StyleValueType::TextTransform(TextTransform::from((id.to_string(), value))));
          }
          "textOverflow" => {
            final_properties.push(StyleValueType::TextOverflow(TextOverflow::from((id.to_string(), value))));
          }
          "letterSpacing" => {
            final_properties.push(StyleValueType::LetterSpacing(LetterSpacing::from((id.to_string(), value))));
          }
          "verticalAlign" => {
            final_properties.push(StyleValueType::VerticalAlign(VerticalAlign::from((id.to_string(), value))));
          }
          // 边框
          "borderColor" => {
            final_properties.push(StyleValueType::BorderColor(BorderColor::from((id.to_string(), value))));
          }
          "borderTopColor" | "borderBottomColor" | "borderLeftColor" | "borderRightColor" => {
            final_properties.push(StyleValueType::BorderColor(BorderColor::from((id.to_string(), value))));
          }
          "borderWidth" => {
            final_properties.push(StyleValueType::BorderWidth(BorderWidth::from((id.to_string(), value))));
          }
          "borderTopWidth" | "borderBottomWidth" | "borderLeftWidth" | "borderRightWidth" => {
            final_properties.push(StyleValueType::BorderWidth(BorderWidth::from((id.to_string(), value))));
          }
          "borderRadius" => {
            final_properties.push(StyleValueType::BorderRadius(BorderRadius::from((id.to_string(), value))));
          }
          "borderTopLeftRadius" | "borderTopRightRadius" | "borderBottomLeftRadius" | "borderBottomRightRadius" => {
            final_properties.push(StyleValueType::BorderRadius(BorderRadius::from((id.to_string(), value))));
          }
          "borderStyle" => {
            final_properties.push(StyleValueType::BorderStyle(BorderStyle::from((id.to_string(), value))));
          }
          "borderTopStyle" | "borderBottomStyle" | "borderLeftStyle" | "borderRightStyle" => {
            final_properties.push(StyleValueType::BorderStyle(BorderStyle::from((id.to_string(), value))));
          }
          "border" => {
            final_properties.push(StyleValueType::Border(Border::from((id.to_string(), value))));
          }
          "borderTop" | "borderBottom" | "borderLeft" | "borderRight" => {
            final_properties.push(StyleValueType::Border(Border::from((id.to_string(), value))));
          }
          // 变换
          "transform" => {
            final_properties.push(StyleValueType::Transform(Transform::from((id.to_string(), value))));
          }
          "transformOrigin" => {
            final_properties.push(StyleValueType::TransformOrigin(TransformOrigin::from((id.to_string(), value))));
          }
          // 背景
          "backgroundRepeat" => {
            final_properties.push(StyleValueType::BackgroundRepeat(BackgroundRepeat::from((id.to_string(), value))));
          }
          "backgroundPosition" => {
            final_properties.push(StyleValueType::BackgroundPosition(BackgroundPosition::from((id.to_string(), value))));
          }
          "backgroundSize" => {
            final_properties.push(StyleValueType::BackgroundSize(BackgroundSize::from((id.to_string(), value))));
          }
          "backgroundImage" => {
            final_properties.push(StyleValueType::BackgroundImage(BackgroundImage::from((id.to_string(), value))));
          }
          "background" => {
            final_properties.push(StyleValueType::Background(Background::from((id.to_string(), value))));
          }
          "boxShadow" => {
            final_properties.push(StyleValueType::BoxShadow(BoxShadow::from((id.to_string(), value))));
          }
          "position" => {
            final_properties.push(StyleValueType::Position(Position::from((id.to_string(), value))));
          }
          "visibility" => {
            final_properties.push(StyleValueType::Visibility(Visibility::from((id.to_string(), value))));
          }
          "opacity" => {
            final_properties.push(StyleValueType::Opacity(Opacity::from((id.to_string(), value))));
          }
          "content" => {
            // 判断content内容是否是空字符串
            let content_value = value.value_to_css_string(PrinterOptions::default()).unwrap().trim().to_string();
            if content_value != "\"\"" {
              // 替换字符串，将左右两边的"干掉
              let content_value = content_value.trim_matches('"');
              final_properties.push(StyleValueType::Normal(Normal::new(CSSPropertyType::Content, content_value.to_string())));
            }
          }
          "animation" | "animationName" => {
            // if let Some(ref keyframes_map) = keyframes_map {
            //   final_properties.push(StyleValueType::Animation(Animation::from((id.to_string(), value, Some(keyframes_map.clone())))));
            // }
            final_properties.push(StyleValueType::AnimationMulti(AnimationMulti::from((id.to_string(), value))));
          }
          "animationDelay" | "animationDuration" | "animationIterationCount" | "animationTimingFunction" | "animationFillMode" => {
            // final_properties.push(StyleValueType::Animation(Animation::from((id.to_string(), value, None))));
            final_properties.push(StyleValueType::AnimationMulti(AnimationMulti::from((id.to_string(), value))));
          }
          "transition" | "transitionProperty" | "transitionDuration" | "transitionDelay" | "transitionTimingFunction" => {
            final_properties.push(StyleValueType::Transition(Transition::from((id.to_string(), value))));
          }
          "zIndex" => {
            final_properties.push(StyleValueType::Normal(Normal::new(CSSPropertyType::ZIndex, value.value_to_css_string(PrinterOptions::default()).unwrap())));
          }
          "lineClamp" => {
            final_properties.push(StyleValueType::Normal(Normal::new(CSSPropertyType::WebkitLineClamp, value.value_to_css_string(PrinterOptions::default()).unwrap())));
          }
          "wordBreak" => {
            final_properties.push(StyleValueType::WordBreak(WordBreak::from(( id.to_string(), value ))))
          }
          "whiteSpace" => {
            final_properties.push(StyleValueType::WhiteSpace(WhiteSpace::from(( id.to_string(), value ))))
          }
          _ => {
            // position、zIndex等... 会自动处理 单位、数字等相关信息
            // final_properties.push(StyleValueType::Normal(Normal::new(id.to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap())));
          }
        }
}

  DeclsAndVars {
    decls: final_properties,
    has_var: has_env
  }
}