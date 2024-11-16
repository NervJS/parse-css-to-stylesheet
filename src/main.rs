use std::{cell::RefCell, fs, rc::Rc};
use style_parser::StyleParser;
use style_propetries::unit::Platform;

use swc_core::{
  ecma::codegen::{text_writer::JsWriter, Emitter},
  common::{comments::SingleThreadedComments, sync::Lrc, SourceMap}
};
use crate::{document::JSXDocument, style_write::StyleWrite};


mod document;
mod scraper;
mod style_write;
mod utils;
mod visitor;
mod constants;
mod style_propetries;
mod style_parser;
mod parse_style_properties;

// component: jsx的code string
// styles: css的code string
// platform_string: "ReactNative" | "Harmony"


pub fn main() {

  let component = fs::read_to_string("__test__/fixure/pesudo.jsx").unwrap();
  let css = fs::read_to_string("__test__/fixure/pesudo.scss").unwrap();

  let platform = Platform::Harmony;
  let is_entry = false; // 是否是入口文件
  let mut is_enable_nesting = true;

  // 解析组件文件
  let cm: Lrc<SourceMap> = Default::default();
  let comments = SingleThreadedComments::default();
  let mut document = JSXDocument::new();
  document.parse(component, cm.clone(), &comments);

  // 解析样式文件
  let mut style_parser = StyleParser::new(&document, platform.clone(), is_entry.clone());
  style_parser.parse(&css);
  let style_data = style_parser.calc();

  // 判断计算的结果是否会含有嵌套选择器
  if is_enable_nesting {
    is_enable_nesting = style_data.has_nesting;
  }

  let program = Rc::new(RefCell::new(document.program.as_ref().unwrap().clone()));
  let jsx_record = Rc::new(RefCell::new(document.jsx_record.as_ref().unwrap().clone()));
  let mut style_write = StyleWrite::new(
    program.clone(),
    jsx_record.clone(),
    style_data.pesudo_style_record.clone(),
    style_data.all_style.clone(),
    is_enable_nesting,
    is_entry
  );
  style_write.write(platform, document.taro_components.clone());

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
  
  println!("{}", code);
}