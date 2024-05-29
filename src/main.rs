use style_parser::StyleParser;
use json_writer::JsonWriter;
use style_propetries::unit::Platform;


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


pub fn main() {

  let css = std::fs::read_to_string("__test__/fixure/pesudo.scss").unwrap();

  let platform = Platform::Harmony;

  // 解析样式文件
  let mut style_parser = StyleParser::new(platform.clone());
  style_parser.parse(&css);
  let style_data = style_parser.calc();

  // 输出成JSON格式
  let style_map = JsonWriter::new(style_data.all_style.borrow().clone());

  print!("{}", style_map.to_json());
  
}