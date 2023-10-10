use std::collections::HashMap;

use ego_tree::Tree;
use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  SourceMap,
};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_visit::VisitWith;

use crate::{
  scraper::{ElementRef, Node, Selector},
  visitor::{AstVisitor, JSXRecord},
};

pub struct JSXDocument {
  pub tree: Tree<Node>,
  pub module: Option<Module>,
  pub jsx_record: Option<JSXRecord>,
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument {
      tree: Tree::new(Node::Document),
      module: None,
      jsx_record: None,
    }
  }

  pub fn parse(&mut self, jsx: String) {
    // 初始化 swc 的 SourceMap
    let cm: Lrc<SourceMap> = Default::default();
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
      None,
    );
    // 初始化 swc 的语法分析器
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
    }

    let module = parser
      .parse_module()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("解析 JSX 失败");
    let mut jsx_record: JSXRecord = HashMap::new();
    let mut vistor = AstVisitor::new(&module, &mut self.tree, &mut jsx_record);
    module.visit_with(&mut vistor);
    self.module = Some(module);
    self.jsx_record = Some(jsx_record);
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
