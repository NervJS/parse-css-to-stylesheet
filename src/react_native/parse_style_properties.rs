use std::collections::HashMap;

use lightningcss::{properties::Property, stylesheet::PrinterOptions};

use crate::style_propetries::{aspect_ratio::AspactRatio, border::Border, border_color::BorderColor, border_radius::BorderRadius, border_style::BorderStyle, border_width::BorderWidth, color::ColorProperty, display::Display, flex::Flex, flex_align::FlexAlign, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, font_size::FontSize, font_style::FontStyle, font_weight::FontWeight, gap::Gap, item_align::ItemAlign, length_value::LengthValueProperty, letter_spacing::LetterSpacing, line_height::LineHeight, marin_padding::MarginPadding, max_size::MaxSizeProperty, normal::Normal, number::NumberProperty, overflow::Overflow, size::SizeProperty, style_value_type::StyleValueType, text_align::TextAlign, text_decoration::TextDecoration, text_shadow::TextShadow, text_transform::TextTransform, transform::Transform, vertical_align::VerticalAlign};

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> HashMap<String, StyleValueType> {
  let mut final_properties = HashMap::new();

  for (id, value)  in properties.iter() {
    let property_name = id.as_str();
    match property_name {
      // 基础样式
      "alignContent" => {
        final_properties.insert("alignContent".to_string(), StyleValueType::FlexAlign(FlexAlign::from((id.to_string(), value))));
      }
      "justifyContent" => {
        final_properties.insert("justifyContent".to_string(), StyleValueType::FlexAlign(FlexAlign::from((id.to_string(), value))));
      }
      "alignItems" => {
        final_properties.insert("alignItems".to_string(), StyleValueType::AlignItems(ItemAlign::from((id.to_string(), value))));
      }
      "alignSelf" => {
        final_properties.insert("alignSelf".to_string(), StyleValueType::AlignItems(ItemAlign::from((id.to_string(), value))));
      }
      "flex" => {
        final_properties.insert("flexBasis".to_string(), StyleValueType::Flex(Flex::from((id.to_string(), value))));
      }
      "flexBasis" => {
        final_properties.insert("flexBasis".to_string(), StyleValueType::FlexBasis(FlexBasis::from((id.to_string(), value))));
      }
      "flexDirection" => {
        final_properties.insert("flexDirection".to_string(), StyleValueType::FlexDirection(FlexDirection::from((id.to_string(), value))));
      }
      "flexGrow" => {
        final_properties.insert("flexGrow".to_string(), StyleValueType::NumberProperty(NumberProperty::from((id.to_string(), value))));
      }
      "flexShrink" => {
        final_properties.insert("flexShrink".to_string(), StyleValueType::NumberProperty(NumberProperty::from((id.to_string(), value))));
      }
      "flexWrap" => {
        final_properties.insert("flexWrap".to_string(), StyleValueType::FlexWrap(FlexWrap::from((id.to_string(), value))));
      }
      "aspectRatio" => {
        final_properties.insert("aspectRatio".to_string(), StyleValueType::AspectRatio(AspactRatio::from((id.to_string(), value))));
      }
      "display" => {
        final_properties.insert("display".to_string(), StyleValueType::Display(Display::from((id.to_string(), value))));
      }
      "gap" | "columnGap" | "rowGap" => {
        final_properties.insert(id.to_string(), StyleValueType::Gap(Gap::from((id.to_string(), value))));
      }
      "margin" | "padding"  => {
        final_properties.insert(id.to_string(), StyleValueType::MarginPadding(MarginPadding::from((id.to_string(), value))));
      }
      "marginTop" | "marginBottom" | "marginLeft" | "marginRight" | "paddingTop" | "paddingBottom" | "paddingLeft" | "paddingRight" | "top" | "bottom" | "left" | "right" => {
        final_properties.insert(id.to_string(), StyleValueType::LengthValueProperty(LengthValueProperty::from((id.to_string(), value))));
      }
      "maxHeight" | "maxWidth" => {
        final_properties.insert(id.to_string(), StyleValueType::MaxSizeProperty(MaxSizeProperty::from((id.to_string(), value))));
      }
      "height" | "width" | "minHeight" | "minWidth" => {
        final_properties.insert(id.to_string(), StyleValueType::SizeProperty(SizeProperty::from((id.to_string(), value))));
      }
      "overflow" => {
        final_properties.insert("overflow".to_string(), StyleValueType::Overflow(Overflow::from((id.to_string(), value))));
      }
      // 文本样式
      "color" => {
        final_properties.insert("color".to_string(), StyleValueType::ColorProperty(ColorProperty::from((id.to_string(), value))));
      }
      "fontSize" => {
        final_properties.insert("fontSize".to_string(), StyleValueType::FontSize(FontSize::from((id.to_string(), value))));
      }
      "fontStyle" => {
        final_properties.insert("fontStyle".to_string(), StyleValueType::FontStyle(FontStyle::from((id.to_string(), value))));
      }
      "fontWeight" => {
        final_properties.insert("fontWeight".to_string(), StyleValueType::FontWeight(FontWeight::from((id.to_string(), value))));
      }
      "lineHeight" => {
        final_properties.insert("lineHeight".to_string(), StyleValueType::LineHeight(LineHeight::from((id.to_string(), value))));
      }
      "textAlign" => {
        final_properties.insert("textAlign".to_string(), StyleValueType::TextAlign(TextAlign::from((id.to_string(), value))));
      }
      "textDecoration" => {
        // textDecorationLine、textDecorationColor、textDecorationStyle
        final_properties.insert("textDecoration".to_string(), StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
      }
      "textDecorationLine" => {
        final_properties.insert("textDecorationLine".to_string(), StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
      }
      "textDecorationColor" => {
        final_properties.insert("textDecorationColor".to_string(), StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
      }
      "textDecorationStyle" => {
        final_properties.insert("textDecorationStyle".to_string(), StyleValueType::TextDecoration(TextDecoration::from((id.to_string(), value))));
      }
      "textShadow" => {
        final_properties.insert("textShadow".to_string(), StyleValueType::TextShadow(TextShadow::from((id.to_string(), value))));
      }
      "textTransform" => {
        final_properties.insert("textTransform".to_string(), StyleValueType::TextTransform(TextTransform::from((id.to_string(), value))));
      }
      "letterSpacing" => {
        final_properties.insert("letterSpacing".to_string(), StyleValueType::LetterSpacing(LetterSpacing::from((id.to_string(), value))));
      }
      "verticalAlign" => {
        final_properties.insert("verticalAlign".to_string(), StyleValueType::VerticalAlign(VerticalAlign::from((id.to_string(), value))));
      }
      "borderColor" => {
        final_properties.insert("borderColor".to_string(), StyleValueType::BorderColor(BorderColor::from((id.to_string(), value))));
      }
      "borderTopColor" | "borderBottomColor" | "borderLeftColor" | "borderRightColor" => {
        final_properties.insert(property_name.to_string(), StyleValueType::BorderColor(BorderColor::from((id.to_string(), value))));
      }
      "borderWidth" => {
        final_properties.insert("borderWidth".to_string(), StyleValueType::BorderWidth(BorderWidth::from((id.to_string(), value))));
      }
      "borderTopWidth" | "borderBottomWidth" | "borderLeftWidth" | "borderRightWidth" => {
        final_properties.insert(property_name.to_string(), StyleValueType::BorderWidth(BorderWidth::from((id.to_string(), value))));
      }
      "borderRadius" => {
        final_properties.insert("borderRadius".to_string(), StyleValueType::BorderRadius(BorderRadius::from((id.to_string(), value))));
      }
      "borderTopLeftRadius" | "borderTopRightRadius" | "borderBottomLeftRadius" | "borderBottomRightRadius" => {
        final_properties.insert(property_name.to_string(), StyleValueType::BorderRadius(BorderRadius::from((id.to_string(), value))));
      }
      "borderStyle" => {
        final_properties.insert("borderStyle".to_string(), StyleValueType::BorderStyle(BorderStyle::from((id.to_string(), value))));
      }
      "borderTopStyle" | "borderBottomStyle" | "borderLeftStyle" | "borderRightStyle" => {
        final_properties.insert(property_name.to_string(), StyleValueType::BorderStyle(BorderStyle::from((id.to_string(), value))));
      }
      "border" => {
        final_properties.insert("border".to_string(), StyleValueType::Border(Border::from((id.to_string(), value))));
      }
      "borderTop" | "borderBottom" | "borderLeft" | "borderRight" => {
        final_properties.insert(property_name.to_string(), StyleValueType::Border(Border::from((id.to_string(), value))));
      }
      "transform" => {
        final_properties.insert(property_name.to_string(), StyleValueType::Transform(Transform::from((id.to_string(), value))));
      }
      _ => {
        // position、zIndex等... 会自动处理 单位、数字等相关信息
        final_properties.insert(property_name.to_string(), StyleValueType::Normal(Normal::new(id.to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap())));
      }
    }
  }
  final_properties
}