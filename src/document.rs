use std::collections::HashMap;

use selectors::attr::CaseSensitivity;

use swc_core::{
  ecma::{
    ast::{EsVersion, Program},
    parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig},
    visit::{FoldWith, VisitAllWith, VisitWith},
    transforms::{
      base::{fixer::fixer, hygiene::hygiene, resolver},
      typescript::strip
    },
  },
  common::{
    comments::SingleThreadedComments,
    errors::{ColorConfig, Handler},
    sync::Lrc,
    Globals, Mark, SourceMap, GLOBALS,FileName
  }
};

use crate::{
  scraper::Element,
  visitor::{AstVisitor, CollectVisitor, JSXRecord},
};

pub struct JSXDocument {
  pub program: Option<Program>,
  pub jsx_record: Option<JSXRecord>,
  pub taro_components: Vec<String>,
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument {
      program: None,
      jsx_record: None,
      taro_components: Vec::new(),
    }
  }

  pub fn jsx_parse (&mut self, jsx: String, cm: Lrc<SourceMap>, comments: &SingleThreadedComments) -> Program {
    // 初始化 swc 的错误处理器
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // 将 JSX 代码转换为 SourceFile
    let fm = cm.new_source_file(FileName::Anon, jsx);
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
    program
  }

  pub fn parse(&mut self, jsx: String, cm: Lrc<SourceMap>, comments: &SingleThreadedComments) {
    let program = self.jsx_parse(jsx, cm, comments);

    let globals = Globals::default();
    GLOBALS.set(&globals, || {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();
      let program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));
      let program = program.fold_with(&mut strip(top_level_mark));
      let program = program.fold_with(&mut hygiene());
      let program = program.fold_with(&mut fixer(Some(comments)));
      let mut jsx_record: JSXRecord = HashMap::new();
      // 收集使用的 Taro Component
      let mut visitor = CollectVisitor::new();
      program.visit_with(&mut visitor);
      self.taro_components = visitor.taro_components.to_vec();
      let mut visitor = AstVisitor::new(&mut jsx_record, &visitor.taro_components);
      program.visit_all_with(&mut visitor);
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
