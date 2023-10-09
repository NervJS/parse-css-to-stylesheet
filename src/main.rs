use std::{fs, rc::Rc, cell::RefCell};

use swc_common::{SourceMap, comments::SingleThreadedComments};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};

use crate::{document::JSXDocument, style_parser::StyleParser, style_write::StyleWrite};

mod document;
mod scraper;
mod utils;
mod visitor;
mod style_parser;
mod style_write;

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
  
  // println!("{:?}", style_record)
  let module = Rc::new(RefCell::new(document.module.as_ref().unwrap().clone()));
  let jsx_record = Rc::new(RefCell::new(document.jsx_record.as_ref().unwrap().clone()));
  let style_record = Rc::new(RefCell::new(style_record));
  let mut style_write = StyleWrite::new(module.clone(), jsx_record.clone(), style_record.clone());
  style_write.write();
  
  // ast 转代码
  let cm = Rc::new(SourceMap::default());
  let comments = SingleThreadedComments::default();

  let mut buf = Vec::new();
  {
    let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    let mut emitter = Emitter {
      cfg: Default::default(),
      cm: cm.clone(),
      wr: writer,
      comments: Some(&comments),
    };
    emitter.emit_module(&module.borrow()).unwrap();
  }
  let code = String::from_utf8(buf).unwrap();
  println!("{}", code);
}
