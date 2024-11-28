use std::collections::HashMap;

use html5ever::{namespace_url, ns, LocalName, QualName};
use pcre2::bytes::Regex;
// use lightningcss::values::number::CSSNumber;
use swc_core::ecma::{
  ast::{
    ArrayLit, CallExpr, Expr, Function, JSXMemberExpr, JSXObject, ObjectLit, Prop, PropName,
    PropOrSpread, TsArrayType,
  },
  visit::{Visit, VisitWith},
};
use serde_json::Value;
use flatbuffers::{FlatBufferBuilder, WIPOffset, UnionWIPOffset};

use crate::{constants::SelectorType, stylesheet_generated::styles};

pub fn lowercase_first(s: &mut str) {
  if let Some(c) = s.get_mut(0..1) {
    c.make_ascii_lowercase();
  }
}

pub fn to_camel_case(s: &str, is_first: bool) -> String {
  let mut result = String::new();
  let mut next_cap = if is_first { true } else { false };
  for c in s.chars() {
    if c == '-' || c == '_' {
      next_cap = true;
    } else if next_cap {
      result.extend(c.to_uppercase());
      next_cap = false;
    } else {
      result.push(c);
    }
  }
  result
}

pub fn hex_to_argb(hex: &str) -> Result<u32, String> {
  let hex = hex.trim_start_matches('#');
  let hex = match hex.len() {
    3 => {
      // 转换简写形式，例如 #000 -> #FF000000
      let r = hex.chars().nth(0).ok_or("0")?;
      let g = hex.chars().nth(1).ok_or("0")?;
      let b = hex.chars().nth(2).ok_or("0")?;
      format!("{}{}{}{}{}{}FF", r, r, g, g, b, b)
    }
    4 => {
      // 转换简写形式，例如 #000 -> #FF000000
      let r = hex.chars().nth(0).ok_or("0")?;
      let g: char = hex.chars().nth(1).ok_or("0")?;
      let b = hex.chars().nth(2).ok_or("0")?;
      let a: char = hex.chars().nth(3).ok_or("ff")?;
      format!("{}{}{}{}{}{}{}{}", r, r, g, g, b, b, a, a)
    }
    6 => format!("{}ff", hex.to_string()),
    8 => hex.to_string(),
    _ => return Err(hex.into()),
  };

  // 解析 RGB 值
  let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
  let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
  let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
  let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;

  // 组合成 ARGB 格式，透明度为 0xFF
  let argb = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
  Ok(argb)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TSelector {
  Selector(SelectorType),
  String(String),
  Array(Vec<String>),
}

// 分割选择器
pub fn split_selector(selector: &str) -> Vec<TSelector> {
  let mut result = Vec::new();
  let mut current_word = String::new();
  let mut buffer = String::new();

  for c in selector.chars() {
    if c == ' ' || c == '>' {
      if !current_word.is_empty() {
        let split_selector = split_classes(current_word.as_str());
        match split_selector {
          TSelector::Selector(_) => {}
          TSelector::String(selector) => {
            result.push(TSelector::String(selector));
          }
          TSelector::Array(selectors) => {
            let length = selectors.len();
            for (index, selector) in selectors.iter().enumerate() {
              result.push(TSelector::String(selector.clone()));
              if index != length - 1 {
                result.push(TSelector::Selector(SelectorType::Multiple))
              }
            }
          }
        }
        current_word.clear();
      }
      buffer.push(c);
      if buffer == " > " {
        // 子选择器
        result.push(TSelector::Selector(SelectorType::Parent));
        buffer.clear();
      }
    } else {
      current_word.push(c);
      if buffer == ' '.to_string() {
        // 后代选择器
        result.push(TSelector::Selector(SelectorType::Ancestor));
        buffer.clear();
      }
    }
  }

  if !current_word.is_empty() {
    let split_selector = split_classes(current_word.as_str());
    match split_selector {
      TSelector::Selector(_) => {}
      TSelector::String(selector) => {
        result.push(TSelector::String(selector));
      }
      TSelector::Array(selectors) => {
        let length = selectors.len();
        for (index, selector) in selectors.iter().enumerate() {
          result.push(TSelector::String(selector.clone()));
          if index != length - 1 {
            result.push(TSelector::Selector(SelectorType::Multiple))
          }
        }
      }
    }
  }

  if !buffer.is_empty() {
    result.push(TSelector::String(buffer.clone()));
  }

  result.reverse();

  result
}

// 分割类名 .a.b.c => ["a", "b", "c"]
fn split_classes(input: &str) -> TSelector {
  let mut matches = Vec::new();
  let mut current_class = String::new();
  for char in input.chars() {
    if char == '.' {
      if !current_class.is_empty() {
        matches.push(current_class.clone());
        current_class.clear();
      }
    } else {
      current_class.push(char);
    }
  }
  if !current_class.is_empty() {
    matches.push(current_class);
  }
  if matches.len() > 1 {
    TSelector::Array(matches)
  } else {
    TSelector::String(input.replace(".", ""))
  }
}

fn process_flatbuffer_value(builder: &mut FlatBufferBuilder, value: &Value) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  match value {
    Value::String(s) => create_flatbuffer_string_value(builder, s),
    Value::Number(n) => create_flatbuffer_integer_value(builder, n),
    Value::Object(obj) => create_flatbuffer_object_value(builder, obj),
    _ => {
      println!("{:?}", value);
      panic!("Invalid value type")
    }
  }
}

fn create_flatbuffer_string_value(builder: &mut FlatBufferBuilder, s: &str) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let string_offset = builder.create_string(s);
  let string = styles::String::create(builder, &styles::StringArgs {
    value: Some(string_offset),
  });
  (styles::Value::String, string.as_union_value())
}

fn create_flatbuffer_integer_value(builder: &mut FlatBufferBuilder, n: &serde_json::Number) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let integer = styles::Integer::create(builder, &styles::IntegerArgs {
    value: n.as_u64().unwrap() as u32,
  });
  (styles::Value::Integer, integer.as_union_value())
}

fn create_flatbuffer_object_value(builder: &mut FlatBufferBuilder, obj: &serde_json::Map<String, Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let key_values: Vec<_> = obj.iter()
      .map(|(key, value)| {
        let key_offset = builder.create_string(key);
        let (value_type, value_offset) = process_flatbuffer_value(builder, value);
        
        styles::KeyValue::create(builder, &styles::KeyValueArgs {
          key: Some(key_offset),
          value_type,
          value: Some(value_offset),
        })
      })
      .collect();

  let fields_offset = builder.create_vector(&key_values);
  let object = styles::Object::create(builder, &styles::ObjectArgs {
    fields: Some(fields_offset),
  });
  
  (styles::Value::Object, object.as_union_value())
}

pub fn convert_json_to_flatbuffer(json_str: &str) -> Result<Vec<u8>, serde_json::Error> {
  let json: Value = serde_json::from_str(json_str)?;
  let mut builder = FlatBufferBuilder::new();
  let fonts: Vec<WIPOffset<&str>> = json["fonts"]
    .as_array()
    .unwrap()
    .iter()
    .map(|f| builder.create_string(f.as_str().unwrap()))
    .collect();
  let fonts = builder.create_vector(&fonts);

  let keyframes: Vec<WIPOffset<&str>> = json["keyframes"]
    .as_array()
    .unwrap()
    .iter()
    .map(|k| builder.create_string(k.as_str().unwrap()))
    .collect();
  let keyframes = builder.create_vector(&keyframes);

  let medias: Vec<WIPOffset<&str>> = json["medias"]
    .as_array()
    .unwrap()
    .iter()
    .map(|m| builder.create_string(m.as_str().unwrap()))
    .collect();
  let medias = builder.create_vector(&medias);

  let styles: Vec<WIPOffset<styles::Style>> = json["styles"]
    .as_array()
    .unwrap()
    .iter()
    .map(|style| {
      let declarations: Vec<WIPOffset<styles::DeclarationTuple>> = style["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decl| {
          let decl_array = decl.as_array().unwrap();
          let property_id = decl_array[0].as_u64().unwrap() as u8;
          let (value_type, value) = process_flatbuffer_value(&mut builder, &decl_array[1]);
          styles::DeclarationTuple::create(&mut builder, &styles::DeclarationTupleArgs {
            property_id: property_id,
            value_type: value_type,
            value: Some(value),
          })
        }).collect();
        let declarations = builder.create_vector(&declarations);
        
        let selector: Vec<WIPOffset<styles::Selector>> = style["selector"]
          .as_array()
          .unwrap()
          .iter()
          .map(|sel| {
            match sel {
              Value::String(s) => {
                let string_offset = builder.create_string(s);
                styles::Selector::create(&mut builder, &styles::SelectorArgs {
                  string_value: Some(string_offset),
                  integer_value: 0,
                  is_string: true,
                })
              },
              Value::Number(n) => styles::Selector::create(&mut builder, &styles::SelectorArgs {
                string_value: None,
                integer_value: n.as_u64().unwrap() as u32,
                is_string: false,
              }),
              _ => panic!("Invalid selector type"),
            }
          }).collect();
        let selector = builder.create_vector(&selector);

        styles::Style::create(&mut builder, &styles::StyleArgs {
          declarations: Some(declarations),
          media: style["media"].as_u64().unwrap() as u32,
          selector: Some(selector),
        })
    }).collect();
    let styles = builder.create_vector(&styles);
    let stylesheet = styles::StyleSheet::create(&mut builder, &styles::StyleSheetArgs {
      fonts: Some(fonts),
      keyframes: Some(keyframes),
      medias: Some(medias),
      styles: Some(styles),
    });

    builder.finish(stylesheet, None);
    Ok(builder.finished_data().to_vec())

}
