use std::{
  cell::RefCell,
  collections::{BTreeMap, HashMap, VecDeque},
  hash::{Hash, Hasher},
  rc::Rc,
  vec,
};

use html5ever::{tendril::StrTendril, Attribute};
use lightningcss::{
  properties::{Property, PropertyId},
  stylesheet::ParserOptions,
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
  VisitWith,
};

use crate::{
  scraper::Element,
  style_parser::{parse_style_properties, StyleValue, StyleValueType, ToExpr},
  utils::{
    create_qualname, is_starts_with_uppercase, recursion_jsx_member, to_camel_case, to_kebab_case,
  },
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

struct VarChecker {
  found: bool,
}

impl Visit for VarChecker {
  noop_visit_type!();

  fn visit_ident(&mut self, ident: &Ident) {
    if ident.sym == "__inner_style__" {
      self.found = true;
    }
  }
}

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

fn properties_to_object_lit_props(
  properties: &BTreeMap<&String, &StyleValueType>,
) -> Vec<PropOrSpread> {
  properties
    .iter()
    .map(|(key, value)| PropOrSpread::Prop(Box::new(Prop::KeyValue(parse_style_kv(key, value)))))
    .collect::<Vec<PropOrSpread>>()
}

pub fn parse_style_kv(key: &str, value: &StyleValueType) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Ident(Ident::new(key.to_string().into(), DUMMY_SP)),
    value: value.to_expr().into(),
  }
}

pub struct ModuleMutVisitor {
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

impl ModuleMutVisitor {
  pub fn new(all_style: Rc<RefCell<HashMap<String, StyleValue>>>) -> Self {
    ModuleMutVisitor { all_style }
  }
}

impl VisitMut for ModuleMutVisitor {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    let binding = self.all_style.borrow();
    let style_entries: BTreeMap<_, _> = binding.iter().collect();
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
              let ordered_value: BTreeMap<_, _> = value.iter().collect();
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Str(Str::from(key.as_str())),
                value: Box::new(Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: ordered_value
                    .iter()
                    .map(|(key, value)| {
                      PropOrSpread::Prop(Box::new(Prop::KeyValue(parse_style_kv(key, value))))
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
    let mut var_checker = VarChecker { found: false };
    module.visit_with(&mut var_checker);
    if var_checker.found {
      // 插入代码 const __inner_style__ = calcDynamicStyle(__inner_style__)
      module
        .body
        .insert(last_import_index, ModuleItem::Stmt(inner_style_stmt));
    }
  }
}

pub struct JSXMutVisitor<'i> {
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, HashMap<String, Property<'i>>>>>,
}

impl<'i> JSXMutVisitor<'i> {
  pub fn new(
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, HashMap<String, Property<'i>>>>>,
  ) -> Self {
    JSXMutVisitor {
      jsx_record,
      style_record,
    }
  }
}

impl<'i> VisitMut for JSXMutVisitor<'i> {
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
                              for (key, value) in style_declaration.iter() {
                                properties.insert(to_camel_case(key, false), value.clone());
                              }
                            }
                            for property in style.iter() {
                              let property = property
                                .split(":")
                                .map(|s| s.to_owned())
                                .collect::<Vec<String>>();
                              if property.len() == 2 {
                                let property_parsed = Property::parse_string(
                                  PropertyId::from(property[0].as_str()),
                                  property[1].as_str(),
                                  ParserOptions::default(),
                                );
                                if property_parsed.is_ok() {
                                  properties.insert(
                                    property[0].clone(),
                                    property_parsed.unwrap().into_owned(),
                                  );
                                }
                              }
                            }
                            let parsed_properties = parse_style_properties(
                              &properties
                                .iter()
                                .map(|(key, value)| (key.to_owned(), value.clone()))
                                .collect::<Vec<_>>(),
                            );
                            let properties_entries: BTreeMap<_, _> =
                              parsed_properties.iter().collect();
                            attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                              span: DUMMY_SP,
                              expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                                span: DUMMY_SP,
                                props: properties_to_object_lit_props(&properties_entries).into(),
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
                                  for declaration in style_declaration.iter() {
                                    let mut has_property = false;
                                    for prop in lit.props.iter_mut() {
                                      match prop {
                                        PropOrSpread::Prop(prop) => match &mut **prop {
                                          Prop::KeyValue(key_value_prop) => {
                                            match &mut key_value_prop.key {
                                              PropName::Ident(ident) => {
                                                ident.sym = to_camel_case(
                                                  ident.sym.to_string().as_str(),
                                                  false,
                                                )
                                                .into();
                                                let property_id = ident.sym.to_string();
                                                if property_id == declaration.0.to_string() {
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
                                let deque = VecDeque::from(lit.props.clone());
                                let mut temp_properties = HashMap::new();
                                for p in deque.iter() {
                                  match p {
                                    PropOrSpread::Prop(prop) => match &**prop {
                                      Prop::KeyValue(key_value_prop) => {
                                        let value = match &*key_value_prop.value {
                                          Expr::Lit(lit) => match lit {
                                            Lit::Str(str) => str.value.to_string(),
                                            Lit::Num(num) => num.to_string(),
                                            _ => "".to_string(),
                                          },
                                          _ => {
                                            has_dynamic_style = true;
                                            "".to_string()
                                          }
                                        };
                                        let name = match &key_value_prop.key {
                                          PropName::Ident(ident) => {
                                            Some(to_kebab_case(ident.sym.to_string().as_str()))
                                          }
                                          PropName::Str(str) => {
                                            Some(to_kebab_case(str.value.to_string().as_str()))
                                          }
                                          _ => None,
                                        };
                                        if let Some(name) = name {
                                          let property_id = PropertyId::from(name.as_str());
                                          let property = Property::parse_string(
                                            property_id,
                                            value.as_str(),
                                            ParserOptions::default(),
                                          );
                                          if property.is_ok() {
                                            temp_properties.insert(
                                              to_camel_case(name.as_str(), false),
                                              property.unwrap().into_owned(),
                                            );
                                          }
                                        }
                                      }
                                      _ => {}
                                    },
                                    PropOrSpread::Spread(_) => {}
                                  }
                                }
                                let mut temp_props = vec![];

                                for property in properties.iter() {
                                  temp_props.push((property.0.to_string(), property.1.clone()));
                                }
                                temp_props.extend(
                                  temp_properties
                                    .iter()
                                    .map(|(key, value)| (key.to_string(), value.clone())),
                                );
                                let mut temp_props = parse_style_properties(&temp_props);

                                let mut props = temp_props
                                  .iter_mut()
                                  .map(|p| {
                                    PropOrSpread::Prop(
                                      Prop::KeyValue(parse_style_kv(p.0, p.1)).into(),
                                    )
                                  })
                                  .collect::<Vec<_>>();
                                props.sort_by(|a, b| {
                                  let a = match a {
                                    PropOrSpread::Prop(prop) => match &**prop {
                                      Prop::KeyValue(key_value_prop) => match &key_value_prop.key {
                                        PropName::Ident(ident) => ident.sym.to_string(),
                                        _ => "".to_string(),
                                      },
                                      _ => "".to_string(),
                                    },
                                    _ => "".to_string(),
                                  };
                                  let b = match b {
                                    PropOrSpread::Prop(prop) => match &**prop {
                                      Prop::KeyValue(key_value_prop) => match &key_value_prop.key {
                                        PropName::Ident(ident) => ident.sym.to_string(),
                                        _ => "".to_string(),
                                      },
                                      _ => "".to_string(),
                                    },
                                    _ => "".to_string(),
                                  };
                                  a.cmp(&b)
                                });
                                lit.props.iter().for_each(|prop| {
                                  if let PropOrSpread::Spread(_) = prop {
                                    props.push(prop.clone())
                                  }
                                });
                                lit.props = props;
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
            for declaration in style_declaration.iter() {
              properties.push(declaration.clone());
            }
            let properties = properties
              .iter()
              .map(|property| (property.0.to_string(), property.1.clone()))
              .collect::<HashMap<_, _>>();
            let parsed_properties = parse_style_properties(
              &properties
                .iter()
                .map(|(key, value)| (key.to_owned(), value.clone()))
                .collect::<Vec<_>>(),
            );
            let properties_entries: BTreeMap<_, _> = parsed_properties.iter().collect();
            if has_empty_style {
              for attr in &mut n.opening.attrs {
                if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                  if let JSXAttrName::Ident(ident) = &attr.name {
                    if ident.sym.to_string() == "style" {
                      attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        span: DUMMY_SP,
                        expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                          span: DUMMY_SP,
                          props: properties_to_object_lit_props(&properties_entries).into(),
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
                    props: properties_to_object_lit_props(&properties_entries).into(),
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
        if has_style {
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
        } else {
          n.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(Ident::new("style".into(), DUMMY_SP)),
            value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
              span: DUMMY_SP,
              expr: JSXExpr::Expr(Box::new(fun_call_expr)),
            })),
          }));
        }
      }
    }
    n.visit_mut_children_with(self);
  }
}
