use std::collections::HashMap;

use lightningcss::{properties::Property, stylesheet::PrinterOptions};

use crate::style_propetries::{style_value_type::StyleValueType, flex_align::FlexAlign, item_align::ItemAlign, aspect_ratio::AspactRatio, display::Display, flex_basis::FlexBasis, flex_direction::FlexDirection, flex_wrap::FlexWrap, gap::Gap, length_value::LengthValueProperty, size::SizeProperty, max_size::MaxSizeProperty, overflow::Overflow, normal::Normal, number::NumberProperty};

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> HashMap<String, StyleValueType> {
  let mut final_properties = HashMap::new();

  for (id, value) in properties.iter() {
    let property_name = id.as_str();
    match (property_name) {
      "alignContent" => {
        final_properties.insert("alignContent".to_string(), StyleValueType::FlexAlign(FlexAlign::from(value)));
      }
      "justifyContent" => {
        final_properties.insert("justifyContent".to_string(), StyleValueType::FlexAlign(FlexAlign::from(value)));
      }
      "alignItems" => {
        final_properties.insert("alignItems".to_string(), StyleValueType::AlignItems(ItemAlign::from(value)));
      }
      "alignSelf" => {
        final_properties.insert("alignSelf".to_string(), StyleValueType::AlignItems(ItemAlign::from(value)));
      }
      "flexBasis" => {
        final_properties.insert("flexBasis".to_string(), StyleValueType::FlexBasis(FlexBasis::from(value)));
      }
      "flexDirection" => {
        final_properties.insert("flexDirection".to_string(), StyleValueType::FlexDirection(FlexDirection::from(value)));
      }
      "flexGrow" => {
        final_properties.insert("flexGrow".to_string(), StyleValueType::NumberProperty(NumberProperty::from(value)));
      }
      "flexShrink" => {
        final_properties.insert("flexShrink".to_string(), StyleValueType::NumberProperty(NumberProperty::from(value)));
      }
      "flexWrap" => {
        final_properties.insert("flexWrap".to_string(), StyleValueType::FlexWrap(FlexWrap::from(value)));
      }
      "aspectRatio" => {
        final_properties.insert("aspectRatio".to_string(), StyleValueType::AspectRatio(AspactRatio::from(value)));
      }
      "display" => {
        final_properties.insert("display".to_string(), StyleValueType::Display(Display::from(value)));
      }
      "columnGap" => {
        final_properties.insert("columnGap".to_string(), StyleValueType::Gap(Gap::from(value)));
      }
      "rowGap" => {
        final_properties.insert("rowGap".to_string(), StyleValueType::Gap(Gap::from(value)));
      }
      "marginTop" | "marginBottom" | "marginLeft" | "marginRight" | "paddingTop" | "paddingBottom" | "paddingLeft" | "paddingRight" | "top" | "bottom" | "left" | "right" => {
        final_properties.insert(id.to_string(), StyleValueType::LengthValueProperty(LengthValueProperty::from(value)));
      }
      "maxHeight" | "maxWidth" => {
        final_properties.insert(id.to_string(), StyleValueType::MaxSizeProperty(MaxSizeProperty::from(value)));
      }
      "height" | "width" | "minHeight" | "minWidth" => {
        final_properties.insert(id.to_string(), StyleValueType::SizeProperty(SizeProperty::from(value)));
      }
      "overflow" => {
        final_properties.insert("overflow".to_string(), StyleValueType::Overflow(Overflow::from(value)));
      }
      _ => {
        // position、zIndex等... 会自动处理 单位、数字等相关信息
        final_properties.insert(property_name.to_string(), StyleValueType::Normal(Normal::new(value.value_to_css_string(PrinterOptions::default()).unwrap())));
      }
    }
  }
  final_properties
}