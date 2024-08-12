use json_writer::JsonWriter;
use style_parser::StyleParser;
use style_propetries::unit::Platform;

mod constants;
mod json_writer;
mod parse_style_properties;
mod style_parser;
mod style_propetries;
mod utils;
mod visitor;

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
  let style_map = JsonWriter::new(
    style_data.all_style.borrow().clone(),
    style_data.all_keyframes.borrow().clone(),
    style_data.all_medias.borrow().clone(),
    style_data.all_fonts.borrow().clone(),
  );

  print!("{}", style_map.to_json());
}
