use std::fs;

use crate::{document::JSXDocument, style_parser::StyleParser};

mod document;
mod scraper;
mod utils;
mod visitor;
mod style_parser;

fn main() {
  // 使用 swc 解析 JSX
  let jsx = fs::read_to_string("asset/mod.jsx").unwrap();
  let css = fs::read_to_string("asset/Mod.scss").unwrap();

  let mut document = JSXDocument::new();
  document.parse(jsx);

  println!();

  let mut style_parser = StyleParser::new(&document);
  style_parser.parse(&css);
  let style_record = style_parser.calc();
  println!("{:?}", style_record)
}
