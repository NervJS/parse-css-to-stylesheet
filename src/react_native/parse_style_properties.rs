use std::collections::HashMap;

use lightningcss::{properties::Property, stylesheet::PrinterOptions};

pub fn parse_style_properties(properties: &Vec<(String, Property)>) -> HashMap<String, String> {
  let mut final_properties = HashMap::new();

  for (id, value) in properties.iter() {
    let property_name = id.as_str();
    match (property_name) {
      // "width" => {
      //   final_properties.insert("width".to_string(), value.value_to_css_string(PrinterOptions::default()).unwrap());
      // },
      _ => {}
    }
  }
  final_properties
}