use std::fs;

use lightningcss::{stylesheet::{StyleSheet, ParserOptions}, visitor::Visit};

use crate::{document::JSXDocument, visitor::StyleVisitor};

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

  let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
  let mut style_visitor = StyleVisitor::new(&document);
  stylesheet.visit(&mut style_visitor).unwrap();
  println!("{:?}", style_visitor.style_record)
}
