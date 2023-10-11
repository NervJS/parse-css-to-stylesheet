use std::collections::HashMap;

use ego_tree::Tree;
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  SourceMap, Mark, comments::SingleThreadedComments, Globals, GLOBALS,
};
use swc_ecma_ast::{EsVersion, Program};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_visit::{VisitWith, FoldWith};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene, resolver};
use swc_ecmascript::transforms::typescript::strip;

use crate::{
  scraper::{ElementRef, Node, Selector},
  visitor::{AstVisitor, JSXRecord, CollectVisitor},
};

pub struct JSXDocument {
  pub tree: Tree<Node>,
  pub program: Option<Program>,
  pub jsx_record: Option<JSXRecord>,
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument {
      tree: Tree::new(Node::Document),
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
      let mut vistor = AstVisitor::new(&program, &mut self.tree, &mut jsx_record, &visitor.export_default_name, &visitor.taro_components);
      program.visit_with(&mut vistor);
      self.program = Some(program);
      self.jsx_record = Some(jsx_record);
    });
  }

  pub fn select<'a>(&self, selector: &'a Selector) -> Vec<ElementRef> {
    let nodes = self.tree.nodes();
    let elements = nodes
      .filter_map(|node| ElementRef::wrap(node))
      .filter(|element| element.parent().is_some() && selector.matches(&element))
      .collect::<Vec<_>>();
    elements
  }
}
