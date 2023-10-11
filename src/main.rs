use std::{fs, rc::Rc, cell::RefCell};

use swc_common::{SourceMap, comments::SingleThreadedComments, sync::Lrc};
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
  let jsx = fs::read_to_string("__test__/fixure/mod.jsx").unwrap();
  let css = fs::read_to_string("__test__/fixure/Mod.scss").unwrap();
  let cm: Lrc<SourceMap> = Default::default();
  let comments = SingleThreadedComments::default();
  let mut document = JSXDocument::new();
  document.parse(jsx, cm.clone(), &comments);

  println!();
  let mut style_parser = StyleParser::new(&document);
  style_parser.parse(&css);
  let style_record = style_parser.calc();
  
  // println!("{:?}", style_record)
  let program = Rc::new(RefCell::new(document.program.as_ref().unwrap().clone()));
  let jsx_record = Rc::new(RefCell::new(document.jsx_record.as_ref().unwrap().clone()));
  let style_record = Rc::new(RefCell::new(style_record));
  let mut style_write = StyleWrite::new(program.clone(), jsx_record.clone(), style_record.clone());
  style_write.write();
  
  // ast 转代码
  let mut buf = vec![];
  {
    let mut emitter = Emitter {
      cfg: swc_ecma_codegen::Config::default(),
      cm: cm.clone(),
      comments: Some(&comments),
      wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };
    emitter.emit_program(&program.borrow()).unwrap();
  }
  let code = String::from_utf8(buf).unwrap().replace("\r\n", "\n");
  println!("{}", code);
}
