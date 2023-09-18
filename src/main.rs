use std::fs;

use crate::{document::JSXDocument, scraper::Selector};

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

  // let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
  // stylesheet.visit(&mut StyleVisitor).unwrap();
  let selector = Selector::parse(".txt2").unwrap();
  println!("{:#?}", document.select(&selector));
}
