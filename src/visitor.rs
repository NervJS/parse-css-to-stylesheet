use ego_tree::{NodeId, Tree, NodeMut, NodeRef};
use html5ever::{Attribute, tendril::StrTendril};
use swc_ecma_ast::{JSXElement, JSXElementName, JSXAttrOrSpread, JSXAttrName, JSXAttrValue, Lit, JSXExpr, Expr, JSXElementChild, Module, Function, Stmt, ExportDefaultExpr, ExportDefaultDecl, DefaultDecl, ClassDecl, ClassMember, PropName, FnDecl, JSXFragment};
use swc_ecma_visit::{Visit, VisitWith};

use crate::{scraper::{Node, Element, Fragment}, utils::{recursion_jsx_member, create_qualname, is_starts_with_uppercase}};

fn recursion_sub_tree<'a>(node: &NodeRef<Node>, current: &mut NodeMut<'a, Node>) {
  for child in node.children() {
    let mut tree_node = current.append(child.value().clone());
    recursion_sub_tree(&child, &mut tree_node);
  }
}

pub struct JSXVisitor<'a> {
  pub tree: &'a mut Tree<Node>,
  pub module: &'a Module,
  pub root_node: Option<NodeId>,
  pub current_node: Option<NodeId>
}

impl<'a> JSXVisitor<'a> {
  pub fn new(tree: &'a mut Tree<Node>, module: &'a Module) -> Self {
    JSXVisitor { tree, module, root_node: None, current_node: None }
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
    let mut elements: Vec<JSXElementChild> = vec![];
    for child in n.iter() {
      match child {
        JSXElementChild::JSXElement(element) => {
          if let JSXElementName::Ident(ident) = &element.opening.name {
            let name = ident.sym.to_string();
            if is_starts_with_uppercase(name.as_str()) {
              let mut visitor = JSXFragmentVisitor::new(self.module, name.as_str());
              self.module.visit_with(&mut visitor);
              let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
              // 将 Fragment 的子节点添加到当前节点
              recursion_sub_tree(&visitor.tree.root(), &mut current);
            } else {
              let node = self.create_element(element);
              let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
              let tree_node = current.append(node);
              nodes.push(tree_node.id());
              elements.push(JSXElementChild::JSXElement(element.clone()));
            }
          } else {
            let node = self.create_element(element);
            let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
            let tree_node = current.append(node);
            nodes.push(tree_node.id());
            elements.push(JSXElementChild::JSXElement(element.clone()));
          }
        },
        JSXElementChild::JSXFragment(fragment) => {
          let node = self.create_fragment();
          let mut current = self.tree.get_mut(self.current_node.unwrap()).unwrap();
          let tree_node = current.append(node);
          nodes.push(tree_node.id());
          elements.push(JSXElementChild::JSXFragment(fragment.clone()));
        },
        _ => {}
      }
    }
    for (index, element) in elements.iter().enumerate() {
      let mut visitor = JSXVisitor::new(self.tree, self.module);
      visitor.current_node = Some(nodes[index]);
      visitor.root_node = self.root_node;
      element.visit_with(&mut visitor);
    }
  }
}

pub struct JSXFragmentVisitor<'a> {
  pub module: &'a Module,
  pub tree: Tree<Node>,
  pub search_fn: &'a str
}

impl<'a> JSXFragmentVisitor<'a> {
  pub fn new(module: &'a Module, search_fn: &'a str) -> Self {
    JSXFragmentVisitor {
      module,
      tree: Tree::new(Node::Fragment(
        Fragment::new(Some(create_qualname(search_fn)))
      )),
      search_fn
    }
  }
}

impl<'a> Visit for JSXFragmentVisitor<'a> {
  fn visit_fn_decl(&mut self, n: &FnDecl) {
    if n.ident.sym.to_string() == self.search_fn {
      match &*n.function {
        Function { body: Some(body), .. } => {
          for stmt in &body.stmts {
            match stmt {
              Stmt::Return(return_stmt) => {
                let mut jsx_visitor = JSXVisitor::new(&mut self.tree, self.module);
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
}

pub struct AstVisitor<'a> {
  pub export_default_name: Option<String>,
  pub module: &'a Module,
  pub tree: &'a mut Tree<Node>
}

impl<'a> AstVisitor<'a> {
  pub fn new(module: &'a Module, tree: &'a mut Tree<Node>) -> Self {
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
                    let mut jsx_visitor = JSXVisitor::new(self.tree, self.module);
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
                                let mut jsx_visitor = JSXVisitor::new(self.tree, self.module);
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
                  let mut jsx_visitor = JSXVisitor::new(self.tree, self.module);
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
                              let mut jsx_visitor = JSXVisitor::new(self.tree, self.module);
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
