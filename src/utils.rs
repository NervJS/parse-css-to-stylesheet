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
    Value::Number(n) => {
      if n.is_f64() {
        create_flatbuffer_double_value(builder, n)
      } else {
        create_flatbuffer_integer_value(builder, n)
      }
    },
    Value::Bool(b) => create_flatbuffer_boolean_value(builder, *b),
    Value::Array(arr) => {
      if arr.len() > 0 {
        match &arr[0] {
          Value::String(_) => create_flatbuffer_array_string_value(builder, arr),
          Value::Number(_) => {
            let is_integer = arr.iter().all(|n| n.is_i64());
            if is_integer {
              create_flatbuffer_array_integer_value(builder, arr)
            } else {
              create_flatbuffer_array_double_value(builder, arr)
            }
          },
          Value::Object(_) => {
            let key_values: Vec<_> = arr.iter()
              .map(|obj| {
                let (value_type, value) = process_flatbuffer_value(builder, obj);
                styles::KeyValue::create(builder, &styles::KeyValueArgs {
                  key: None,
                  value_type,
                  value: Some(value),
                })
              })
              .collect();
            let fields_offset = builder.create_vector(&key_values);
            let array = styles::ObjectArray::create(builder, &styles::ObjectArrayArgs {
              values: Some(fields_offset),
            });
            (styles::Value::ObjectArray, array.as_union_value())
          },
          Value::Array(_) => {
            let is_integer = arr.iter().all(|n| n.is_i64());
            if is_integer {
              create_flatbuffer_array_array_integer_value(builder, arr)
            } else {
              create_flatbuffer_array_array_double_value(builder, arr)
            }
          },
          _ => {
            println!("{:?}", value);
            panic!("Invalid array type")
          }
        }
      } else {
        panic!("Empty array")
      }
    }
    Value::Object(obj) => create_flatbuffer_object_value(builder, obj),
    _ => {
      println!("{:?}", value);
      panic!("Invalid value type")
    }
  }
}

fn create_flatbuffer_array_array_integer_value(builder: &mut FlatBufferBuilder, arr: &Vec<Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let integer_values: Vec<_> = arr.iter()
    .map(|n| {
      let values: Vec<_> = n.as_array().unwrap()
        .iter()
        .map(|n| n.as_i64().unwrap() as i64)
        .collect();
      let integers = builder.create_vector(&values);
      styles::IntegerArray::create(builder, &&styles::IntegerArrayArgs {
        values: Some(integers),
      })
    })
    .collect();
  let integers = builder.create_vector(&integer_values);
  let array = styles::IntegereArrayArray::create(builder, &&styles::IntegereArrayArrayArgs {
    values: Some(integers),
  });
  (styles::Value::IntegereArrayArray, array.as_union_value())
}

fn create_flatbuffer_array_array_double_value(builder: &mut FlatBufferBuilder, arr: &Vec<Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let double_values: Vec<_> = arr.iter()
    .map(|n| {
      let values: Vec<_> = n.as_array().unwrap()
        .iter()
        .map(|n| n.as_f64().unwrap())
        .collect();
      let doubles = builder.create_vector(&values);
      styles::DoubleArray::create(builder, &&styles::DoubleArrayArgs {
        values: Some(doubles),
      })
    })
    .collect();
  let doubles = builder.create_vector(&double_values);
  let array = styles::DoubleArrayArray::create(builder, &&styles::DoubleArrayArrayArgs {
    values: Some(doubles),
  });
  (styles::Value::DoubleArrayArray, array.as_union_value())
}

fn create_flatbuffer_array_string_value(builder: &mut FlatBufferBuilder, arr: &Vec<Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let string_offsets: Vec<_> = arr.iter()
    .map(|s| builder.create_string(s.as_str().unwrap()))
    .collect();
  let strings = builder.create_vector(&string_offsets);
  let array = styles::StringArray::create(builder, &styles::StringArrayArgs {
    values: Some(strings),
  });
  (styles::Value::StringArray, array.as_union_value())
}

fn create_flatbuffer_array_double_value(builder: &mut FlatBufferBuilder, arr: &Vec<Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let double_values: Vec<_> = arr.iter()
    .map(|n| n.as_f64().unwrap())
    .collect();
  let doubles = builder.create_vector(&double_values);
  let array = styles::DoubleArray::create(builder, &styles::DoubleArrayArgs {
    values: Some(doubles),
  });
  (styles::Value::DoubleArray, array.as_union_value())
}

fn create_flatbuffer_array_integer_value(builder: &mut FlatBufferBuilder, arr: &Vec<Value>) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let integer_values: Vec<_> = arr.iter()
    .map(|n| n.as_i64().unwrap() as i64)
    .collect();
  let integers = builder.create_vector(&integer_values);
  let array = styles::IntegerArray::create(builder, &styles::IntegerArrayArgs {
    values: Some(integers),
  });
  (styles::Value::IntegerArray, array.as_union_value())
}

fn create_flatbuffer_boolean_value(builder: &mut FlatBufferBuilder, b: bool) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let boolean = styles::Boolean::create(builder, &styles::BooleanArgs {
    value: b,
  });
  (styles::Value::Boolean, boolean.as_union_value())
}

fn create_flatbuffer_string_value(builder: &mut FlatBufferBuilder, s: &str) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let string_offset = builder.create_string(s);
  let string = styles::String::create(builder, &styles::StringArgs {
    value: Some(string_offset),
  });
  (styles::Value::String, string.as_union_value())
}

fn create_flatbuffer_double_value(builder: &mut FlatBufferBuilder, n: &serde_json::Number) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let double = styles::Double::create(builder, &styles::DoubleArgs {
    value: n.as_f64().unwrap(),
  });
  (styles::Value::Double, double.as_union_value())
}

fn create_flatbuffer_integer_value(builder: &mut FlatBufferBuilder, n: &serde_json::Number) -> (styles::Value, WIPOffset<UnionWIPOffset>) {
  let integer = styles::Integer::create(builder, &styles::IntegerArgs {
    value: n.as_i64().unwrap() as i64,
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

fn create_flatbuffer_condition<'a>(builder: &mut FlatBufferBuilder<'a>, cond: &serde_json::Value) -> WIPOffset<styles::Condition<'a>> {
  let cond_array = cond.as_array().unwrap();
  let cond_type = cond_array[0].as_u64().unwrap() as u8;
  let condition_value = match cond_type {
    0 => { // PrimitiveCondition
      let params = cond_array[1].as_array().unwrap();
      let feature = params[0].as_u64().unwrap() as u8;
      let operator = params[1].as_u64().unwrap() as u8;
      // 处理value值
      let (value_type, value) = process_flatbuffer_value(
        builder, 
        &params[2]
      );
      
      // 创建PrimitiveCondition
      let primitive = styles::PrimitiveCondition::create(
        builder,
        &styles::PrimitiveConditionArgs {
          feature,
          operator,
          value_type,
          value: Some(value),
        }
      );
      styles::Condition::create(builder, &styles::ConditionArgs {
        type_: cond_type,
        value_type: styles::ConditionValue::PrimitiveCondition,
        value: Some(primitive.as_union_value()),
    })
    },
    _ => {
      let compound_conditions: Vec<_> = cond_array[1]
        .as_array()
        .unwrap()
        .iter()
        .map(|cond| create_flatbuffer_condition(builder, cond))
        .collect();
      
      let vec_offset = builder.create_vector(&compound_conditions);
      let compound = styles::CompoundCondition::create(
        builder,
        &styles::CompoundConditionArgs {
          conditions: Some(vec_offset),
        }
      );
      
      styles::Condition::create(
        builder,
        &styles::ConditionArgs {
          type_: cond_type,
          value_type: styles::ConditionValue::CompoundCondition,
          value: Some(compound.as_union_value()),
        }
      )
    }
  };
  condition_value
}

fn create_flatbuffer_pseudo_key<'a>(builder: &mut FlatBufferBuilder<'a>, key: &serde_json::Value) -> WIPOffset<styles::PseudoKey<'a>> {
  if key.is_i64() {
    let integer_value = key.as_i64().unwrap() as i8;
    styles::PseudoKey::create(builder, &styles::PseudoKeyArgs {
      integer_value,
      bool_value: false,
      is_int: true,
    })
  } else {
    let bool_value = key.as_bool().unwrap();
    styles::PseudoKey::create(builder, &styles::PseudoKeyArgs {
      integer_value: 0,
      bool_value,
      is_int: false,
    })
  }
}

fn create_flatbuffer_variables<'a>(builder: &mut FlatBufferBuilder<'a>, key: &String, value: &serde_json::Value) -> WIPOffset<styles::KeyValueString<'a>>  {
  let key_offset = builder.create_string(key);
  let value_offset = builder.create_string(value.as_str().unwrap());
  styles::KeyValueString::create(builder, &styles::KeyValueStringArgs {
    key: Some(key_offset),
    value: Some(value_offset),
  })
}

pub fn convert_json_to_flatbuffer(json_str: &str) -> Result<Vec<u8>, serde_json::Error> {
  let json: Value = serde_json::from_str(json_str)?;
  let mut builder = FlatBufferBuilder::new();
  let fonts: Vec<WIPOffset<styles::Font>> = json["fonts"]
    .as_array()
    .unwrap()
    .iter()
    .map(|f| {
      let font_family = f["fontFamily"].as_str().unwrap_or("");
      let src = f["src"].as_str().unwrap_or("");

      let font_family = builder.create_string(font_family);
      let src = builder.create_string(src);
      let font = styles::Font::create(&mut builder, &styles::FontArgs {
        font_family: Some(font_family),
        src: Some(src),
      });
      font
    }).collect();
  let fonts = builder.create_vector(&fonts);

  // let keyframes: Vec<WIPOffset<&str>> = json["keyframes"]
  //   .as_array()
  //   .unwrap()
  //   .iter()
  //   .map(|k| builder.create_string(k.as_str().unwrap()))
  //   .collect();
  // let keyframes = builder.create_vector(&keyframes);

  let medias: Vec<WIPOffset<styles::Media>> = json["medias"]
    .as_array()
    .unwrap()
    .iter()
    .map(|m| {
      let conditions: Vec<_> = m["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|cond_array| create_flatbuffer_condition(&mut builder, cond_array))
        .collect();
      let conditions_vector = builder.create_vector(&conditions);
      let media = styles::Media::create(&mut builder, &styles::MediaArgs {
        id: m["id"].as_u64().unwrap() as u8,
        conditions: Some(conditions_vector),
      });
      media
    })
    .collect();
  let medias = builder.create_vector(&medias);

  let styles: Vec<WIPOffset<styles::Style>> = json["styles"]
    .as_array()
    .unwrap()
    .iter()
    .map(|style| {
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
              integer_value: n.as_u64().unwrap() as u8,
              is_string: false,
            }),
            _ => panic!("Invalid selector type"),
          }
        }).collect();
      let selector = builder.create_vector(&selector);

      let declarations: Vec<WIPOffset<styles::DeclarationTuple>> = style["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decl| {
          let decl_array = decl.as_array().unwrap();
          let property_id = decl_array[0].as_u64().unwrap() as u8;
          
          let (value_type, value) = process_flatbuffer_value(&mut builder, &decl_array[1]);
          
          let mut property_flag = 0;
          // 判断下标2是否存在
          if decl_array.len() == 3 {
            property_flag = decl_array[2].as_u64().unwrap() as u8;
          }
          
          styles::DeclarationTuple::create(&mut builder, &styles::DeclarationTupleArgs {
            property_id: property_id,
            value_type: value_type,
            value: Some(value),
            flag: property_flag
          })
        }).collect();
        let declarations = builder.create_vector(&declarations);
        let pseudo_key: Vec<WIPOffset<styles::PseudoKey>> = style["pseudo_key"]
          .as_array()
          .unwrap_or(&vec![])
          .iter()
          .map(|key| create_flatbuffer_pseudo_key(&mut builder, key))
          .collect();
        let pseudo_key_build = builder.create_vector(&pseudo_key);
        let pseudo_val = if let Some(pseudo_val) = style["pseudo_val"].as_str() {
          Some(builder.create_string(pseudo_val))
        } else {
          None
        };

        // variables: Object {"--color": String("red")}
        let variables: Vec<WIPOffset<styles::KeyValueString>> = style["variables"]
          .as_object()
          .unwrap_or(&serde_json::Map::new())
          .iter()
          .map(|(key, value)| {
            create_flatbuffer_variables(&mut builder, key, value)
          })
          .collect();

        let variables_build = builder.create_vector(&variables);

        styles::Style::create(&mut builder, &styles::StyleArgs {
          declarations: Some(declarations),
          media: style["media"].as_u64().unwrap() as u8,
          pseudo: style["pseudo"].as_u64().unwrap_or(0) as u8,
          selector: Some(selector),
          pseudo_key: if pseudo_key.len() > 0 {
            Some(pseudo_key_build)
          } else {
            None
          },
          pseudo_val: pseudo_val,
          variables: if variables.len() > 0 {
            Some(variables_build)
          } else {
            None
          },
        })
    }).collect();
    let styles = builder.create_vector(&styles);
    let design_width = json["design_width"].as_u64().unwrap_or(0) as u16;
    let stylesheet = styles::StyleSheet::create(&mut builder, &styles::StyleSheetArgs {
      fonts: Some(fonts),
      keyframes: None,
      medias: Some(medias),
      styles: Some(styles),
      design_width: design_width,
    });

    builder.finish(stylesheet, None);
    Ok(builder.finished_data().to_vec())

}
