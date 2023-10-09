use std::{collections::HashMap, rc};

use ego_tree::Tree;
use swc_common::{sync::Lrc, SourceMap, errors::{Handler, ColorConfig}, comments::SingleThreadedComments};
use swc_ecma_ast::{EsVersion, Module};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_visit::{VisitWith, VisitMutWith};

use crate::{scraper::{Node, Selector, ElementRef}, visitor::{AstVisitor, JSXRecord, AstMutVisitor}};

pub struct JSXDocument {
  pub tree: Tree<Node>,
  pub module: Option<Module>,
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument {
      tree: Tree::new(Node::Document),
      module: None,
    }
  }

  pub fn parse(&mut self, jsx: String) {
    // 初始化 swc 的 SourceMap
    let cm: Lrc<SourceMap> = Default::default();
    // 初始化 swc 的错误处理器
    let handler = Handler::with_tty_emitter(
      ColorConfig::Auto,
      true,
      false,
      Some(cm.clone()),
    );

    // 将 JSX 代码转换为 SourceFile
    let fm = cm.new_source_file(
      swc_common::FileName::Anon,
      jsx,
    );

    // 初始化 swc 的词法分析器
    let lexer = Lexer::new(
      Syntax::Typescript(
        TsConfig {
          tsx: true,
          ..Default::default()
        }
      ),
      EsVersion::Es2019,
      StringInput::from(&*fm),
      None
    );
    // 初始化 swc 的语法分析器
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
    }

    let mut module = parser
      .parse_module()
      .map_err(|e| {
        e.into_diagnostic(&handler).emit()
      })
      .expect("failed to parser module");
    let mut jsx_record: JSXRecord = HashMap::new();
    let mut vistor = AstVisitor::new(&module, &mut self.tree, &mut jsx_record);
    module.visit_with(&mut vistor);
    let mut mut_visitor = AstMutVisitor::new( &mut jsx_record);
    module.visit_mut_with(&mut mut_visitor);
    // // ast 转代码
    // let cm = rc::Rc::new(SourceMap::default());
    // let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    // let comments = SingleThreadedComments::default();

    // let mut buf = Vec::new();
    // {
    //   let writer = Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None));
    //   let mut emitter = Emitter {
    //     cfg: Default::default(),
    //     cm: cm.clone(),
    //     wr: writer,
    //     comments: Some(&comments),
    //   };
    //   emitter.emit_module(&module).unwrap();
    // }
    // let code = String::from_utf8(buf).unwrap();
    // println!("{}", code);
    self.module = Some(module);
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
