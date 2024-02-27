
use std::{
  cell::RefCell,
  collections::{BTreeMap, HashMap, VecDeque},
  hash::{Hash, Hasher},
  rc::Rc,
  vec,
};

use html5ever::{tendril::StrTendril, Attribute};
use indexmap::IndexMap;
use lightningcss::{
  properties::{Property, PropertyId},
  stylesheet::ParserOptions,
};
use swc_common::{Span, DUMMY_SP};
use swc_ecma_ast::{
  CallExpr, Callee, Decl, Expr, ExprOrSpread, Ident, ImportDecl,
  ImportNamedSpecifier, ImportSpecifier, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
  JSXElement, JSXElementName, JSXExpr, JSXExprContainer, JSXFragment, KeyValueProp, Lit, Module,
  ModuleDecl, ModuleItem, Null, ObjectLit, Prop, PropName, PropOrSpread,
  Stmt, Str, ReturnStmt, FnExpr, Function, BlockStmt, FnDecl, IfStmt, VarDecl, BindingIdent, AssignExpr, AssignOp, ExprStmt, MemberProp
};
use swc_ecma_visit::{
  noop_visit_mut_type, noop_visit_type, Visit, VisitAll, VisitAllWith, VisitMut, VisitMutWith,
  VisitWith,
};

use crate::{
  constants::{CALC_DYMAMIC_STYLE, CONVERT_STYLE_PX_FN, INNER_STYLE, INNER_STYLE_DATA, RN_CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN}, react_native::{parse_style_properties::parse_style_properties, rn_style_parser::StyleValue}, scraper::Element, style_propetries::{style_value_type::StyleValueType, traits::ToStyleValue, unit::{Platform, PropertyTuple}}, utils::{
    create_qualname, get_callee_attributes, recursion_jsx_member, to_camel_case, to_kebab_case
  }
};

#[derive(Debug, Clone)]
pub enum JSXElementOrJSXCallee<'a> {
  JSXElement(&'a JSXElement),
  JSXCallee(&'a CallExpr),
}

#[derive(Eq, Clone, Copy, Debug)]
pub struct SpanKey(pub Span);

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
    if ident.sym == INNER_STYLE {
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

  fn create_element(&mut self, jsx_element_or_callee: JSXElementOrJSXCallee) -> Element {
    match jsx_element_or_callee {
      JSXElementOrJSXCallee::JSXElement(&ref jsx_element) => self.create_element_from_jsx(&jsx_element),
      JSXElementOrJSXCallee::JSXCallee(&ref call_expr) => self.create_element_from_call_expr(&call_expr),
    }
  }

  fn create_element_from_jsx (&mut self, jsx_element: &JSXElement) -> Element {
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
              _ => "".to_string(),
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

  fn create_element_from_call_expr (&mut self, jsx_callee: &CallExpr) -> Element {
    let name_expr = &*jsx_callee.args.first().unwrap().expr;
    let name = match name_expr {
      Expr::Lit(lit) => match lit {
        Lit::Str(str) => {
          let name = str.value.to_string();
          name
        }
        _ => String::new(),
      },
      Expr::Ident(ident) => ident.sym.to_string(),
      _ => String::new(),
    };
    let qual_name = create_qualname(name.as_str());
    let attributes = get_callee_attributes(jsx_callee).iter().map(|(key, value)| {
      let value: String = match &**value {
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
      };
      Attribute {
        name: create_qualname(key.as_str()),
        value: StrTendril::from(value.as_str()),
      }
    }).collect::<Vec<_>>();

    Element::new(qual_name, SpanKey(jsx_callee.span), attributes)
  }
}

impl<'a> VisitAll for AstVisitor<'a> {
  noop_visit_type!();

  fn visit_jsx_element(&mut self, jsx: &JSXElement) {
    let element = self.create_element(JSXElementOrJSXCallee::JSXElement(jsx));
    if let JSXElementName::Ident(ident) = &jsx.opening.name {
      let name = ident.sym.to_string();
      // if is_starts_with_uppercase(name.as_str()) {
      //   if self.taro_components.contains(&name) {
      //     self.jsx_record.insert(SpanKey(jsx.span), element);
      //   }
      // } else {
        self.jsx_record.insert(SpanKey(jsx.span), element);
      // }
    }
    jsx.visit_all_children_with(self);
  }

  fn visit_jsx_fragment(&mut self, jsx: &JSXFragment) {
    jsx.visit_all_children_with(self);
  }

  // 兼容 React.createElement 的方式
  fn visit_call_expr(&mut self, call_expr: &CallExpr) {
    if let Callee::Expr(expr) = &call_expr.callee {
      if let Expr::Member(member) = &**expr {
        if let Expr::Ident(ident) = &*member.obj {
          if ident.sym.to_string() == "React" {
            if let MemberProp::Ident(ident) = &member.prop {
              if ident.sym.to_string() == "createElement" {
                let element = self.create_element(JSXElementOrJSXCallee::JSXCallee(call_expr));
                let name_ident = call_expr.args.get(0).unwrap();
                let name = match &*name_ident.expr {
                  Expr::Ident(ident) => ident.sym.to_string(),
                  Expr::Lit(lit) => match lit {
                    Lit::Str(str) => str.value.to_string(),
                    _ => "".to_string(),
                  },
                  _ => "".to_string()
                };

                // if is_starts_with_uppercase(name.as_str()) {
                //   if self.taro_components.contains(&name) {
                //     self.jsx_record.insert(SpanKey(call_expr.span), element);
                //   }
                // } else {
                  if !name.is_empty() {
                    self.jsx_record.insert(SpanKey(call_expr.span), element);
                  }
                // }
              }
            }
          }
        }
      }
    }
    call_expr.visit_all_children_with(self);
  }
}


pub fn parse_style_values(value: Vec<StyleValueType>, platform: Platform) -> Vec<PropOrSpread> {
  
  let mut prop_or_spread = vec![];

  // 使用有序表
  let mut index_map = IndexMap::new();
  
  value.into_iter().for_each(|style_value| {
    let prop = style_value.to_expr(platform.clone());
    match prop {
      PropertyTuple::One(id, expr) => {
        if let Expr::Invalid(_) = expr { return }
        index_map.insert(id, Box::new(expr));
      }
      PropertyTuple::Array(prop_arr) => {
        prop_arr.into_iter().for_each(|(id, expr)| {
          if let Expr::Invalid(_) = expr { return }
          index_map.insert(id, Box::new(expr));
        })
      }
    }
  });

  index_map.into_iter().for_each(|(id, expr)| {
    prop_or_spread.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new(id.into(), swc_common::DUMMY_SP)),
      value: expr,
    }))))
  });

  prop_or_spread
}


// 插入运行时所需的引入
pub fn insert_import_module_decl(module: &mut Module, last_import_index: usize, platform: Platform) -> usize {
  let mut last_index = last_import_index;
  match platform {
    Platform::ReactNative => {
      //   import { StyleSheet } from 'react-native'
      //   import { scalePx2dp, scaleVu2dp } from '@tarojs/runtime-rn'
      //   // 用来标识 rn-runner transformer 是否读写缓存
      //   function ignoreStyleFileCache() {}
      module.body.insert(
        last_index,
        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
          span: DUMMY_SP,
          specifiers: vec![
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new("StyleSheet".into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            })
          ],
          src: Box::new(Str::from("react-native")),
          type_only: false,
          with: None,
        })),
      );
      last_index += 1;
      module.body.insert(
        last_index,
        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
          span: DUMMY_SP,
          specifiers: vec![
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(RN_CONVERT_STYLE_PX_FN.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(RN_CONVERT_STYLE_VU_FN.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            })
          ],
          src: Box::new(Str::from("@tarojs/runtime-rn")),
          type_only: false,
          with: None,
        }))
      );
      last_index += 1;
      module.body.insert(
        last_index,
        ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl {
          ident: Ident::new("ignoreStyleFileCache".into(), DUMMY_SP),
          function: Box::new(Function {
            params: vec![],
            decorators: vec![],
            span: DUMMY_SP,
            body: Some(BlockStmt {
              span: DUMMY_SP,
              stmts: vec![],
            }),
            is_generator: false,
            is_async: false,
            type_params: None,
            return_type: None,
          }),
          declare: false,
        }))
      ));
      last_index += 1;
    },
    Platform::Harmony => {
      module.body.insert(
        last_index,
        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
          span: DUMMY_SP,
          specifiers: vec![
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(CALC_DYMAMIC_STYLE.into(), DUMMY_SP),
              // imported: Some(ModuleExportName::Ident(Ident::new(
              //   "calcDynamicStyle".into(),
              //   DUMMY_SP,
              // ))),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(CONVERT_STYLE_PX_FN.into(), DUMMY_SP),
              // imported: Some(ModuleExportName::Ident(Ident::new(
              //   CONVERT_STYLE_PX_FN.into(),
              //   DUMMY_SP,
              // ))),
              imported: None,
              is_type_only: false,
            })
          ],
          src: Box::new(Str::from("@tarojs/runtime")),
          type_only: false,
          with: None,
        }))
      )
    },
  }
  last_index
}

pub struct ModuleMutVisitor {
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
  pub platform: Platform
}

impl ModuleMutVisitor {
  pub fn new(all_style: Rc<RefCell<HashMap<String, StyleValue>>>, platform: Platform) -> Self {
    ModuleMutVisitor { all_style, platform }
  }
}

impl VisitMut for ModuleMutVisitor {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    let binding = self.all_style.borrow();
    let style_entries: BTreeMap<_, _> = binding.iter().collect();

    let ident  = Ident::new(INNER_STYLE_DATA.into(), DUMMY_SP);

    let identifier = Stmt::Decl(Decl::Var(Box::new(VarDecl {
      span: DUMMY_SP,
      kind: swc_ecma_ast::VarDeclKind::Let,
      declare: false,
      decls: vec![swc_ecma_ast::VarDeclarator {
        span: DUMMY_SP,
        name: swc_ecma_ast::Pat::Ident(BindingIdent {
          id: ident.clone(),
          type_ann: None,
        }),
        init: None,
        definite: false,
      }]
    })));

    let inner_style_func = {

      let style_object = Box::new(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: style_entries
          .iter()
          .map(|(key, value)| {
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Str(Str::from(key.as_str())),
              value: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: parse_style_values(
                  value.to_vec(),
                  self.platform.clone()
                )
              })),
            })))
          })
          .collect::<Vec<PropOrSpread>>()
          .into(),
      }));

      let body = vec![
          // if (__inner_style_data__) return __inner_style_data__
          Stmt::If(IfStmt { 
            span: DUMMY_SP,
            test: Box::new(Expr::Ident(ident.clone())),
            cons: Box::new(
              Stmt::Return(ReturnStmt { 
                span: DUMMY_SP,
                arg: Some(Box::new(Expr::Ident(ident.clone())))
              })
            ),
            alt: None
          }),
          // __inner_style_data__ = { ... }
          Stmt::Expr(
            ExprStmt {
              span: DUMMY_SP,
              expr: Box::new(
                Expr::Assign(AssignExpr { span: DUMMY_SP, op: AssignOp::Assign, 
                  left: swc_ecma_ast::PatOrExpr::Expr(Box::new(Expr::Ident(ident.clone()))), 
                  right: style_object
                })
              )
            }
          ),
          // return __inner_style_data__
          Stmt::Return(ReturnStmt {
            span: DUMMY_SP,
            arg: Some(Box::new(Expr::Ident(ident)))
          })
        ];


      let func = FnExpr {
        ident: Some(Ident::new(INNER_STYLE.into(), DUMMY_SP)) ,
        function: Box::new(Function {
          params: vec![],
          decorators: vec![],
          span: DUMMY_SP,
          body: Some(BlockStmt {
              span: DUMMY_SP,
              stmts: body,
          }),
          is_generator: false,
          is_async: false,
          type_params: None,
          return_type: None,
        })
      };

      Stmt::Decl(Decl::Fn(FnDecl {
        ident: func.ident.clone().unwrap(),
        function: func.function,
        declare: false
      }))
    };

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
    // 插入平台所需的运行时代码， 如： import { calcDynamicStyle } from '@tarojs/runtime'
    last_import_index = insert_import_module_decl(module, last_import_index, self.platform.clone());
  
    let mut var_checker = VarChecker { found: false };
    module.visit_with(&mut var_checker);
    if var_checker.found {
      // 插入代码 let __inner_style_data__;
      module.body.insert(last_import_index, ModuleItem::Stmt(identifier));
      last_import_index += 1;
      // 插入代码 function __inner_style__() { ... }
      module
        .body
        .insert(last_import_index, ModuleItem::Stmt(inner_style_func));
    }
  }
}

pub struct JSXMutVisitor<'i> {
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Property<'i>)>>>>,
  pub platform: Platform
}

impl<'i> JSXMutVisitor<'i> {
  pub fn new(
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Property<'i>)>>>>,
    platform: Platform
  ) -> Self {
    JSXMutVisitor {
      jsx_record,
      style_record,
      platform
    }
  }

  fn check_is_jsx_callee (&self, call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
      if let Expr::Member(member) = &**expr {
        if let Expr::Ident(ident) = &*member.obj {
          if ident.sym.to_string() == "React" {
            if let MemberProp::Ident(ident) = &member.prop {
              if ident.sym.to_string() == "createElement" {
                return true
              }
            }
          }
        }
      }
    }

    return false
  }

  fn get_jsx_element_or_callee_calss_value_and_dynamic_class_bool (&self, jsx_element_or_callee: &JSXElementOrJSXCallee) -> (Option<Expr>, bool) {
    let mut has_dynamic_class = false;
    let mut class_attr_value = None;
    
    match jsx_element_or_callee {
      JSXElementOrJSXCallee::JSXElement(jsx_element) => {
        let attrs = &jsx_element.opening.attrs;
        attrs.iter().for_each(|attr| {
          if let JSXAttrOrSpread::JSXAttr(attr) = attr {
            if let JSXAttrName::Ident(ident) = &attr.name {
              if ident.sym.to_string() == "className" {
                if let Some(value) = &attr.value {
                  match value {
                    JSXAttrValue::JSXExprContainer(expr_container) => {
                      match &expr_container.expr {
                        JSXExpr::Expr(expr) => {
                          class_attr_value = Some((**expr).clone());
                        },
                        _ => ()
                      };
                      has_dynamic_class = true;
                    },
                    JSXAttrValue::Lit(lit) => {
                      class_attr_value = Some(lit.clone().into());
                    },
                    _ => ()
                  }
                }
              }
            }
          }
        });
      },
      JSXElementOrJSXCallee::JSXCallee(call_expr) => {
        let mut attributes = get_callee_attributes(call_expr);
        if let Some(value) = attributes.remove("className") {
          class_attr_value = Some((*value).clone());

          match *value {
            Expr::Lit(lit) => {
              class_attr_value = Some(lit.into());
            },
            _ => {
              has_dynamic_class = true;
            }
          }
        }
      }
    };

    (class_attr_value, has_dynamic_class)
  }

  fn process_attribute_lit_value (&self, lit: &Lit, has_dynamic_class: bool, element: &Element, style_record: &HashMap<SpanKey, Vec<(String, Property<'i>)>>) -> Option<Vec<StyleValueType>> {
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

          return Some(parsed_properties);
        }
      }
      _ => {}
    };

    return None;
  }

  fn process_attribute_expr_value (&self, expr: &mut Expr, has_dynamic_class: bool, element: &Element, style_record: &HashMap<SpanKey, Vec<(String, Property<'i>)>>) -> bool {
    let mut has_dynamic_style = false;
    
    match expr {
      Expr::Object(lit) => {
        if !has_dynamic_class {
          let mut properties = Vec::new();
          // 动态的style属性
          let mut dynamic_properties = Vec::new();
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
                  PropOrSpread::Spread(_) => {
                    has_dynamic_style = true;
                  }
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
                      _ => "".to_string()
                    },
                    _ => {
                      has_dynamic_style = true;
                      dynamic_properties.push(p.clone());
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
                    if value.ne("") {
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
                }
                _ => {}
              },
              PropOrSpread::Spread(_) => {
                has_dynamic_style = true;
              }
            }
          }
          let mut temp_props = vec![];

          if !has_dynamic_style {
            for property in properties.iter() {
              temp_props.push((property.0.to_string(), property.1.clone()));
            }
          }
          temp_props.extend(
            temp_properties
              .iter()
              .map(|(key, value)| (key.to_string(), value.clone())),
          );
          let mut temp_props = parse_style_properties(&temp_props);

          let mut props = parse_style_values(temp_props, self.platform.clone());
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
          props.extend(dynamic_properties);
          lit.props = props;
        }
      }
      _ => {
        has_dynamic_style = true;
      }
    }

    has_dynamic_style
  }
}

impl<'i> VisitMut for JSXMutVisitor<'i> {
  noop_visit_mut_type!();

  fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
    let mut has_style = false;
    let mut has_dynamic_style = false;
    let mut style_attr_value = None;

    if self.check_is_jsx_callee(n) {
      let span_key = SpanKey(n.span);
      if let Some(element) = self.jsx_record.borrow().get(&span_key) {
        let style_record = self.style_record.borrow();
        let jsx_element_or_callee = JSXElementOrJSXCallee::JSXCallee(&n);
        let (class_attr_value, has_dynamic_class) = self.get_jsx_element_or_callee_calss_value_and_dynamic_class_bool(&jsx_element_or_callee);

        if let Some(attr) = n.args.get_mut(1) {
          if let Expr::Object(object) = &mut *attr.expr {
            for prop in object.props.iter_mut() {
              if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(key_value_prop) = &mut **prop {
                  if let PropName::Ident(ident) = &key_value_prop.key {
                    if ident.sym.to_string() == "style" {
                      has_style = true;
                      let expr = &mut *key_value_prop.value;
                      // 只支持值为字符串、对象形式的 style
                      match expr {
                        Expr::Lit(lit) => {
                          let value = self.process_attribute_lit_value(lit, has_dynamic_class, element, &self.style_record.borrow());

                          if let Some(value) = value {
                            key_value_prop.value = Expr::Object(ObjectLit {
                              span: DUMMY_SP,
                              props: parse_style_values(value, self.platform.clone()),
                            }).into();
                          }
                        }
                        _ => {
                          let mut expr = &mut key_value_prop.value;
                          has_dynamic_style = self.process_attribute_expr_value(expr, has_dynamic_class, element, &self.style_record.borrow());
                          style_attr_value = Some((*expr).clone());
                          
                        }
                      };
                    }
                  }
                }
              }
            }
          }
        }

        if !has_dynamic_class && !has_dynamic_style {
          if !has_style {
            if let Some(style_declaration) = style_record.get(&element.span) {
              let parsed_properties = parse_style_properties(&style_declaration);
              let attrs = n.args.get_mut(1);
              if let Some(attr) = attrs {
                if let Expr::Object(object) = &mut *attr.expr {
                  object.props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident::new("style".into(), DUMMY_SP)),
                    value: Box::new(Expr::Object(ObjectLit {
                      span: DUMMY_SP,
                      props: parse_style_values(parsed_properties, self.platform.clone())
                    })),
                  }))));
                }
              }
            }
          }
        } else {
          let fun_call_expr = Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
              CALC_DYMAMIC_STYLE.into(),
              DUMMY_SP,
            )))),
            args: vec![
              ExprOrSpread::from(Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                  INNER_STYLE.into(),
                  DUMMY_SP,
                )))),
                type_args: None,
                args: vec![]
              }))),
              match class_attr_value {
                Some(value) => ExprOrSpread::from(Box::new(value)),
                None => ExprOrSpread::from(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
              },
              match style_attr_value {
                Some(value) => ExprOrSpread::from(value),
                None => ExprOrSpread::from(Box::new(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })))),
              },
            ],
            type_args: None,
          });
          if has_style {
            if let Some(attr) = n.args.get_mut(1) {
              let expr = &mut attr.expr;
              if let Expr::Object(object) = &mut **expr {
                for prop in object.props.iter_mut() {
                  if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(key_value_prop) = &mut **prop {
                      if let PropName::Ident(ident) = &key_value_prop.key {
                        if ident.sym.to_string() == "style" {
                          key_value_prop.value = Box::new(fun_call_expr.clone());
                        }
                      }
                    }
                  }
                }
              }
            }
          } else {
            let attrs = n.args.get_mut(1);
            if let Some(attr) = attrs {
              if let Expr::Object(object) = &mut *attr.expr {
                object.props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("style".into(), DUMMY_SP)),
                  value: Box::new(fun_call_expr),
                }))));
              }
            }
          }
        }
      }
    }

    n.visit_mut_children_with(self);
  }

  fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
    let mut has_style = false;
    let mut has_dynamic_style = false;
    let mut style_attr_value = None;
    let span_key = SpanKey(n.span);

    if let Some(element) = self.jsx_record.borrow().get(&span_key) {
      // 将 style_record 中的样式添加到 JSXElement 的 style 属性中
      let style_record = self.style_record.borrow();
      let jsx_element_or_callee = JSXElementOrJSXCallee::JSXElement(&n);
      let (class_attr_value, has_dynamic_class) = self.get_jsx_element_or_callee_calss_value_and_dynamic_class_bool(&jsx_element_or_callee);

      let attrs = &mut n.opening.attrs;
      for attr in attrs.iter_mut() {
        if let JSXAttrOrSpread::JSXAttr(attr) = attr {
          if let JSXAttrName::Ident(ident) = &attr.name {
            if ident.sym.to_string() == "style" {
              has_style = true;
              // 只支持值为字符串、对象形式的 style
              match &mut attr.value {
                Some(value) => {
                  match value {
                    JSXAttrValue::Lit(lit) => {
                      let value = self.process_attribute_lit_value(lit, has_dynamic_class, element, &style_record);

                      if let Some(value) = value {
                        attr.value = Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                          span: DUMMY_SP,
                          expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                            span: DUMMY_SP,
                            props: parse_style_values(value, self.platform.clone())
                          }))),
                        }));
                      }
                    }
                    JSXAttrValue::JSXExprContainer(expr_container) => {
                      match &mut expr_container.expr {
                        JSXExpr::Expr(expr) => {
                          has_dynamic_style = self.process_attribute_expr_value(expr, has_dynamic_class, element, &style_record);
                          style_attr_value = Some((**expr).clone());
                        }
                        _ => {
                          has_style = false;
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
            let parsed_properties = parse_style_properties(&style_declaration);

            n.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
              span: DUMMY_SP,
              name: JSXAttrName::Ident(Ident::new("style".into(), DUMMY_SP)),
              value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                span: DUMMY_SP,
                expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                  span: DUMMY_SP,
                  props: parse_style_values(parsed_properties, self.platform.clone())
                }))),
              })),
            }));
          }
        }
      } else {
        let fun_call_expr = Expr::Call(CallExpr {
          span: DUMMY_SP,
          callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            CALC_DYMAMIC_STYLE.into(),
            DUMMY_SP,
          )))),
          args: vec![
            ExprOrSpread::from(Box::new(Expr::Call(CallExpr {
              span: DUMMY_SP,
              callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
                INNER_STYLE.into(),
                DUMMY_SP,
              )))),
              type_args: None,
              args: vec![]
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
