#![deny(clippy::all)]

use serde::Deserialize;

use json_writer::JsonWriter;
use style_parser::StyleParser;
use style_propetries::unit::Platform;

#[macro_use]
extern crate napi_derive;

mod constants;
mod json_writer;
mod parse_style_properties;
mod style_parser;
mod style_propetries;
mod utils;
mod visitor;

// styles: css的code string
// platform_string: "ReactNative" | "Harmony"

#[napi(object)]
#[derive(Deserialize)]
pub struct ParseOptions {
  pub platform_string: String,
}

#[napi(object)]
pub struct ParseResult {
  pub code: String,
}

#[napi]
pub fn parse(styles: Vec<String>, options: ParseOptions) -> ParseResult {
  let platform = match options.platform_string.as_str() {
    "ReactNative" => Platform::ReactNative,
    "Harmony" => Platform::Harmony,
    _ => Platform::Harmony,
  };

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
  );

  ParseResult {
    code: style_map.to_json(),
  }
}
