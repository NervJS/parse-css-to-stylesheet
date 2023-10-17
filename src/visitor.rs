use std::{
  cell::RefCell,
  collections::{HashMap, VecDeque},
  hash::{Hash, Hasher},
  rc::Rc,
};

use html5ever::{tendril::StrTendril, Attribute};
use lightningcss::{
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
};
use swc_common::{Span, DUMMY_SP};
use swc_ecma_ast::{
  BindingIdent, CallExpr, Callee, Decl, Expr, ExprOrSpread, Ident, ImportDecl,
  ImportNamedSpecifier, ImportSpecifier, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
  JSXElement, JSXElementName, JSXExpr, JSXExprContainer, JSXFragment, KeyValueProp, Lit, Module,
  ModuleDecl, ModuleExportName, ModuleItem, Null, ObjectLit, Pat, Prop, PropName, PropOrSpread,
  Stmt, Str, VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecma_visit::{
  noop_visit_mut_type, noop_visit_type, Visit, VisitAll, VisitAllWith, VisitMut, VisitMutWith,
};

use crate::{
  scraper::Element,
  style_parser::StyleDeclaration,
  utils::{create_qualname, is_starts_with_uppercase, recursion_jsx_member, to_camel_case},
};

#[derive(Eq, Clone, Copy, Debug)]
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

pub type JSXRecord = HashMap<SpanKey, Element>;

pub struct CollectVisitor {
  pub taro_components: Vec<String>,
}

impl CollectVisitor {
  pub fn new() -> Self {
    CollectVisitor {
      taro_components: vec![],
    }
  }
}

impl Visit for CollectVisitor {
  fn visit_import_decl(&mut self, n: &ImportDecl) {
    if n.src.value.to_string().starts_with("@tarojs/components") {
      for specifier in &n.specifiers {
        match specifier {
          ImportSpecifier::Named(named_specifier) => self
            .taro_components
            .push(named_specifier.local.sym.to_string()),
          _ => {}
        }
      }
    }
  }
}

pub struct AstVisitor<'a> {
  pub taro_components: &'a [String],
  pub jsx_record: &'a mut JSXRecord,
}

impl<'a> AstVisitor<'a> {
  pub fn new(jsx_record: &'a mut JSXRecord, taro_components: &'a [String]) -> Self {
    AstVisitor {
      taro_components,
      jsx_record,
    }
  }

  fn create_element(&mut self, jsx_element: &JSXElement) -> Element {
    let name = match &jsx_element.opening.name {
      JSXElementName::Ident(ident) => ident.sym.to_string(),
      JSXElementName::JSXMemberExpr(expr) => recursion_jsx_member(expr),
      JSXElementName::JSXNamespacedName(namespaced_name) => {
        format!(
          "{}:{}",
          namespaced_name.ns.sym.to_string(),
          namespaced_name.name.sym.to_string()
        )
      }
    };
    let qual_name = create_qualname(name.as_str());
    let mut attributes = Vec::new();
    for attr in &jsx_element.opening.attrs {
      if let JSXAttrOrSpread::JSXAttr(attr) = attr {
        let name = match &attr.name {
          JSXAttrName::Ident(ident) => ident.sym.to_string(),
          JSXAttrName::JSXNamespacedName(namespaced_name) => {
            format!(
              "{}:{}",
              namespaced_name.ns.sym.to_string(),
              namespaced_name.name.sym.to_string()
            )
          }
        };
        let value = match &attr.value {
          Some(value) => match value {
            JSXAttrValue::Lit(lit) => match lit {
              Lit::Str(str) => str.value.to_string(),
              Lit::Num(num) => num.value.to_string(),
              Lit::Bool(bool) => bool.value.to_string(),
              Lit::Null(_) => "null".to_string(),
              Lit::BigInt(bigint) => bigint.value.to_string(),
              Lit::Regex(regex) => regex.exp.to_string(),
              Lit::JSXText(text) => text.value.to_string(),
            },
            JSXAttrValue::JSXExprContainer(expr_container) => match &expr_container.expr {
              JSXExpr::JSXEmptyExpr(_) => "{{}}".to_string(),
              JSXExpr::Expr(expr) => match &**expr {
                Expr::Lit(lit) => match lit {
                  Lit::Str(str) => str.value.to_string(),
                  Lit::Num(num) => num.value.to_string(),
                  Lit::Bool(bool) => bool.value.to_string(),
                  Lit::Null(_) => "null".to_string(),
                  Lit::BigInt(bigint) => bigint.value.to_string(),
                  Lit::Regex(regex) => regex.exp.to_string(),
                  Lit::JSXText(text) => text.value.to_string(),
                },
                _ => "".to_string(),
              },
            },
            JSXAttrValue::JSXElement(_) => "".to_string(),
            JSXAttrValue::JSXFragment(_) => "".to_string(),
          },
          None => "".to_string(),
        };
        attributes.push(Attribute {
          name: create_qualname(name.as_str()),
          value: StrTendril::from(value),
        });
      }
    }
    Element::new(qual_name, SpanKey(jsx_element.span), attributes)
  }
}

impl<'a> VisitAll for AstVisitor<'a> {
  noop_visit_type!();

  fn visit_jsx_element(&mut self, jsx: &JSXElement) {
    let element = self.create_element(jsx);
    if let JSXElementName::Ident(ident) = &jsx.opening.name {
      let name = ident.sym.to_string();
      if is_starts_with_uppercase(name.as_str()) {
        if self.taro_components.contains(&name) {
          self.jsx_record.insert(SpanKey(jsx.span), element);
        }
      } else {
        self.jsx_record.insert(SpanKey(jsx.span), element);
      }
    }
    jsx.visit_all_children_with(self);
  }

  fn visit_jsx_fragment(&mut self, jsx: &JSXFragment) {
    jsx.visit_all_children_with(self);
  }
}

pub struct ModuleMutVisitor<'a> {
  pub all_style: Rc<RefCell<HashMap<String, StyleDeclaration<'a>>>>,
}

impl<'a> ModuleMutVisitor<'a> {
  pub fn new(all_style: Rc<RefCell<HashMap<String, StyleDeclaration<'a>>>>) -> Self {
    ModuleMutVisitor { all_style }
  }
}

impl<'a> VisitMut for ModuleMutVisitor<'a> {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    let binding = self.all_style.borrow();
    let mut style_entries: Vec<_> = binding.iter().collect();
    style_entries.sort_by(|a, b| a.0.cmp(&b.0));
    let inner_style_stmt = Stmt::Decl(Decl::Var(Box::new(VarDecl {
      span: DUMMY_SP,
      kind: VarDeclKind::Var,
      declare: false,
      decls: vec![VarDeclarator {
        span: DUMMY_SP,
        name: Pat::Ident(BindingIdent {
          id: Ident::new("__inner_style__".into(), DUMMY_SP),
          type_ann: None,
        }),
        definite: false,
        init: Some(Box::new(Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: style_entries
            .iter()
            .map(|(key, value)| {
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Str(Str::from(key.as_str())),
                value: Box::new(Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: value
                    .declaration
                    .declarations
                    .iter()
                    .map(|declaration| {
                      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new(
                          to_camel_case(
                            declaration
                              .property_id()
                              .to_css_string(PrinterOptions::default())
                              .unwrap()
                              .as_str(),
                          )
                          .into(),
                          DUMMY_SP,
                        )),
                        value: declaration
                          .value_to_css_string(PrinterOptions {
                            minify: false,
                            targets: Targets {
                              include: Features::HexAlphaColors,
                              ..Targets::default()
                            },
                            ..PrinterOptions::default()
                          })
                          .unwrap()
                          .into(),
                      })))
                    })
                    .collect::<Vec<PropOrSpread>>()
                    .into(),
                })),
              })))
            })
            .collect::<Vec<PropOrSpread>>()
            .into(),
        }))),
      }],
    })));

    // 将 inner_style_stmt 插入到 module 的最后一条 import 语句之后
    let mut last_import_index = 0;
    for (index, stmt) in module.body.iter().enumerate() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(_)) = stmt {
        last_import_index = index;
      }
    }
    if last_import_index != 0 {
      last_import_index += 1;
    }
    // 插入代码 import { calcDynamicStyle } from '@tarojs/runtime'
    module.body.insert(
      last_import_index,
      ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          local: Ident::new("calcDynamicStyle".into(), DUMMY_SP),
          imported: Some(ModuleExportName::Ident(Ident::new(
            "calcDynamicStyle".into(),
            DUMMY_SP,
          ))),
          is_type_only: false,
        })],
        src: Box::new(Str::from("@tarojs/runtime")),
        type_only: false,
        with: None,
      })),
    );
    last_import_index += 1;
    module
      .body
      .insert(last_import_index, ModuleItem::Stmt(inner_style_stmt));
  }
}

pub struct JSXMutVisitor<'a> {
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleDeclaration<'a>>>>,
}

impl<'a> JSXMutVisitor<'a> {
  pub fn new(
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, StyleDeclaration<'a>>>>,
  ) -> Self {
    JSXMutVisitor {
      jsx_record,
      style_record,
    }
  }

  fn properties_to_object_lit_props(
    &self,
    properties: &Vec<(&String, &String)>,
  ) -> Vec<PropOrSpread> {
    properties
      .iter()
      .map(|(key, value)| {
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new(to_camel_case(key.as_str()).into(), DUMMY_SP)),
          value: value.to_string().into(),
        })))
      })
      .collect::<Vec<PropOrSpread>>()
  }
}

impl<'a> VisitMut for JSXMutVisitor<'a> {
  noop_visit_mut_type!();

  fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
    let span_key = SpanKey(n.span);
    if let Some(element) = self.jsx_record.borrow().get(&span_key) {
      // 将 style_record 中的样式添加到 JSXElement 的 style 属性中
      let style_record = self.style_record.borrow();
      let attrs = &mut n.opening.attrs;
      let mut has_style = false;
      let mut has_empty_style = false;
      let mut has_dynamic_style = false;
      let mut class_attr_value = None;
      let mut style_attr_value = None;
      let has_dynamic_class = attrs.iter().any(|attr| {
        if let JSXAttrOrSpread::JSXAttr(attr) = attr {
          if let JSXAttrName::Ident(ident) = &attr.name {
            if ident.sym.to_string() == "className" {
              if let Some(value) = &attr.value {
                if let JSXAttrValue::JSXExprContainer(expr_container) = value {
                  match &expr_container.expr {
                    JSXExpr::JSXEmptyExpr(_) => {}
                    JSXExpr::Expr(expr) => {
                      class_attr_value = Some((**expr).clone());
                    }
                  };
                  return true;
                }
              }
            }
          }
        }
        false
      });
      for attr in attrs {
        if let JSXAttrOrSpread::JSXAttr(attr) = attr {
          if let JSXAttrName::Ident(ident) = &attr.name {
            if ident.sym.to_string() == "style" {
              has_style = true;
              // 只支持值为字符串、对象形式的 style
              match &mut attr.value {
                Some(value) => {
                  match value {
                    JSXAttrValue::Lit(lit) => {
                      match lit {
                        Lit::Str(str) => {
                          if !has_dynamic_class {
                            // 将 style 属性的值转换为对象形式
                            let mut properties = HashMap::new();
                            let style = str.value.to_string();
                            let style = style
                              .split(";")
                              .map(|s| s.to_owned())
                              .collect::<Vec<String>>();
                            if let Some(style_declaration) = style_record.get(&element.span) {
                              for declaration in style_declaration.declaration.declarations.iter() {
                                let property_id = declaration
                                  .property_id()
                                  .to_css_string(PrinterOptions::default())
                                  .unwrap();
                                let property_value = declaration
                                  .value_to_css_string(PrinterOptions {
                                    minify: false,
                                    targets: Targets {
                                      include: Features::HexAlphaColors,
                                      ..Targets::default()
                                    },
                                    ..PrinterOptions::default()
                                  })
                                  .unwrap();
                                properties.insert(property_id, property_value);
                              }
                            }
                            for property in style.iter() {
                              let property = property
                                .split(":")
                                .map(|s| s.to_owned())
                                .collect::<Vec<String>>();
                              if property.len() == 2 {
                                properties.insert(property[0].clone(), property[1].clone());
                              }
                            }
                            let mut properties_entries: Vec<_> = properties.iter().collect();
                            properties_entries.sort_by(|a, b| a.0.cmp(&b.0));
                            attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                              span: DUMMY_SP,
                              expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                                span: DUMMY_SP,
                                props: self
                                  .properties_to_object_lit_props(&properties_entries)
                                  .into(),
                              }))),
                            }));
                          }
                        }
                        _ => {}
                      }
                    }
                    JSXAttrValue::JSXExprContainer(expr_container) => {
                      match &mut expr_container.expr {
                        JSXExpr::JSXEmptyExpr(_) => {
                          has_empty_style = true;
                          has_style = false;
                        }
                        JSXExpr::Expr(expr) => {
                          style_attr_value = Some((**expr).clone());
                          match &mut **expr {
                            Expr::Object(lit) => {
                              if !has_dynamic_class {
                                let mut properties = Vec::new();
                                if let Some(style_declaration) = style_record.get(&element.span) {
                                  for declaration in
                                    style_declaration.declaration.declarations.iter()
                                  {
                                    let mut has_property = false;
                                    for prop in lit.props.iter_mut() {
                                      match prop {
                                        PropOrSpread::Prop(prop) => match &**prop {
                                          Prop::KeyValue(key_value_prop) => {
                                            match &key_value_prop.key {
                                              PropName::Ident(ident) => {
                                                let property_id = ident.sym.to_string();
                                                if property_id
                                                  == declaration
                                                    .property_id()
                                                    .to_css_string(PrinterOptions::default())
                                                    .unwrap()
                                                {
                                                  has_property = true;
                                                  break;
                                                }
                                              }
                                              _ => {}
                                            }
                                          }
                                          _ => {}
                                        },
                                        PropOrSpread::Spread(_) => {}
                                      }
                                    }
                                    if !has_property {
                                      properties.push(declaration.clone());
                                    }
                                  }
                                }
                                let mut deque = VecDeque::from(lit.props.clone());
                                for property in properties.iter() {
                                  deque.push_front(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                    KeyValueProp {
                                      key: PropName::Ident(Ident::new(
                                        to_camel_case(
                                          property
                                            .property_id()
                                            .to_css_string(PrinterOptions::default())
                                            .unwrap()
                                            .as_str(),
                                        )
                                        .into(),
                                        DUMMY_SP,
                                      )),
                                      value: property
                                        .value_to_css_string(PrinterOptions {
                                          minify: false,
                                          targets: Targets {
                                            include: Features::HexAlphaColors,
                                            ..Targets::default()
                                          },
                                          ..PrinterOptions::default()
                                        })
                                        .unwrap()
                                        .into(),
                                    },
                                  ))));
                                }
                                lit.props = deque.into();
                              }
                            }
                            _ => {
                              has_dynamic_style = true;
                            }
                          }
                        }
                      }
                    }
                    JSXAttrValue::JSXElement(_) => {
                      has_dynamic_style = true;
                    }
                    JSXAttrValue::JSXFragment(_) => {
                      has_dynamic_style = true;
                    }
                  }
                }
                None => {
                  has_empty_style = true;
                  has_style = false;
                }
              };
            }
          }
        }
      }

      if !has_dynamic_class && !has_dynamic_style {
        if !has_style {
          if let Some(style_declaration) = style_record.get(&element.span) {
            let mut properties = Vec::new();
            for declaration in style_declaration.declaration.declarations.iter() {
              properties.push(declaration.clone());
            }
            let properties = properties
              .iter()
              .map(|property| {
                (
                  property
                    .property_id()
                    .to_css_string(PrinterOptions::default())
                    .unwrap(),
                  property
                    .value_to_css_string(PrinterOptions {
                      minify: false,
                      targets: Targets {
                        include: Features::HexAlphaColors,
                        ..Targets::default()
                      },
                      ..PrinterOptions::default()
                    })
                    .unwrap(),
                )
              })
              .collect::<HashMap<_, _>>();
            let mut properties_entries: Vec<_> = properties.iter().collect();
            properties_entries.sort_by(|a, b| a.0.cmp(&b.0));
            if has_empty_style {
              for attr in &mut n.opening.attrs {
                if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                  if let JSXAttrName::Ident(ident) = &attr.name {
                    if ident.sym.to_string() == "style" {
                      attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        span: DUMMY_SP,
                        expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                          span: DUMMY_SP,
                          props: self
                            .properties_to_object_lit_props(&properties_entries)
                            .into(),
                        }))),
                      }));
                    }
                  }
                }
              }
            } else {
              n.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new("style".into(), DUMMY_SP)),
                value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                  span: DUMMY_SP,
                  expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: self
                      .properties_to_object_lit_props(&properties_entries)
                      .into(),
                  }))),
                })),
              }));
            }
          }
        }
      } else {
        let fun_call_expr = Expr::Call(CallExpr {
          span: DUMMY_SP,
          callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            "calcDynamicStyle".into(),
            DUMMY_SP,
          )))),
          args: vec![
            ExprOrSpread::from(Box::new(Expr::Ident(Ident {
              span: DUMMY_SP,
              sym: "__inner_style__".into(),
              optional: false,
            }))),
            match class_attr_value {
              Some(value) => ExprOrSpread::from(Box::new(value)),
              None => ExprOrSpread::from(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
            },
            match style_attr_value {
              Some(value) => ExprOrSpread::from(Box::new(value)),
              None => ExprOrSpread::from(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
            },
          ],
          type_args: None,
        });
        for attr in &mut n.opening.attrs {
          if let JSXAttrOrSpread::JSXAttr(attr) = attr {
            if let JSXAttrName::Ident(ident) = &attr.name {
              if ident.sym.to_string() == "style" {
                attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                  span: DUMMY_SP,
                  expr: JSXExpr::Expr(Box::new(fun_call_expr.clone())),
                }));
              }
            }
          }
        }
      }
    }
    n.visit_mut_children_with(self);
  }
}
