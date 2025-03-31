#![deny(clippy::all)]

use serde::Deserialize;
use napi::bindgen_prelude::Buffer;

use json_writer::JsonWriter;
use style_parser::StyleParser;
use style_propetries::unit::Platform;

#[macro_use]
extern crate napi_derive;

mod stylesheet_generated;
mod constants;
mod json_writer;
mod parse_style_properties;
mod style_parser;
mod style_propetries;
mod utils;
mod visitor;
// mod ffi;

// use ffi::process_stylesheet;

// styles: css的code string
// platform_string: "ReactNative" | "Harmony"

#[napi(object)]
#[derive(Deserialize)]
pub struct OutputOptions {
  pub is_bin: Option<bool>
}

#[napi(object)]
#[derive(Deserialize)]
pub struct ParseOptions {
  pub platform_string: String,
  pub design_width: Option<i32>,
  pub output: Option<OutputOptions>,
}

#[napi(object)]
pub struct ParseResult {
  pub code: Option<String>,
  pub buffer: Option<Buffer>,
}

#[napi]
pub fn parse(styles: Vec<String>, options: ParseOptions) -> ParseResult {
  let platform = match options.platform_string.as_str() {
    "ReactNative" => Platform::ReactNative,
    "Harmony" => Platform::Harmony,
    _ => Platform::Harmony,
  };
  let design_width = options.design_width;
  let output = options.output.unwrap_or(OutputOptions {
    is_bin: Some(false)
  });
  let is_bin = output.is_bin.unwrap_or(false);

  // 解析样式文件
  let css = styles.join("\n");
  let mut style_parser = StyleParser::new(platform.clone());
  style_parser.parse(&css);
  let style_data = style_parser.calc();

  // 解析过滤器

  // 输出成JSON格式
  let style_map = JsonWriter::new(
    style_data.all_style.borrow().clone(),
    style_data.all_keyframes.borrow().clone(),
    style_data.all_medias.borrow().clone(),
    style_data.all_fonts.borrow().clone(),
    design_width,
  );

  let style_json = style_map.to_json();
  if is_bin {
    // 输出到文件
    let convert_result = utils::convert_json_to_flatbuffer(&style_json);
    if let Ok(buffer) = convert_result {
      return ParseResult {
        code: None,
        buffer: Some(Buffer::from(buffer)),
      };
    }
    return ParseResult {
      code: Some(style_json),
      buffer: None
    };
  }

  ParseResult {
    code: Some(style_json),
    buffer: None
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::convert_json_to_flatbuffer;
  use serde_json::json;
  use crate::stylesheet_generated::styles;

  #[test]
  fn test_valid_input() {
    let json_input = json!({"fonts":[],"keyframes":[],"medias":[],"styles":[{"declarations":[[22,293],[42,4278753764u32],[25, "var(--h)", 1]],"media":0,"selector":["app"]},{"declarations":[[41,4294901760u32]],"media":0,"selector":["tit"]},{"declarations":[[29,24],[41,4291979550u32]],"media":0,"selector":["tit",2,"app"]},{"declarations":[[22,100],[25,100]],"media":0,"selector":["img"]}, {"declarations":[[79,"hello"]],"media":0,"pseudo":1,"selector":["hello"]},{"declarations":[[42,4294967264u32]],"media":0,"pseudo":5,"pseudo_key":[2,0,true],"pseudo_val":"2n","selector":["bbb"]}]}).to_string();

    let result = convert_json_to_flatbuffer(&json_input);
    assert!(result.is_ok());

    let buffer = result.unwrap();
    let style_sheet = styles::root_as_style_sheet(&buffer).unwrap();

    // 验证 fonts
    assert_eq!(style_sheet.fonts().unwrap().len(), 0);

    // 验证 keyframes
    // assert_eq!(style_sheet.keyframes().unwrap().len(), 0);

    // 验证 medias
    assert_eq!(style_sheet.medias().unwrap().len(), 0);

    // 验证 styles
    let styles = style_sheet.styles().unwrap();
    assert_eq!(styles.len(), 6); // 根据实际情况调整
    let first_style = styles.get(0);
    assert_eq!(first_style.declarations().unwrap().len(), 3);
    
    let first_declaration = first_style.declarations().unwrap().get(0);
    assert_eq!(first_declaration.property_id(), 22);
    assert!(first_declaration.value_as_integer().is_some());
    assert_eq!(first_declaration.value_as_integer().unwrap().value(), 293);

    let second_declaration = first_style.declarations().unwrap().get(1);
    assert_eq!(second_declaration.property_id(), 42);
    assert!(second_declaration.value_as_integer().is_some());
    assert_eq!(second_declaration.value_as_integer().unwrap().value(), 4278753764);
    assert_eq!(second_declaration.flag(), 0);

    let third_declaration = first_style.declarations().unwrap().get(2);
    assert_eq!(third_declaration.property_id(), 25);
    assert!(third_declaration.value_as_string().is_some());
    assert_eq!(third_declaration.flag(), 1);

    let first_selector = first_style.selector().unwrap().get(0);
    assert_eq!(first_selector.string_value().unwrap(), "app");

    let thrird_style = styles.get(2);
    assert_eq!(thrird_style.declarations().unwrap().len(), 2);
    let selector = thrird_style.selector().unwrap();
    assert_eq!(selector.len(), 3);
    let first_selector = selector.get(0);
    assert_eq!(first_selector.string_value().unwrap(), "tit");
    let second_selector = selector.get(1);
    assert_eq!(second_selector.integer_value(), 2);

    let fourth_style = styles.get(3);
    assert_eq!(fourth_style.declarations().unwrap().len(), 2);
    let selector = fourth_style.selector().unwrap();
    assert_eq!(selector.len(), 1);
    let first_selector = selector.get(0);
    assert_eq!(first_selector.string_value().unwrap(), "img");

    let fifth_style = styles.get(4);
    assert_eq!(fifth_style.declarations().unwrap().len(), 1);
    let selector = fifth_style.selector().unwrap();
    assert_eq!(selector.len(), 1);
    let first_selector = selector.get(0);
    assert_eq!(first_selector.string_value().unwrap(), "hello");
    assert_eq!(fifth_style.pseudo(), 1);

    let sixth_style = styles.get(5);
    assert_eq!(sixth_style.declarations().unwrap().len(), 1);
    let selector = sixth_style.selector().unwrap();
    assert_eq!(selector.len(), 1);
    let first_selector = selector.get(0);
    assert_eq!(first_selector.string_value().unwrap(), "bbb");
    assert_eq!(sixth_style.pseudo(), 5);
    assert_eq!(sixth_style.pseudo_key().unwrap().len(), 3);
    let first_pseudo_key = sixth_style.pseudo_key().unwrap().get(0);
    assert_eq!(first_pseudo_key.integer_value(), 2);
    assert_eq!(first_pseudo_key.bool_value(), false);
    assert_eq!(first_pseudo_key.is_int(), true);
    let second_pseudo_key = sixth_style.pseudo_key().unwrap().get(1);
    assert_eq!(second_pseudo_key.integer_value(), 0);
    assert_eq!(second_pseudo_key.bool_value(), false);
    assert_eq!(second_pseudo_key.is_int(), true);
    let third_pseudo_key = sixth_style.pseudo_key().unwrap().get(2);
    assert_eq!(third_pseudo_key.integer_value(), 0);
    assert_eq!(third_pseudo_key.bool_value(), true);
    assert_eq!(third_pseudo_key.is_int(), false);
    assert_eq!(sixth_style.pseudo_val().unwrap(), "2n");
  }
}
