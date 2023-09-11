use std::{fs, convert::Infallible};

use cssparser::{ParserInput, Parser as CSSParser, StyleSheetParser, QualifiedRuleParser, AtRuleParser};
use ego_tree::{Tree, NodeMut};
use html5ever::{Attribute, tendril::StrTendril};
use swc_common::{sync::Lrc, SourceMap, errors::{Handler, ColorConfig}};
use swc_ecma_ast::{EsVersion, JSXElement, JSXAttrOrSpread, JSXAttrName, JSXAttrValue, Lit, ModuleItem, Stmt, Expr, JSXElementChild, JSXElementName, JSXMemberExpr, Ident, JSXObject, JSXExpr};
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_visit::{Visit, VisitWith};
use lightningcss::{stylesheet::{StyleSheet, ParserOptions}, visitor::{Visitor, VisitTypes, Visit as CSSVisit}, rules::{CssRule, style::StyleRule}, visit_types};
use test_cssparse::{Node, Element, create_qualame};

struct JSXVisitor;

impl Visit for JSXVisitor {
  fn visit_jsx_element(
    &mut self,
    jsx: &JSXElement,
  ) {
    // 打印 JSX 元素的 className 属性
    for attr in &jsx.opening.attrs {
      if let JSXAttrOrSpread::JSXAttr(attr) = attr {
        if let JSXAttrName::Ident(ident) = &attr.name {
          if ident.sym.to_string() == "className" {
            if let Some(value) = &attr.value {
              match value {
                JSXAttrValue::Lit(lit) => {
                  if let Lit::Str(str) = lit {
                    println!("className: {}", str.value);
                  }
                },
                _ => {}
              }
            }
          }
        }
      }
    }
    jsx.visit_children_with(self);
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
    // 遍历语法树，查找 JSX 节点
    // let mut vistor = JSXVisitor;
    // module.visit_with(&mut vistor);
    // for module_item in &module.body {
    //   match &module_item {
    //     // JSXElement { span, opening, children, closing } => {
    //     //   for attr in &opening.attrs {
    //     //     if let JSXAttrOrSpread::JSXAttr(attr) = attr {
    //     //       if let JSXAttrName::Ident(ident) = &attr.name {
    //     //         if ident.sym.to_string() == "className" {
    //     //           if let Some(value) = &attr.value {
    //     //             match value {
    //     //               JSXAttrValue::Lit(lit) => {
    //     //                 if let Lit::Str(str) = lit {
    //     //                   println!("className: {}", str.value);
    //     //                 }
    //     //               },
    //     //               _ => {}
    //     //             }
    //     //           }
    //     //         }
    //     //       }
    //     //     }
    //     //   }
    //     // }

    //   }
    // }
    let mut root = self.tree.root_mut();
    for module_item in &module.body {
      match module_item {
        ModuleItem::Stmt(stmt) => {
          match stmt {
            Stmt::Expr(expr_stmt) => {
              match &*expr_stmt.expr {
                Expr::JSXElement(jsx_element) => {
                  println!("{:?}", jsx_element.opening.name);
                },
                _ => ()
              }
            },
            _ => ()
          }
        },
        _ => ()
      }
    }
  }

  fn create_element(&self, jsx_element: &JSXElement) -> NodeMut<Node> {
    let name = match &jsx_element.opening.name {
      JSXElementName::Ident(ident) => ident.sym.to_string(),
      JSXElementName::JSXMemberExpr(expr) => {
        recursion_jsx_menber(expr)
      },
      JSXElementName::JSXNamespacedName(namespaced_name) => {
        format!("{}:{}", namespaced_name.ns.sym.to_string(), namespaced_name.name.sym.to_string())
      }
    };
    let qual_name = create_qualame(name.as_str());
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
                  JSXExpr::JSXEmptyExpr(empty_expr) => "{{}}".to_string(),
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
              JSXAttrValue::JSXElement(jsx_element) => {
                "".to_string()
              },
              JSXAttrValue::JSXFragment(jsx_fragment) => {
                "".to_string()
              }
            }
          },
          None => "".to_string()
        };
        attributes.push(Attribute {
          name: create_qualame(name.as_str()),
          value: StrTendril::from(value),
        });
      }
    }
    self.tree.orphan(Node::Element(Element::new(qual_name, attributes)))
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
  // 遍历语法树，查找 JSX 节点
  let mut vistor = JSXVisitor;
  module.visit_with(&mut vistor);

  // 解析 CSS 代码，解析出所有规则
  // let mut input = ParserInput::new(&css);
  // let mut parser = CSSParser::new(&mut input);

  // let mut custom_parser = CSSRuleParser;
  // let style_sheet_parser = StyleSheetParser::new(input, &mut custom_parser);
  // let style_sheet: Vec<_> = style_sheet_parser.collect();
  // println!("{:?}", style_sheet);

  let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
  stylesheet.visit(&mut StyleVisitor).unwrap();
  
}
