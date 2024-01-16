use std::collections::HashMap;

use lightningcss::properties::Property;

use crate::style_propetries::{style_value_type::StyleValueType, flex_align::FlexAlign, item_align::ItemAlign, aspect_ratio::AspactRatio, display::Display, flex_basis::FlexBasis};

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> HashMap<String, StyleValueType> {
  let mut final_properties = HashMap::new();

  for (id, value) in properties.iter() {
    let property_name = id.as_str();
    match (property_name) {
      // "width" => {
      //   final_properties.insert("width".to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap());
      // },
      // 布局属性
      "alignContent" => {
        final_properties.insert("alignContent".to_string(), StyleValueType::FlexAlign(FlexAlign::from(value)));
      },
      "alignItems" => {
        final_properties.insert("alignItems".to_string(), StyleValueType::AlignItems(ItemAlign::from(value)));
      },
      "alignSelf" => {
        final_properties.insert("alignSelf".to_string(), StyleValueType::AlignItems(ItemAlign::from(value)));
      },
      "flexBasis" => {
        final_properties.insert("flexBasis".to_string(), StyleValueType::FlexBasis(FlexBasis::from(value)));
      }
      "aspectRatio" => {
        final_properties.insert("aspectRatio".to_string(), StyleValueType::AspectRatio(AspactRatio::from(value)));
      },
      "display" => {
        final_properties.insert("display".to_string(), StyleValueType::Display(Display::from(value)));
      }
      _ => {}
    }
  }
  final_properties
}