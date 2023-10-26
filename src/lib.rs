#![deny(clippy::all)]

use std::{cell::RefCell, rc::Rc};

use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

use crate::{document::JSXDocument, style_parser::StyleParser, style_write::StyleWrite};

#[macro_use]
extern crate napi_derive;

mod document;
mod scraper;
mod style_parser;
mod style_transform;
mod style_write;
mod utils;
mod visitor;

#[napi]
pub fn parse(component: String, styles: Vec<String>) -> String {
  // 解析组件文件
  let cm: Lrc<SourceMap> = Default::default();
  let comments = SingleThreadedComments::default();
  let mut document = JSXDocument::new();
  document.parse(component, cm.clone(), &comments);

  // 解析样式文件
  let css = styles.join("\n");
  let mut style_parser = StyleParser::new(&document);
  style_parser.parse(&css);
  let style_data = style_parser.calc();

  let program = Rc::new(RefCell::new(document.program.as_ref().unwrap().clone()));
  let jsx_record = Rc::new(RefCell::new(document.jsx_record.as_ref().unwrap().clone()));
  let mut style_write = StyleWrite::new(
    program.clone(),
    jsx_record.clone(),
    style_data.style_record.clone(),
    style_data.all_style.clone(),
  );
  style_write.write();

  // ast 转代码
  let mut buf = Vec::new();
  {
    let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    let mut emitter = Emitter {
      cfg: Default::default(),
      cm: cm.clone(),
      wr: writer,
      comments: Some(&comments),
    };
    emitter.emit_program(&program.borrow()).unwrap();
  }
  let code = String::from_utf8(buf).unwrap().replace("\r\n", "\n");
  code
}
