use std::collections::HashMap;

use selectors::attr::CaseSensitivity;
use swc_common::{
  comments::SingleThreadedComments,
  errors::{ColorConfig, Handler},
  sync::Lrc,
  Globals, Mark, SourceMap, GLOBALS,
};
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecma_visit::{FoldWith, VisitAllWith, VisitWith};
use swc_ecmascript::transforms::typescript::strip;

use crate::{
  scraper::Element,
  visitor::{AstVisitor, CollectVisitor, JSXRecord},
};

pub struct JSXDocument {
  pub program: Option<Program>,
  pub jsx_record: Option<JSXRecord>,
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument {
      program: None,
      jsx_record: None,
    }
  }

  pub fn parse(&mut self, jsx: String, cm: Lrc<SourceMap>, comments: &SingleThreadedComments) {
    // 初始化 swc 的错误处理器
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // 将 JSX 代码转换为 SourceFile
    let fm = cm.new_source_file(swc_common::FileName::Anon, jsx);
    // 初始化 swc 的词法分析器
    let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: true,
        ..Default::default()
      }),
      EsVersion::Es2019,
      StringInput::from(&*fm),
      Some(comments),
    );
    // 初始化 swc 的语法分析器
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
    }
    let program = parser
      .parse_program()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("解析 JSX 失败");

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();
      let program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));
      let program = program.fold_with(&mut strip(top_level_mark));
      let program = program.fold_with(&mut hygiene());
      let program = program.fold_with(&mut fixer(Some(comments)));
      let mut jsx_record: JSXRecord = HashMap::new();
      let mut visitor = CollectVisitor::new();
      program.visit_with(&mut visitor);
      let mut vistor = AstVisitor::new(&mut jsx_record, &visitor.taro_components);
      program.visit_all_with(&mut vistor);
      self.program = Some(program);
      self.jsx_record = Some(jsx_record);
    });
  }

  pub fn select<'a>(&self, selector: &'a str) -> Vec<Element> {
    match self.jsx_record {
      Some(ref jsx_record) => {
        let mut elements = Vec::new();
        for (_, element) in jsx_record.iter() {
          if element.has_class(selector, CaseSensitivity::CaseSensitive) {
            elements.push(element.clone());
          }
        }
        elements
      }
      None => Vec::new(),
    }
  }
}
