use ego_tree::Tree;
use swc_common::{sync::Lrc, SourceMap, errors::{Handler, ColorConfig}};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_visit::VisitWith;

use crate::{scraper::{Node, Selector, ElementRef}, visitor::AstVisitor};

pub struct JSXDocument {
  pub tree: Tree<Node>
}

impl JSXDocument {
  pub fn new() -> Self {
    JSXDocument { tree: Tree::new(Node::Document) }
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

    let module = parser
      .parse_module()
      .map_err(|e| {
        e.into_diagnostic(&handler).emit()
      })
      .expect("failed to parser module");

    let mut vistor = AstVisitor::new(&module, &mut self.tree);
    module.visit_with(&mut vistor);
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
