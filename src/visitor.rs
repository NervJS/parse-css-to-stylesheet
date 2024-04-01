
use std::{
  cell::RefCell, collections::{BTreeMap, HashMap}, hash::{Hash, Hasher}, rc::Rc, vec
};

use html5ever::{tendril::StrTendril, Attribute};
use indexmap::IndexMap;
use lightningcss::properties::Property;
use swc_core::{
  atoms::Atom, common::{Span, DUMMY_SP}, ecma::{
    utils::quote_ident,
    visit::{
      noop_visit_mut_type, noop_visit_type, Visit, VisitAll, VisitAllWith, VisitMut, VisitMutWith,
      VisitWith,
    }
  }
};
use swc_core::ecma::ast::*;

use crate::{
  constants::{CALC_STATIC_STYLE, COMBINE_NESTING_STYLE, CONVERT_STYLE_PX_FN, CSS_VAR_FN, HM_STYLE, INNER_STYLE, INNER_STYLE_DATA, NESTING_STYLE, NESTINT_STYLE_DATA, RN_CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN, SUPPORT_PSEUDO_KEYS}, scraper::Element, style_parser::StyleValue, style_propetries::{style_value_type::StyleValueType, traits::ToStyleValue, unit::{Platform, PropertyTuple}}, utils::{
    create_qualname, get_callee_attributes, is_starts_with_uppercase, prefix_style_key, recursion_jsx_member, split_selector, TSelector
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

  fn find_jsx_callee_ident_name (&mut self, ident: &Ident, call_expr: &CallExpr) -> Option<(String, Element)> {
    let name;
    let element;
    if ident.sym.to_string() == "createElement" {
      element = self.create_element(JSXElementOrJSXCallee::JSXCallee(call_expr));
      let name_ident = call_expr.args.get(0).unwrap();
      name = match &*name_ident.expr {
        Expr::Ident(ident) => ident.sym.to_string(),
        Expr::Lit(lit) => match lit {
          Lit::Str(str) => str.value.to_string(),
          _ => "".to_string(),
        },
        _ => "".to_string()
      };
      return Some((name, element))
    };
    None
  }
}

impl<'a> VisitAll for AstVisitor<'a> {
  noop_visit_type!();

  fn visit_jsx_element(&mut self, jsx: &JSXElement) {
    let element = self.create_element(JSXElementOrJSXCallee::JSXElement(jsx));
    if let JSXElementName::Ident(_) = &jsx.opening.name {
      // let name = ident.sym.to_string();
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
      match &**expr {
        Expr::Member(member) => {
          if let MemberProp::Ident(ident) = &member.prop {
            if let Some((name, element)) = self.find_jsx_callee_ident_name(ident, call_expr) {
              if !name.is_empty() {
                self.jsx_record.insert(SpanKey(call_expr.span), element);
              }
            }
          }
        },
        Expr::Ident(ident) => {
          if let Some((name, element)) = self.find_jsx_callee_ident_name(ident, call_expr) {
            if !name.is_empty() {
              self.jsx_record.insert(SpanKey(call_expr.span), element);
            }
          }
        },
        _ => {}
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
        index_map.insert(prefix_style_key(id, platform.clone()), Box::new(expr));
      }
      PropertyTuple::Array(prop_arr) => {
        prop_arr.into_iter().for_each(|(id, expr)| {
          if let Expr::Invalid(_) = expr { return }
          index_map.insert(prefix_style_key(id, platform.clone()), Box::new(expr));
        })
      }
    }
  });

  index_map.into_iter().for_each(|(id, expr)| {
    prop_or_spread.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
      key: PropName::Ident(Ident::new(id.into(), DUMMY_SP)),
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
          phase: Default::default(),
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
          phase: Default::default(),
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
          phase: Default::default(),
          specifiers: vec![
            // ImportSpecifier::Named(ImportNamedSpecifier {
            //   span: DUMMY_SP,
            //   local: Ident::new(CALC_DYMAMIC_STYLE.into(), DUMMY_SP),
            //   imported: None,
            //   is_type_only: false,
            // }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(CALC_STATIC_STYLE.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(CONVERT_STYLE_PX_FN.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(COMBINE_NESTING_STYLE.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new(CSS_VAR_FN.into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
            ImportSpecifier::Named(ImportNamedSpecifier {
              span: DUMMY_SP,
              local: Ident::new("globalCss".into(), DUMMY_SP),
              imported: None,
              is_type_only: false,
            }),
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
  pub platform: Platform,
  pub is_enable_nesting: bool,
}

impl ModuleMutVisitor {
  pub fn new(
    all_style: Rc<RefCell<HashMap<String, StyleValue>>>, 
    platform: Platform, 
    is_enable_nesting: bool
  ) -> Self {
    ModuleMutVisitor { all_style, platform, is_enable_nesting }
  }
}

impl ModuleMutVisitor {
  fn get_nesting_visitor (&self) -> impl VisitMut {
    struct MyVisitor {
      is_enable_nesting: bool
    }
    impl MyVisitor {
      fn new (is_enable_nesting: bool) -> Self {
        MyVisitor {
          is_enable_nesting
        }
      }
    }
    impl VisitMut for MyVisitor {
      fn visit_mut_function(&mut self, _: &mut Function) {}
      fn visit_mut_arrow_expr(&mut self, _: &mut ArrowExpr) {}
      fn visit_mut_return_stmt(&mut self, stmt: &mut ReturnStmt) {
        let arg = &mut stmt.arg;
        if let Some(expr_in_box) = arg {
          let is_return_jsx_like = match &mut **expr_in_box {
            Expr::JSXElement(_) |
            Expr::JSXFragment(_) |
            Expr::JSXMember(_) => {
              true
            },
            _ => false
          };
          if is_return_jsx_like {
            let expr = arg.take().unwrap();
            *arg = Some(Box::new(Expr::Call(CallExpr {
              span: DUMMY_SP,
              callee: Callee::Expr(Box::new(Expr::Ident(quote_ident!(COMBINE_NESTING_STYLE)))),
              args: vec![
                ExprOrSpread { expr, spread: None },
                ExprOrSpread { 
                  expr: Box::new(
                    match self.is_enable_nesting {
                      true => Expr::Call(CallExpr {
                        span: DUMMY_SP,
                        callee: Callee::Expr(Box::new(Expr::Ident(quote_ident!(NESTING_STYLE)))),
                        args: vec![],
                        type_args: None
                      }),
                      false => Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))
                    }
                  ), 
                  spread: None }
              ],
              type_args: None,
            })))
          }
        }
      }
      fn visit_mut_call_expr(&mut self,n: &mut CallExpr) {
        n.args.iter_mut().for_each(|arg| {
          arg.expr.visit_mut_children_with(self);
          match &mut *arg.expr {
            Expr::Arrow(arrow) => {
              arrow.body.visit_mut_children_with(self);
            },
            Expr::Fn(func) => {
              func.function.body.visit_mut_children_with(self);
            },
            Expr::Call(call) => {
              call.visit_mut_with(self);
            }
            _ => {}
          }
        });
      }
    }
    MyVisitor::new(self.is_enable_nesting)
  }
  fn enable_nesting_for_class (&self, class: &mut Box<Class>) {
    let render_function = class.body.iter_mut().find(|item| {
      // Todo: support ClassProperty
      if let ClassMember::Method(ClassMethod { key, .. }) = item {
        return key.is_ident() && key.as_ident().unwrap().sym == "render";
      }
      return false;
    });
    if render_function.is_some() {
      let body = &mut render_function.unwrap().as_mut_method().unwrap().function;
      body.visit_mut_children_with(&mut self.get_nesting_visitor());
    };
  }
  fn enable_nesting_for_function (&self, body: &mut Box<Function>) {
    body.visit_mut_children_with(&mut &mut self.get_nesting_visitor());
  }
  fn enable_nesting_for_arrow_function (&self, body: &mut Box<BlockStmtOrExpr>) {
    body.visit_mut_children_with(&mut &mut self.get_nesting_visitor());
  }
  fn enable_nesting_for_call_expr (&self, call: &mut CallExpr) {
    call.visit_mut_with(&mut &mut self.get_nesting_visitor());
  }
}

impl VisitMut for ModuleMutVisitor {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    let binding = self.all_style.borrow_mut();
    let style_entries: BTreeMap<_, _> = binding.iter().collect();

    // __inner_style__普通样式对象
    let mut final_style_entries: BTreeMap<String, Vec<PropOrSpread>> = BTreeMap::new();
    // __nesting_style__嵌套样式对象
    let mut nesting_style_entries: BTreeMap<String, (Vec<TSelector>, Vec<PropOrSpread>)> = BTreeMap::new();
    
    // 合并伪类样式, .pesudo {}、.pesudo:after {}  => .pesudo: { xxx, ["::after"]: {xxx}}
    style_entries.iter().for_each(|(key, value)| {
  
      let mut insert_key = key.to_string();
      let mut insert_value = vec![];

      if (SUPPORT_PSEUDO_KEYS.into_iter().any(|s| key.contains(s))) && self.platform == Platform::Harmony {
        let mut pesudo_key = String::new();
        let key_arr = key.split(":").collect::<Vec<&str>>();
        if key_arr.len() == 2 {
          insert_key = key_arr[0].to_string();
          pesudo_key = key_arr[1].to_string();
        }

        // 插入伪类样式
        let prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
              span: DUMMY_SP,
              value: Atom::from(format!("::{}", pesudo_key)),
              raw: None,
            }))),
          }),
          value: Box::new(Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: parse_style_values(value.to_vec(),self.platform.clone())
          })),
        })));
        insert_value.push(prop)
      } else {
        insert_value = parse_style_values(value.to_vec(),self.platform.clone())
      }

      // 判断是否嵌套样式
      if self.platform == Platform::Harmony {
        if insert_key.contains(" ") || insert_key.chars().filter(|&c| c == '.').count() > 1 {
          // 拆分选择器字符串，安装' ' 或 '>' 拆分，如：container > wrapper item => ['container', '>', 'wrapper', ' ', 'item']
          let selectors = split_selector(insert_key.as_str());

          if let Some(props) = nesting_style_entries.get(&insert_key.clone()) {
            let key = props.0.clone();
            let mut new_insert_value = props.1.clone();
            new_insert_value.extend(insert_value);
            nesting_style_entries.insert(insert_key, (key, new_insert_value));
          } else {
            nesting_style_entries.insert(insert_key, (selectors, insert_value));
          }
          return;
        }
      }

      let _key = insert_key.replace(".", "");
      if let Some(props) = final_style_entries.get(_key.as_str()) {
        let mut new_insert_value = props.clone();
        new_insert_value.extend(insert_value);
        final_style_entries.insert(_key, new_insert_value);
      } else {
        final_style_entries.insert(_key, insert_value);
      }

    });

    // 将 inner_style_stmt 插入到 module 的最后一条 import 语句之后
    let mut last_import_index = 0;
    for (index, stmt) in module.body.iter_mut().enumerate() {
      if let ModuleItem::ModuleDecl(ModuleDecl::Import(_)) = stmt {
        last_import_index = index;
      }
      // 开启层叠功能
      match stmt {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(ExportDefaultDecl { decl, .. })) => {
          match decl {
            // export defualt class Index {}
            DefaultDecl::Class(ClassExpr { class, .. }) => {
              self.enable_nesting_for_class(class);
            },
            // export defualt function Index () {}
            DefaultDecl::Fn(FnExpr { function, ..}) => {
              self.enable_nesting_for_function(function);
            }
            _ => ()
          }
        },
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, .. })) => {
          match decl {
            // export class Index {}
            Decl::Class(ClassDecl { class, .. }) => {
              self.enable_nesting_for_class(class);
            },
            // export function Index () {}
            Decl::Fn(FnDecl { function, ..}) => {
              self.enable_nesting_for_function(function);
            }
            // export const Index = () => {}
            Decl::Var(var_decl) => {
              var_decl.decls.iter_mut().for_each(|decl| {
                match &mut decl.init {
                  Some(init) => {
                    match &mut **init {
                      // export const Index = () => {}
                      Expr::Arrow(ArrowExpr { body, .. }) => {
                        self.enable_nesting_for_arrow_function(body);
                      },
                      // export const Index = withXxxx(() => {})
                      Expr::Call(call) => {
                        self.enable_nesting_for_call_expr(call);
                      },
                      // export const Index = function() {}
                      Expr::Fn(FnExpr { function, .. }) => {
                        self.enable_nesting_for_function(function);
                      },
                      _ => ()
                    }
                  },
                  None => ()
                }
              })
            }
            _ => ()
          }
        },
        ModuleItem::Stmt(Stmt::Decl(Decl::Class(ClassDecl { class, .. }))) => {
          // class Index {}
          self.enable_nesting_for_class(class);
        },
        ModuleItem::Stmt(Stmt::Decl(Decl::Fn(FnDecl { function, .. }))) => {
          // function Index () {}
          self.enable_nesting_for_function(function);
        },
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(var_decl))) => {
          var_decl.decls.iter_mut().for_each(|decl| {
            match &mut decl.init {
              Some(init) => {
                match &mut **init {
                  Expr::Fn(FnExpr { function, .. }) => {
                    // const Index = function () {}
                    self.enable_nesting_for_function(function);
                  },
                  Expr::Arrow(ArrowExpr { body, .. }) => {
                    // const Index = () => {}
                    self.enable_nesting_for_arrow_function(body);
                  },
                  Expr::Call(call) => {
                    // const Index = withStyle()(() => {})
                    self.enable_nesting_for_call_expr(call);
                  },
                  _ => {}
                  // expr => {
                  //   self.enable_nesting_for_expr(expr)
                  // }
                }
              },
              None => (),
            }
          })
        }
        _ => ()
      }
    }
    last_import_index += 1;
    // 插入平台所需的运行时代码， 如： import { calcDynamicStyle } from '@tarojs/runtime'
    last_import_index = insert_import_module_decl(module, last_import_index, self.platform.clone());
    last_import_index += 1;
  
    let mut var_checker = VarChecker { found: false };
    module.visit_with(&mut var_checker);
    if var_checker.found {
      let style_object = Box::new(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: final_style_entries
          .iter()
          .map(|(key, value)| {
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Str(Str::from(key.as_str())),
              value: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: value.to_vec()
              })),
            })))
          })
          .collect::<Vec<PropOrSpread>>()
          .into(),
      }));

      let (identifier, style_func) = generate_stylesheet(INNER_STYLE.to_string(), INNER_STYLE_DATA.to_string(), style_object);
      // 插入代码 let __inner_style_data__;
      module.body.insert(last_import_index, ModuleItem::Stmt(identifier));
      last_import_index += 1;
      // 插入代码 function __inner_style__() { ... }
      module
        .body
        .insert(last_import_index, ModuleItem::Stmt(style_func));
    }

    if self.is_enable_nesting {
      // 插入嵌套样式

      let mut nestings = nesting_style_entries.into_iter().collect::<Vec<(String, (Vec<TSelector>, Vec<PropOrSpread>))>>();
      // 根据类的数量进行权重排序
      nestings.sort_by(|a, b| {
        a.1.0.len().cmp(&b.1.0.len())
      });

      let nesting_style_object = Box::new(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems: nestings.into_iter()
          .map(|(_, value)| {
            Some(ExprOrSpread {
              spread: None,
              expr: Box::new(Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![
                  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str::from("selectors")),
                    value: Box::new(Expr::Array(ArrayLit {
                      span: DUMMY_SP,
                      elems: value.0.iter().map(|prop| {
                        Some(ExprOrSpread { 
                          spread: None, 
                          expr: Box::new(
                            match prop {
                                TSelector::String(str) => Expr::Lit(Lit::Str(Str::from(str.as_str()))),
                                TSelector::Array(arr) => Expr::Array(ArrayLit {
                                  span: DUMMY_SP,
                                  elems: arr.iter().map(|prop| {
                                    Some(ExprOrSpread { 
                                      spread: None, 
                                      expr: Box::new(Expr::Lit(Lit::Str(Str::from(prop.as_str()))))
                                    })
                                  }).collect::<Vec<Option<ExprOrSpread>>>()
                                }),
                            }
                          )
                        })
                      }).collect::<Vec<Option<ExprOrSpread>>>()
                    })),
                  }))),
                  PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Str(Str::from("declaration")),
                    value: Box::new(Expr::Object(ObjectLit {
                      span: DUMMY_SP,
                      props: value.1.to_vec()
                    })),
                  })))

                ]
              }))
            })
          })
          .collect::<Vec<Option<ExprOrSpread>>>()
          .into(),
    }));

    let (identifier, style_func) = generate_stylesheet(NESTING_STYLE.to_string(), NESTINT_STYLE_DATA.to_string(), nesting_style_object);
    // 插入代码 let __inner_style_data__;
    module.body.insert(last_import_index, ModuleItem::Stmt(identifier));
    last_import_index += 1;
    // 插入代码 function __inner_style__() { ... }
    module
      .body
      .insert(last_import_index, ModuleItem::Stmt(style_func))
    }
  }
}

fn generate_stylesheet(fn_name: String, fn_data_name: String, style_object: Box<Expr>) -> (Stmt, Stmt) {

  let ident  = Ident::new(fn_data_name.into(), DUMMY_SP);

  let identifier = Stmt::Decl(Decl::Var(Box::new(VarDecl {
    span: DUMMY_SP,
    kind: VarDeclKind::Let,
    declare: false,
    decls: vec![VarDeclarator {
      span: DUMMY_SP,
      name: Pat::Ident(BindingIdent {
        id: ident.clone(),
        type_ann: None,
      }),
      init: None,
      definite: false,
    }]
  })));

  let inner_style_func = {

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
                left: AssignTarget::Simple(SimpleAssignTarget::Ident(BindingIdent {
                  id: ident.clone(),
                  type_ann: None
                })),
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
      ident: Some(Ident::new(fn_name.into(), DUMMY_SP)) ,
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
  (
    identifier,
    inner_style_func
  )
}

pub enum EtsDirection {
  Row,
  Column,
}
pub struct JSXMutVisitor<'i> {
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>, 
  pub pesudo_style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Vec<(String, Property<'i>)>)>>>>,
  pub taro_components: Vec<String>,
  pub platform: Platform,
  // 半编译模式组件
  is_compile_mode: bool
}

impl<'i> JSXMutVisitor<'i> {
  pub fn new(
    jsx_record: Rc<RefCell<JSXRecord>>,
    all_style: Rc<RefCell<HashMap<String, StyleValue>>>, 
    pesudo_style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Vec<(String, Property<'i>)>)>>>>,
    taro_components: Vec<String>,
    platform: Platform
  ) -> Self {
    JSXMutVisitor {
      jsx_record,
      all_style,
      pesudo_style_record,
      taro_components,
      platform,
      is_compile_mode: false
    }
  }

  fn check_is_jsx_callee (&self, call_expr: &CallExpr) -> bool {
    if let Callee::Expr(expr) = &call_expr.callee {
      if let Expr::Member(member) = &**expr {
        if let MemberProp::Ident(ident) = &member.prop {
          if ident.sym.to_string() == "createElement" {
            return true
          }
        }
      }
      if let Expr::Ident(ident) = &**expr {
        if ident.sym.to_string() == "createElement" {
          return true
        }
      }
    }

    return false
  }

  fn get_jsx_element_or_callee_calss_value_and_dynamic_class_styles (&self, jsx_element_or_callee: &JSXElementOrJSXCallee) -> (Option<Expr>, bool) {
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

  // fn process_attribute_lit_value (&self, lit: &Lit, has_dynamic_class: bool) -> Option<Vec<StyleValueType>> {
  //   match lit {
  //     Lit::Str(str) => {
  //       if !has_dynamic_class {
  //         // 将 style 属性的值转换为对象形式
  //         let mut properties = HashMap::new();
  //         let style = str.value.to_string();
  //         let style = style
  //           .split(";")
  //           .map(|s| s.to_owned())
  //           .collect::<Vec<String>>();
  //         for property in style.iter() {
  //           let property = property
  //             .split(":")
  //             .map(|s| s.to_owned())
  //             .collect::<Vec<String>>();
  //           if property.len() == 2 {
  //             let property_parsed = Property::parse_string(
  //               PropertyId::from(property[0].as_str()),
  //               property[1].as_str(),
  //               ParserOptions::default(),
  //             );
  //             if property_parsed.is_ok() {
  //               properties.insert(
  //                 property[0].clone(),
  //                 property_parsed.unwrap().into_owned(),
  //               );
  //             }
  //           }
  //         }
  //         let parsed_properties = parse_style_properties(
  //           &properties
  //             .iter()
  //             .map(|(key, value)| (key.to_owned(), value.clone()))
  //             .collect::<Vec<_>>()
  //         );

  //         return Some(parsed_properties);
  //       }
  //     }
  //     _ => {}
  //   };

  //   return None;
  // }

  // fn process_attribute_expr_value (&self, expr: &mut Expr, has_dynamic_class: bool) -> (Vec<PropOrSpread>, Vec<PropOrSpread>) {
  //   // 静态的style属性
  //   let mut static_props = vec![];
  //   // 动态的style属性
  //   let mut dynamic_properties = vec![];
  //   match expr {
  //     Expr::Object(lit) => {
  //       if !has_dynamic_class {
  //         let deque = VecDeque::from(lit.props.clone());
  //         let mut temp_properties = HashMap::new();
  //         for p in deque.iter() {
  //           match p {
  //             PropOrSpread::Prop(prop) => match &**prop {
  //               Prop::KeyValue(key_value_prop) => {
  //                 let value = match &*key_value_prop.value {
  //                   Expr::Lit(lit) => match lit {
  //                     Lit::Str(str) => str.value.to_string(),
  //                     Lit::Num(num) => num.to_string(),
  //                     _ => "".to_string()
  //                   },
  //                   _ => {
  //                     dynamic_properties.push(p.clone());
  //                     "".to_string()
  //                   }
  //                 };
  //                 let name = match &key_value_prop.key {
  //                   PropName::Ident(ident) => {
  //                     Some(to_kebab_case(ident.sym.to_string().as_str()))
  //                   }
  //                   PropName::Str(str) => {
  //                     Some(to_kebab_case(str.value.to_string().as_str()))
  //                   }
  //                   _ => None,
  //                 };
  //                 if let Some(name) = name {
  //                   if value.ne("") {
  //                     let property_id = PropertyId::from(name.as_str());
  //                     let property = Property::parse_string(
  //                       property_id,
  //                       value.as_str(),
  //                       ParserOptions::default(),
  //                     );
  //                     if property.is_ok() {
  //                       temp_properties.insert(
  //                         to_camel_case(name.as_str(), false),
  //                         property.unwrap().into_owned(),
  //                       );
  //                     }
  //                   }
  //                 }
  //               }
  //               _ => {}
  //             },
  //             PropOrSpread::Spread(_) => {
  //             }
  //           }
  //         }
  //         let mut temp_props = vec![];

  //         temp_props.extend(
  //           temp_properties
  //             .iter()
  //             .map(|(key, value)| (key.to_string(), value.clone())),
  //         );
  //         let temp_props = parse_style_properties(&temp_props);

  //         static_props = parse_style_values(temp_props, self.platform.clone());
  //         static_props.sort_by(|a, b| {
  //           let a = match a {
  //             PropOrSpread::Prop(prop) => match &**prop {
  //               Prop::KeyValue(key_value_prop) => match &key_value_prop.key {
  //                 PropName::Ident(ident) => ident.sym.to_string(),
  //                 _ => "".to_string(),
  //               },
  //               _ => "".to_string(),
  //             },
  //             _ => "".to_string(),
  //           };
  //           let b = match b {
  //             PropOrSpread::Prop(prop) => match &**prop {
  //               Prop::KeyValue(key_value_prop) => match &key_value_prop.key {
  //                 PropName::Ident(ident) => ident.sym.to_string(),
  //                 _ => "".to_string(),
  //               },
  //               _ => "".to_string(),
  //             },
  //             _ => "".to_string(),
  //           };
  //           a.cmp(&b)
  //         });
  //         lit.props.iter().for_each(|prop| {
  //           if let PropOrSpread::Spread(_) = prop {
  //             static_props.push(prop.clone())
  //           }
  //          });
  //       }
  //     }
  //     _ => {
  //     }
  //   }

  //   (static_props, dynamic_properties)
  // }

  // 半编译模版注入
  // 1、识别静态style的值是否携带flexDirection、display
  // 2、识别静态classname所对应的表里是否携带flexDirection、display
  // 3、识别出最终会计算出的值，设置harmonyDirection
  fn compile_mode_inject (&self, jsx: &mut JSXElement) {
    let binding = self.all_style.borrow();

    let mut has_harmony_direction = false;
    let mut direction = EtsDirection::Column;
    let mut is_flex = false;
    let mut static_styles = vec![];

    // 识别jsx中的classname
    jsx.opening.attrs.iter_mut().for_each(|attr| {
      if let JSXAttrOrSpread::JSXAttr(attr) = attr {
        if let JSXAttrName::Ident(ident) = &attr.name {
          if ident.sym.to_string() == "harmonyDirection" {
            has_harmony_direction = true;
            if let Some(JSXAttrValue::Lit(Lit::Str(Str { value, .. }))) = &attr.value {
              if value == "row" {
                  direction = EtsDirection::Row;
              }
            }
          } else if ident.sym.to_string() == "className" {
            // 遍历attr.value
            if let Some(value) = &attr.value {
              match value {
                // 静态classname
                JSXAttrValue::Lit(Lit::Str(lit_str)) => {
                  // 根据空格分割lit_str
                  let classnames = lit_str.value.split(" ").collect::<Vec<&str>>();
                  // 遍历classnames，查看self.allStyle是否含有该classname
                  classnames.iter().for_each(|classname| {
                    if let Some(style_value) = binding.get(format!(".{}", classname).as_str()) {
                      // 遍历style_value，查看是否含有flexDirection、display
                      for style_value_type in style_value.iter() {
                        match style_value_type {
                          StyleValueType::FlexDirection(flex_direction) => {
                            is_flex = true;
                            // 判断是否为column
                            match flex_direction.value {
                              crate::style_propetries::flex_direction::EnumValue::Column =>  {
                                direction = EtsDirection::Column
                              },
                              crate::style_propetries::flex_direction::EnumValue::Row =>  {
                                direction = EtsDirection::Row
                              },
                              _ => {}
                            }
                          },
                          StyleValueType::Display(display) => {
                            // 判断是否为flex
                            match display.value {
                              crate::style_propetries::display::EnumValue::Flex => {
                                if is_flex == false {
                                  direction = EtsDirection::Row
                                }
                              },
                              _ => {}
                            }
                          },
                          _ => {}
                        }
                      }
                    }
                  });
                },
                _ => {}
              }
            }
          } else if ident.sym.to_string() == "style" {
            // 判断是否是style
            if let Some(JSXAttrValue::JSXExprContainer(jsx_expr_container)) = &attr.value {
              if let JSXExpr::Expr(expr) = &jsx_expr_container.expr {
                match *expr.clone() {
                  Expr::Object(obj) => {
                    static_styles = obj.props.clone();
                  },
                  _ => {}
                }
              }
            }
          }
          
        }
      }
    });
    
    // 识别静态style
    static_styles.into_iter().for_each(|props_or_value| {
      if let PropOrSpread::Prop(prop) = props_or_value {
        if let Prop::KeyValue(key_value) = *prop.clone() {
          // 判断是否是display
          if let PropName::Ident(ident) = key_value.key {
            if ident.sym.to_string() == "display" {
              match *key_value.value {
                Expr::Lit(Lit::Str(str)) => {
                  match str.value.as_str() {
                    "flex" => {
                      if is_flex == false {
                        direction = EtsDirection::Row;
                      }
                    },
                    _ => {}
                  }
                },
                _ => {}
              }
            } else if ident.sym.to_string() == "flexDirection" {
              // 判断是否是flexDirection
              is_flex = true;
              match *key_value.value {
                Expr::Member(member) => {
                  if let Expr::Ident(ident) = *member.obj {
                    if ident.sym.to_string() == "FlexDirection" {
                      match member.prop {
                        MemberProp::Ident(ident) => {
                          match ident.sym.to_string().as_str() {
                            "Column" => {
                              direction = EtsDirection::Column
                            },
                            "Row" => {
                              direction = EtsDirection::Row
                            }
                            _ => {}
                          }
                        },
                        _ => {}
                      }
                    }
                  }
                }
                _ => {}
              }
            }
          }
        }
       }
    });

    // jsx属性插入harmonyDirection
    if !has_harmony_direction {
      let value = match direction {
        EtsDirection::Row => "row",
        EtsDirection::Column => "column"
      };
      jsx.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
        span: DUMMY_SP,
        name: JSXAttrName::Ident(Ident::new("harmonyDirection".into(), DUMMY_SP)),
        value: Some(JSXAttrValue::Lit(Lit::Str(value.into()))),
      }));
    
    }
  }
}

impl<'i> VisitMut for JSXMutVisitor<'i> {
  noop_visit_mut_type!();

  fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
    if self.check_is_jsx_callee(n) {
    
      let span_key = SpanKey(n.span);
      if let Some(_) = self.jsx_record.borrow_mut().get(&span_key) {
        let jsx_element_or_callee = JSXElementOrJSXCallee::JSXCallee(&n);
        let (class_attr_value, _) = self.get_jsx_element_or_callee_calss_value_and_dynamic_class_styles(&jsx_element_or_callee);
        // 插入静态style
        fn get_fun_call_expr (class_attr_value: Expr) -> Expr {
          Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
              CALC_STATIC_STYLE.into(),
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
              ExprOrSpread::from(Box::new(class_attr_value)),
            ],
            type_args: None,
          })
        }
        let expr = n.args.get_mut(0);
        match expr {
          Some(expr_or_spread) => {
            match *expr_or_spread.expr.clone() {
              Expr::Lit(lit) => {
                if let Lit::Str(_) = lit {
                  if let Some(attr) = n.args.get_mut(1) {
                    if let Expr::Object(object) = &mut *attr.expr {
                      let mut should_insert = false;
                      if let Some(_) = &class_attr_value {
                        should_insert = true
                      }
                      if should_insert {
                        object.props.insert(
                          0,
                          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(HM_STYLE.into(), DUMMY_SP)),
                            value: Box::new(get_fun_call_expr(match class_attr_value {
                                Some(value) => value.clone(),
                                None => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                            })),
                          }))),
                        )
                      }
                    }
                  }
                }
               },
              Expr::Ident(ident) => {
                let name = ident.sym.to_string();
                if let Some(attr) = n.args.get_mut(1) {
                  if let Expr::Object(object) = &mut *attr.expr {
                    if 
                      (is_starts_with_uppercase(name.as_str()) && self.taro_components.contains(&name))
                      || !is_starts_with_uppercase(name.as_str())
                    {
                      let mut should_insert = false;
                      if let Some(_) = &class_attr_value {
                        should_insert = true
                      }
                      if should_insert {
                        object.props.insert(
                          0,
                          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(HM_STYLE.into(), DUMMY_SP)),
                            value: Box::new(get_fun_call_expr(match class_attr_value {
                                Some(value) => value.clone(),
                                None => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                            })),
                          }))),
                        )
                      }
                    } else {
                      if let Some(class_attr_value) = class_attr_value {
                        object.props.insert(
                          0,
                          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new("__styleSheet".into(), DUMMY_SP)),
                            value: Box::new(Expr::Object(ObjectLit {
                              span: DUMMY_SP,
                              props: vec![
                                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                  key: PropName::Ident(Ident::new("key".into(), DUMMY_SP)),
                                  value: Box::new(class_attr_value.clone()),
                                }))),
                                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                  key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
                                  value: Box::new(get_fun_call_expr(class_attr_value.clone())),
                                }))),
                              ],
                            })),
                          }))
                        ))
                      };
                    }
                  }
                }
              
              },
              _ => {}
            };
          },
          None => {}
        }
    
        
      }
    }

    n.visit_mut_children_with(self);
  }

  fn visit_mut_jsx_element(&mut self, n: &mut JSXElement) {
    let span_key = SpanKey(n.span);

    if let Some(_) = self.jsx_record.borrow_mut().get(&span_key) {
      // 将 style_record 中的样式添加到 JSXElement 的 style 属性中
      let jsx_element_or_callee = JSXElementOrJSXCallee::JSXElement(&n);
      let (class_attr_value, _) = self.get_jsx_element_or_callee_calss_value_and_dynamic_class_styles(&jsx_element_or_callee);

      let attrs = &mut n.opening.attrs;
      for attr in attrs.iter_mut() {
        if let JSXAttrOrSpread::JSXAttr(attr) = attr {
          if let JSXAttrName::Ident(ident) = &attr.name {
            if ident.sym.to_string() == "compileMode" {
              self.is_compile_mode = true
            }
          }
        }
      }

      // 半编译模式下，识别能识别出来的静态样式，设置harmonyDirection
      if self.is_compile_mode {
        self.compile_mode_inject(n);
      }
      
      // 插入静态style
      fn get_fun_call_expr (class_attr_value: Expr) -> Expr {
        Expr::Call(CallExpr {
          span: DUMMY_SP,
          callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            CALC_STATIC_STYLE.into(),
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
            ExprOrSpread::from(Box::new(class_attr_value))
          ],
          type_args: None,
        })
      }
      
      // 判断是否Taro组件还是自定义组件
      // Taro组件插入__hmStyle__属性
      // 自定义组件插入__styleSheet属性
      if let JSXElementName::Ident(ident) = &n.opening.name {
        let name = ident.sym.to_string();
        if 
          (is_starts_with_uppercase(name.as_str()) && self.taro_components.contains(&name))
          || !is_starts_with_uppercase(name.as_str())
         {
          let mut should_insert = false;
          if let Some(_) = &class_attr_value {
            should_insert = true
          }
          if should_insert {
            n.opening.attrs.insert(
              0,
              JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new(HM_STYLE.into(), DUMMY_SP)),
                value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                  span: DUMMY_SP,
                  expr: JSXExpr::Expr(Box::new(get_fun_call_expr(match class_attr_value {
                      Some(value) => value.clone(),
                      None => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                  }))),
                })),
              }),
            );
          }
        } else {
          match class_attr_value.clone() {
            Some(class_attr_value) => {
              n.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new("__styleSheet".into(), DUMMY_SP)),
                value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                  span: DUMMY_SP,
                  expr: JSXExpr::Expr(Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: vec![
                      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new("key".into(), DUMMY_SP)),
                        value: Box::new(class_attr_value.clone()),
                      }))),
                      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
                        value: Box::new(get_fun_call_expr(class_attr_value.clone())),
                      }))),
                    ],
                  }))),
                })),
              }));
            },
            None => {},
          }
        }
      }
    }
    n.visit_mut_children_with(self);
  }
}
