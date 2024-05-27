#![deny(clippy::all)]

use std::{cell::RefCell, rc::Rc};
use serde::Deserialize;

use style_parser::StyleParser;
use style_propetries::unit::Platform;

use swc_core::{
  ecma::codegen::{text_writer::JsWriter, Emitter},
  common::{comments::SingleThreadedComments, sync::Lrc, SourceMap}
};

#[macro_use]
extern crate napi_derive;

mod utils;
mod visitor;
mod constants;
mod style_propetries;
mod style_parser;
mod parse_style_properties;
mod json_writer;

// component: jsx的code string
// styles: css的code string
// platform_string: "ReactNative" | "Harmony"

#[napi(object)]
#[derive(Deserialize)]
pub struct ParseOptions {
  pub platform_string: String
}

#[napi(object)]
pub struct ParseResult {
  pub code: String
}

#[napi]
pub fn parse(component: String, styles: Vec<String>, options: ParseOptions) -> ParseResult {

  let platform = match options.platform_string.as_str() {
    "ReactNative" => Platform::ReactNative,
    "Harmony" => Platform::Harmony,
    _ => Platform::Harmony
  };

  // 解析样式文件
  let css = styles.join("\n");
  let mut style_parser = StyleParser::new(platform.clone());
  style_parser.parse(&css);
  let style_data = style_parser.calc();

  // 输出成JSON格式
  let style_map = json_writer::JsonWriter::new(style_data.all_style.borrow().clone());

  ParseResult {
    code: style_map.to_json()
  }
}
