use std::{hash::{Hash, Hasher}, collections::HashMap};

use ego_tree::{NodeId, Tree, NodeMut, NodeRef};
use html5ever::{Attribute, tendril::StrTendril};
use swc_common::{Span, DUMMY_SP};
use swc_ecma_ast::{JSXElement, JSXElementName, JSXAttrOrSpread, JSXAttrName, JSXAttrValue, Lit, JSXExpr, Expr, JSXElementChild, Module, Function, Stmt, ExportDefaultExpr, ExportDefaultDecl, DefaultDecl, ClassDecl, ClassMember, PropName, FnDecl, Callee, MemberProp, Str, JSXAttr, Ident};
use swc_ecma_visit::{Visit, VisitWith, VisitMut, noop_visit_type, noop_visit_mut_type, VisitMutWith};

use crate::{scraper::{Node, Element, Fragment}, utils::{recursion_jsx_member, create_qualname, is_starts_with_uppercase, calculate_hash}};

#[derive(Eq, Clone, Debug)]
pub struct SpanKey(Span);

impl PartialEq for SpanKey {
  fn eq(&self, other: &Self) -> bool {
    self.0.lo == other.0.lo && self.0.hi == other.0.hi
  }
}

impl Hash for SpanKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.lo.hash(state);
    self.0.hi.hash(state);
  }
}

pub type JSXRecord = HashMap<SpanKey, NodeId>;

fn recursion_sub_tree<'a>(node: &NodeRef<Node>, current: &mut NodeMut<'a, Node>) {
  for child in node.children() {
    let mut tree_node = current.append(child.value().clone());
    recursion_sub_tree(&child, &mut tree_node);
  }
}

pub struct JSXVisitor<'a> {
  pub tree: &'a mut Tree<Node>,
  pub module: &'a Module,
  pub jsx_record: &'a mut JSXRecord,
  pub root_node: Option<NodeId>,
  pub current_node: Option<NodeId>,
}

impl<'a> JSXVisitor<'a> {
  pub fn new(tree: &'a mut Tree<Node>, module: &'a Module, jsx_record: &'a mut JSXRecord) -> Self {
    JSXVisitor { tree, module, jsx_record, root_node: None, current_node: None }
  }
  fn create_element(&mut self, jsx_element: &JSXElement) -> Node {
    let name = match &jsx_element.opening.name {
      JSXElementName::Ident(ident) => ident.sym.to_string(),
      JSXElementName::JSXMemberExpr(expr) => {
        recursion_jsx_member(expr)
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

  fn create_fragment(&mut self) -> Node {
    Node::Fragment(Fragment::new(Some(create_qualname("__Fragment__"))))
  }
}

impl<'a> Visit for JSXVisitor<'a> {
  noop_visit_type!();

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
      self.jsx_record.insert(SpanKey(jsx.span), current.id());
    }
    jsx.visit_children_with(self)
  }
    
  fn visit_jsx_element_children(&mut self, n: &[JSXElementChild]) {
    let mut nodes = vec![];
    let mut elements: Vec<&JSXElementChild> = vec![];
    for child in n.iter() {
      match child {
        JSXElementChild::JSXElement(element) => {
          if let JSXElementName::Ident(ident) = &element.opening.name {
            let name = ident.sym.to_string();
            if is_starts_with_uppercase(name.as_str()) {
              let mut visitor = JSXFragmentVisitor::new(self.module, self.jsx_record, name.as_str(), SearchType::Normal);
              self.module.visit_with(&mut visitor);
              let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
              // 将 Fragment 的子节点添加到当前节点
              recursion_sub_tree(&visitor.tree.root(), &mut current);
            } else {
              let node = self.create_element(element);
              let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
              let tree_node = current.append(node);
              nodes.push(tree_node.id());
              elements.push(child);
              self.jsx_record.insert(SpanKey(element.span), tree_node.id());
            }
          } else {
            let node = self.create_element(element);
            let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
            let tree_node = current.append(node);
            nodes.push(tree_node.id());
            elements.push(child);
            self.jsx_record.insert(SpanKey(element.span), tree_node.id());
          }
        },
        JSXElementChild::JSXFragment(fragment) => {
          let node = self.create_fragment();
          let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
          let tree_node = current.append(node);
          nodes.push(tree_node.id());
          elements.push(child);
          self.jsx_record.insert(SpanKey(fragment.span), tree_node.id());
        },
        // 找到函数调用中的 JSX
        JSXElementChild::JSXExprContainer(expr) => {
          match &expr.expr {
            JSXExpr::JSXEmptyExpr(_) => {},
            JSXExpr::Expr(expr) => {
              match &**expr {
                Expr::Call(call_expr) => {
                  match &call_expr.callee {
                    Callee::Expr(expr) => {
                      match &**expr {
                        Expr::Ident(ident) => {
                          let name = ident.sym.to_string();
                          let mut visitor = JSXFragmentVisitor::new(self.module, self.jsx_record, name.as_str(), SearchType::Normal);
                          self.module.visit_with(&mut visitor);
                          let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
                          // 将 Fragment 的子节点添加到当前节点
                          recursion_sub_tree(&visitor.tree.root(), &mut current);
                        },
                        Expr::Member(member_expr) => {
                          if let Expr::This(_) = &*member_expr.obj {
                            match &member_expr.prop {
                              MemberProp::Ident(ident) => {
                                let name = ident.sym.to_string();
                                let mut visitor = JSXFragmentVisitor::new(self.module, self.jsx_record, name.as_str(), SearchType::Class);
                                self.module.visit_with(&mut visitor);
                                let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
                                // 将 Fragment 的子节点添加到当前节点
                                recursion_sub_tree(&visitor.tree.root(), &mut current);
                              },
                              _ => {}
                            }
                          }
                        },
                        _ => {}
                      }
                    },
                    _ => {}
                  }
                },
                _ => {}
              }
            },
          }
        },
        _ => {}
      }
    }
    for (index, element) in elements.iter().enumerate() {
      let mut visitor = JSXVisitor::new(self.tree, self.module, self.jsx_record);
      visitor.current_node = Some(nodes[index]);
      visitor.root_node = self.root_node;
      element.visit_with(&mut visitor);
    }
  }
}

#[derive(PartialEq)]
pub enum SearchType {
  Normal,
  Class
}

pub struct JSXFragmentVisitor<'a> {
  pub module: &'a Module,
  pub tree: Tree<Node>,
  pub jsx_record: &'a mut JSXRecord,
  pub search_fn: &'a str,
  pub search_type: SearchType
}

impl<'a> JSXFragmentVisitor<'a> {
  pub fn new(module: &'a Module, jsx_record: &'a mut JSXRecord, search_fn: &'a str, search_type: SearchType) -> Self {
    JSXFragmentVisitor {
      module,
      jsx_record,
      tree: Tree::new(Node::Fragment(
        Fragment::new(Some(create_qualname(search_fn)))
      )),
      search_fn,
      search_type
    }
  }
}

impl<'a> Visit for JSXFragmentVisitor<'a> {
  noop_visit_type!();

  fn visit_fn_decl(&mut self, n: &FnDecl) {
    if n.ident.sym.to_string() == self.search_fn && self.search_type == SearchType::Normal {
      match &*n.function {
        Function { body: Some(body), .. } => {
          for stmt in &body.stmts {
            match stmt {
              Stmt::Return(return_stmt) => {
                let mut jsx_visitor = JSXVisitor::new(&mut self.tree, self.module, self.jsx_record);
                return_stmt.visit_with(&mut jsx_visitor);
              },
              _ => {}
            }
          }
        },
        _ => {}
      }
    }
  }

  fn visit_class_method(&mut self,n: &swc_ecma_ast::ClassMethod) {
    if self.search_type == SearchType::Class {
      match &n.key {
        PropName::Ident(ident) => {
          if ident.sym.to_string() == self.search_fn {
            match &*n.function {
              Function { body: Some(body), .. } => {
                for stmt in &body.stmts {
                  match stmt {
                    Stmt::Return(return_stmt) => {
                      let mut jsx_visitor = JSXVisitor::new(&mut self.tree, self.module, self.jsx_record);
                      return_stmt.visit_with(&mut jsx_visitor);
                    },
                    _ => {}
                  }
                }
              },
              _ => {}
            }
          }
        },
        _ => {}
      }
    }
  }
}

pub struct AstVisitor<'a> {
  pub export_default_name: Option<String>,
  pub module: &'a Module,
  pub tree: &'a mut Tree<Node>,
  pub jsx_record: &'a mut JSXRecord
}

impl<'a> AstVisitor<'a> {
  pub fn new(module: &'a Module, tree: &'a mut Tree<Node>, jsx_record: &'a mut JSXRecord) -> Self {
    AstVisitor { export_default_name: None, module, tree, jsx_record }
  }
}

impl<'a> Visit for AstVisitor<'a> {
  noop_visit_type!();

  fn visit_fn_decl(&mut self, n: &swc_ecma_ast::FnDecl) {
    match &self.export_default_name {
      Some(name) => {
        if n.ident.sym.to_string() == name.as_str() {
          match &*n.function {
            Function { body: Some(body), .. } => {
              for stmt in &body.stmts {
                match stmt {
                  Stmt::Return(return_stmt) => {
                    let mut jsx_visitor = JSXVisitor::new(self.tree, self.module, self.jsx_record);
                    return_stmt.visit_with(&mut jsx_visitor);
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

  fn visit_class_decl(&mut self, n: &ClassDecl) {
    match &self.export_default_name {
      Some(name) => {
        if n.ident.sym.to_string() == name.as_str() {
          for member in &n.class.body {
            match member {
              ClassMember::Method(method) => {
                match &method.key {
                  PropName::Ident(ident) => {
                    if ident.sym.to_string() == "render" {
                      match &*method.function {
                        Function { body: Some(body), .. } => {
                          for stmt in &body.stmts {
                            match stmt {
                              Stmt::Return(return_stmt) => {
                                let mut jsx_visitor = JSXVisitor::new(self.tree, self.module, self.jsx_record);
                                return_stmt.visit_with(&mut jsx_visitor);
                              },
                              _ => {}
                            }
                          }
                        },
                        _ => {}
                      }
                    }
                  },
                  _ => {}
                }
              },
              _ => {}
            }
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

  fn visit_export_default_decl(&mut self, n: &ExportDefaultDecl) {
    match &n.decl {
      DefaultDecl::Fn(n) => {
        match &*n.function {
          Function { body: Some(body), .. } => {
            for stmt in &body.stmts {
              match stmt {
                Stmt::Return(return_stmt) => {
                  let mut jsx_visitor = JSXVisitor::new(self.tree, self.module, self.jsx_record);
                  return_stmt.visit_with(&mut jsx_visitor);
                },
                _ => {}
              }
            }
          },
          _ => {}
        }
      },
      DefaultDecl::Class(n) => {
        for member in &n.class.body {
          match member {
            ClassMember::Method(method) => {
              match &method.key {
                PropName::Ident(ident) => {
                  if ident.sym.to_string() == "render" {
                    match &*method.function {
                      Function { body: Some(body), .. } => {
                        for stmt in &body.stmts {
                          match stmt {
                            Stmt::Return(return_stmt) => {
                              let mut jsx_visitor = JSXVisitor::new(self.tree, self.module, self.jsx_record);
                              return_stmt.visit_with(&mut jsx_visitor);
                            },
                            _ => {}
                          }
                        }
                      },
                      _ => {}
                    }
                  }
                },
                _ => {}
              }
            },
            _ => {}
          }
        }
      },
      _ => {}
    }
  }
}

pub struct AstMutVisitor<'a> {
  pub jsx_record: &'a JSXRecord
}

impl<'a> AstMutVisitor<'a> {
  pub fn new(jsx_record: &'a JSXRecord) -> Self {
    AstMutVisitor { jsx_record }
  }
}

impl<'a> VisitMut for AstMutVisitor<'a> {
  noop_visit_mut_type!();

  fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
    let span_key = SpanKey(n.span);
    if let Some(node_id) = self.jsx_record.get(&span_key) {
      // 在节点上增加 data-styleid 属性，值为 node_id hash 后的值
      let attr_value = JSXAttrValue::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: calculate_hash(node_id).to_string().into(),
        raw: None
      }));
      let attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
        span: DUMMY_SP,
        name: JSXAttrName::Ident(Ident::new("data-styleid".into(), DUMMY_SP)),
        value: Some(attr_value)
      });
      n.opening.attrs.push(attr);
    }
    n.visit_mut_children_with(self);
  }
}
