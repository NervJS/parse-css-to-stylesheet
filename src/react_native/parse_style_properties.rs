use lightningcss::{properties::Property, stylesheet::PrinterOptions};

use crate::style_propetries::{aspect_ratio::AspactRatio, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, color::ColorProperty, display::Display, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, overflow::Overflow, size::SizeProperty, style_value_type::StyleValueType, text_align::TextAlign, text_decoration::TextDecoration, text_shadow::TextShadow, text_transform::TextTransform, transform::Transform, transform_origin::TransformOrigin, vertical_align::VerticalAlign};

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> Vec<StyleValueType> {
  let mut final_properties = vec![];

  for (id, value)  in properties.iter() {
    let property_name = id.as_str();
    match property_name {
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
        final_properties.push(StyleValueType::AspectRatio(AspactRatio::from((id.to_string(), value))));
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
      _ => {
        // position、zIndex等... 会自动处理 单位、数字等相关信息
        final_properties.push(StyleValueType::Normal(Normal::new(id.to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap())));
      }
    }
  }
  final_properties
}