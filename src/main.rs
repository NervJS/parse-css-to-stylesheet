use std::{fs, convert::Infallible};

use ego_tree::{Tree, NodeMut, NodeId};
use html5ever::{Attribute, tendril::StrTendril};
use swc_common::{sync::Lrc, SourceMap, errors::{Handler, ColorConfig}};
use swc_ecma_ast::{EsVersion, JSXElement, JSXAttrOrSpread, JSXAttrName, JSXAttrValue, Lit, Stmt, Expr, JSXElementChild, JSXElementName, JSXMemberExpr, JSXObject, JSXExpr, Function, ExportDefaultExpr, Module};
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_visit::{Visit, VisitWith};
use lightningcss::{visitor::{Visitor, VisitTypes}, rules::{CssRule}, visit_types};
use test_cssparse::{Node, Element, create_qualname};

struct JSXVisitor<'a> {
  tree: &'a mut Tree<Node>,
  root_node: Option<NodeId>,
  current_node: Option<NodeId>
}

impl<'a> JSXVisitor<'a> {
  fn new(tree: &'a mut Tree<Node>) -> Self {
    JSXVisitor { tree, root_node: None, current_node: None }
  }
  fn create_element(&mut self, jsx_element: &JSXElement) -> Node {
    let name = match &jsx_element.opening.name {
      JSXElementName::Ident(ident) => ident.sym.to_string(),
      JSXElementName::JSXMemberExpr(expr) => {
        recursion_jsx_menber(expr)
      },
      JSXElementName::JSXNamespacedName(namespaced_name) => {
        format!("{}:{}", namespaced_name.ns.sym.to_string(), namespaced_name.name.sym.to_string())
      }
    };
    let qual_name = create_qualname(name.as_str());
    let mut attributes = Vec::new();
    for attr in &jsx_element.opening.attrs {
      if let JSXAttrOrSpread::JSXAttr(attr) = attr {
        let name = match &attr.name {
          JSXAttrName::Ident(ident) => ident.sym.to_string(),
          JSXAttrName::JSXNamespacedName(namespaced_name) => {
            format!("{}:{}", namespaced_name.ns.sym.to_string(), namespaced_name.name.sym.to_string())
          }
        };
        let value = match &attr.value {
          Some(value) => {
            match value {
              JSXAttrValue::Lit(lit) => {
                match lit {
                  Lit::Str(str) => str.value.to_string(),
                  Lit::Num(num) => num.value.to_string(),
                  Lit::Bool(bool) => bool.value.to_string(),
                  Lit::Null(_) => "null".to_string(),
                  Lit::BigInt(bigint) => bigint.value.to_string(),
                  Lit::Regex(regex) => regex.exp.to_string(),
                  Lit::JSXText(text) => text.value.to_string(),
                }
              },
              JSXAttrValue::JSXExprContainer(expr_container) => {
                match &expr_container.expr {
                  JSXExpr::JSXEmptyExpr(_) => "{{}}".to_string(),
                  JSXExpr::Expr(expr) => {
                    match &**expr {
                      Expr::Lit(lit) => {
                        match lit {
                          Lit::Str(str) => str.value.to_string(),
                          Lit::Num(num) => num.value.to_string(),
                          Lit::Bool(bool) => bool.value.to_string(),
                          Lit::Null(_) => "null".to_string(),
                          Lit::BigInt(bigint) => bigint.value.to_string(),
                          Lit::Regex(regex) => regex.exp.to_string(),
                          Lit::JSXText(text) => text.value.to_string(),
                        }
                      },
                      _ => "".to_string()
                    }
                  },
                }
              },
              JSXAttrValue::JSXElement(_) => {
                "".to_string()
              },
              JSXAttrValue::JSXFragment(_) => {
                "".to_string()
              }
            }
          },
          None => "".to_string()
        };
        attributes.push(Attribute {
          name: create_qualname(name.as_str()),
          value: StrTendril::from(value),
        });
      }
    }
    Node::Element(Element::new(qual_name, attributes))
  }
}

impl<'a> Visit for JSXVisitor<'a> {
  fn visit_jsx_element(
    &mut self,
    jsx: &JSXElement,
  ) {
    if self.root_node.is_none() {
      let node = self.create_element(jsx);
      let mut root = self.tree.root_mut();
      self.root_node = Some(root.id());
      let current = root.append(node);
      self.current_node = Some(current.id());
    }
    jsx.visit_children_with(self)
  }
    
  fn visit_jsx_element_children(&mut self, n: &[JSXElementChild]) {
    let mut nodes = vec![];
    let mut elements = vec![];
    for child in n.iter() {
      match child {
        JSXElementChild::JSXElement(element) => {
          let node = self.create_element(element);
          let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
          let tree_node = current.append(node);
          nodes.push(tree_node.id());
          elements.push(element);
        },
        _ => {}
      }
    }
    for (index, element) in elements.iter().enumerate() {
      let mut visitor = JSXVisitor::new(self.tree);
      visitor.current_node = Some(nodes[index]);
      visitor.root_node = self.root_node;
      element.visit_with(&mut visitor);
    }
  }
}

struct AstVisitor<'a> {
  export_default_name: Option<String>,
  module: &'a Module,
  tree: &'a mut Tree<Node>
}

impl<'a> AstVisitor<'a> {
  fn new(module: &'a Module, tree: &'a mut Tree<Node>) -> Self {
    AstVisitor { export_default_name: None, module, tree }
  }
}

impl<'a> Visit for AstVisitor<'a> {
  fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
    match &self.export_default_name {
      Some(name) => {
        if n.ident.sym.to_string() == name.as_str() {
          match &*n.function {
            Function { body: Some(body), .. } => {
              for stmt in &body.stmts {
                match stmt {
                  Stmt::Return(return_stmt) => {
                    let mut jsx_visitor = JSXVisitor::new(self.tree);
                    return_stmt.visit_with(&mut jsx_visitor);
                    println!("{:?}", jsx_visitor.tree)
                  },
                  _ => {}
                }
              }
            },
            _ => {}
          }
        }
      },
      None => {}
    }
  }

  fn visit_export_default_expr(&mut self, n: &ExportDefaultExpr) {
    match &*n.expr {
      Expr::Ident(ident) => {
        if self.export_default_name.is_none() {
          self.export_default_name = Some(ident.sym.to_string());
          self.module.visit_with(self)
        }
      },
      _ => {}
    }
  }
}

// struct CSSRuleParser;

// impl<'i> QualifiedRuleParser<'i> for CSSRuleParser {
//   type Prelude = ();
//   type QualifiedRule = ();
//   type Error = ();

//   fn parse_block<'t>(
//           &mut self,
//           prelude: Self::Prelude,
//           start: &cssparser::ParserState,
//           input: &mut CSSParser<'i, 't>,
//       ) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>> {
//     println!("parse_block");
//     Ok(())
//   }
// }

// impl<'i> AtRuleParser<'i> for CSSRuleParser {
//   type Prelude = ();
//   type AtRule = ();
//   type Error = ();

//   fn parse_block<'t>(
//           &mut self,
//           prelude: Self::Prelude,
//           start: &cssparser::ParserState,
//           input: &mut CSSParser<'i, 't>,
//       ) -> Result<Self::AtRule, cssparser::ParseError<'i, Self::Error>> {
//     println!("parse_block at rule");
//     Ok(())
//   }
// }

struct StyleVisitor;

impl<'i> Visitor<'i> for StyleVisitor {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        println!("{:?}", style.selectors);
        println!();
      },
      _ => {}
    }
    Ok(())
  }
}

pub struct JSXDocument {
  tree: Tree<Node>
}

impl JSXDocument {
  fn new() -> Self {
    JSXDocument { tree: Tree::new(Node::Document) }
  }

  fn parse(&mut self, jsx: String) {
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
    // for module_item in &module.body {
    //   match module_item {
    //     ModuleItem::ModuleDecl(module_del) => {
    //       println!("{:?}", module_del);
    //       match module_del {
    //         ModuleDecl::ExportDecl(export_decl) => {
    //           match &export_decl.decl {
    //             Decl::Fn(fn_decl) => {
    //               match &*fn_decl.function {
    //                 Function { body: Some(body), .. } => {
    //                   for stmt in &body.stmts {
    //                     match stmt {
    //                       Stmt::Return(return_stmt) => {
    //                         return_stmt.arg.as_ref().map(|arg| {
    //                           match &**arg {
    //                             Expr::JSXElement(jsx_element) => {
    //                               println!("JSXElement");
    //                             },
    //                             _ => {}
    //                           }
    //                         });
    //                       },
    //                       _ => {}
    //                     }
    //                   }
    //                 },
    //                 _ => {}
    //               }
    //             },
    //             _ => {}
    //           }
    //         },
    //         _ => {}
    //       }
    //     },
    //     ModuleItem::Stmt(stmt) => {
    //       match stmt {
    //         Stmt::Expr(expr_stmt) => {
    //           match &*expr_stmt.expr {
    //             Expr::JSXElement(jsx_element) => {
    //               let mut node = self.create_element(jsx_element);
    //               root.append(node);
    //             },
    //             _ => {}
    //           }
    //         },
    //         _ => {}
    //       }
    //     }
    //   }
    // }
  }
}

fn recursion_jsx_menber(expr: &JSXMemberExpr) -> String {
  match &expr.obj {
    JSXObject::JSXMemberExpr(expr) => {
      format!("{}.{}", recursion_jsx_menber(expr), expr.prop.sym.to_string())
    },
    JSXObject::Ident(ident) => {
      format!("{}.{}", ident.sym.to_string(), expr.prop.sym.to_string())
    }
  }
}

fn convert_jsx_element_child(parent: &mut NodeMut<Node>, child: &JSXElementChild) {

}

fn main() {
  // 使用 swc 解析 JSX
  let jsx = fs::read_to_string("asset/mod.jsx").unwrap();
  let css = fs::read_to_string("asset/Mod.scss").unwrap();

  let mut document = JSXDocument::new();
  document.parse(jsx);

  // let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
  // stylesheet.visit(&mut StyleVisitor).unwrap();
  
}
