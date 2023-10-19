use std::{
  cell::RefCell,
  collections::{BTreeMap, HashMap, VecDeque},
  hash::{Hash, Hasher},
  rc::Rc,
};

use html5ever::{tendril::StrTendril, Attribute};
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
  style_parser::{
    BorderRadius, MarginPadding, StyleValue, StyleValueType, TextDecoration, ToObjectExpr,
  },
  utils::{
    create_qualname, delete_items, is_starts_with_uppercase, recursion_jsx_member, to_camel_case,
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
  properties: &BTreeMap<&String, &mut StyleValueType>,
) -> Vec<PropOrSpread> {
  properties
    .iter()
    .map(|(key, value)| PropOrSpread::Prop(Box::new(Prop::KeyValue(parse_style_kv(key, value)))))
    .collect::<Vec<PropOrSpread>>()
}

pub fn parse_style_kv(key: &str, value: &StyleValueType) -> KeyValueProp {
  KeyValueProp {
    key: PropName::Ident(Ident::new(key.to_string().into(), DUMMY_SP)),
    value: match value {
      StyleValueType::Normal(value) => value.to_string().into(),
      StyleValueType::TextDecoration(text_decoration) => text_decoration.to_object_expr().into(),
      StyleValueType::BorderRadius(border_radius) => border_radius.to_object_expr().into(),
      StyleValueType::MarginPadding(margin_padding) => margin_padding.to_object_expr().into(),
    },
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
    module
      .body
      .insert(last_import_index, ModuleItem::Stmt(inner_style_stmt));
  }
}

pub struct JSXMutVisitor {
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleValue>>>,
}

impl JSXMutVisitor {
  pub fn new(
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, StyleValue>>>,
  ) -> Self {
    JSXMutVisitor {
      jsx_record,
      style_record,
    }
  }
}

impl VisitMut for JSXMutVisitor {
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
                                properties.insert(
                                  property[0].clone(),
                                  StyleValueType::Normal(property[1].clone()),
                                );
                              }
                            }
                            let color = properties.get("color").cloned();
                            let properties_entries: BTreeMap<_, _> = properties
                              .iter_mut()
                              .map(|(key, value)| {
                                if key == "textDecoration" {
                                  if let Some(color) = &color {
                                    match value {
                                      StyleValueType::TextDecoration(text_decoration) => {
                                        text_decoration.color = color.to_string();
                                      }
                                      _ => {}
                                    }
                                  }
                                }
                                (key, value)
                              })
                              .collect();
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
                                let mut deque = VecDeque::from(lit.props.clone());
                                for property in properties.iter() {
                                  deque.push_front(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                    KeyValueProp {
                                      key: PropName::Ident(Ident::new(
                                        property.0.to_string().into(),
                                        DUMMY_SP,
                                      )),
                                      value: property.1.to_string().into(),
                                    },
                                  ))));
                                }
                                let color = deque.iter().fold(None, |c, p| {
                                  let color = match p {
                                    PropOrSpread::Prop(prop) => match &**prop {
                                      Prop::KeyValue(key_value_prop) => match &key_value_prop.key {
                                        PropName::Ident(ident) => {
                                          if ident.sym.to_string() == "color" {
                                            Some(key_value_prop.value.clone())
                                          } else {
                                            c
                                          }
                                        }
                                        _ => c,
                                      },
                                      _ => c,
                                    },
                                    _ => c,
                                  };
                                  color
                                });
                                let mut margin = None;
                                let mut margin_index = None;
                                let mut margin_left = None;
                                let mut margin_left_index = None;
                                let mut margin_right = None;
                                let mut margin_right_index = None;
                                let mut margin_top = None;
                                let mut margin_top_index = None;
                                let mut margin_bottom = None;
                                let mut margin_bottom_index = None;
                                let mut padding = None;
                                let mut padding_index = None;
                                let mut padding_left = None;
                                let mut padding_left_index = None;
                                let mut padding_right = None;
                                let mut padding_right_index = None;
                                let mut padding_top = None;
                                let mut padding_top_index = None;
                                let mut padding_bottom = None;
                                let mut padding_bottom_index = None;
                                let mut border_radius = None;
                                let mut border_radius_index = None;
                                let mut border_radius_top_left = None;
                                let mut border_radius_top_left_index = None;
                                let mut border_radius_top_right = None;
                                let mut border_radius_top_right_index = None;
                                let mut border_radius_bottom_left = None;
                                let mut border_radius_bottom_left_index = None;
                                let mut border_radius_bottom_right = None;
                                let mut border_radius_bottom_right_index = None;
                                deque.iter_mut().enumerate().for_each(|(index, p)| match p {
                                  PropOrSpread::Prop(prop) => match &**prop {
                                    Prop::KeyValue(key_value_prop) => {
                                      let value = match &*key_value_prop.value {
                                        Expr::Lit(lit) => match lit {
                                          Lit::Str(str) => {
                                            to_camel_case(str.value.to_string().as_str(), true)
                                          }
                                          _ => "".to_string(),
                                        },
                                        _ => {
                                          has_dynamic_style = true;
                                          "".to_string()
                                        }
                                      };
                                      let name = match &key_value_prop.key {
                                        PropName::Ident(ident) => {
                                          Some(to_camel_case(ident.sym.to_string().as_str(), false))
                                        }
                                        PropName::Str(str) => {
                                          Some(to_camel_case(str.value.to_string().as_str(), false))
                                        }
                                        _ => None,
                                      };
                                      if let Some(name) = name {
                                        if name == "margin" {
                                          margin = Some(value);
                                          margin_index = Some(index);
                                        } else if name == "marginLeft" {
                                          margin_left = Some(value);
                                          margin_left_index = Some(index);
                                        } else if name == "marginRight" {
                                          margin_right = Some(value);
                                          margin_right_index = Some(index);
                                        } else if name == "marginTop" {
                                          margin_top = Some(value);
                                          margin_top_index = Some(index);
                                        } else if name == "marginBottom" {
                                          margin_bottom = Some(value);
                                          margin_bottom_index = Some(index);
                                        } else if name == "padding" {
                                          padding = Some(value);
                                          padding_index = Some(index);
                                        } else if name == "paddingLeft" {
                                          padding_left = Some(value);
                                          padding_left_index = Some(index);
                                        } else if name == "paddingRight" {
                                          padding_right = Some(value);
                                          padding_right_index = Some(index);
                                        } else if name == "paddingTop" {
                                          padding_top = Some(value);
                                          padding_top_index = Some(index);
                                        } else if name == "paddingBottom" {
                                          padding_bottom = Some(value);
                                          padding_bottom_index = Some(index);
                                        } else if name == "borderRadius" {
                                          border_radius = Some(value);
                                          border_radius_index = Some(index);
                                        } else if name == "borderTopLeftRadius" {
                                          border_radius_top_left = Some(value);
                                          border_radius_top_left_index = Some(index);
                                        } else if name == "borderTopRightRadius" {
                                          border_radius_top_right = Some(value);
                                          border_radius_top_right_index = Some(index);
                                        } else if name == "borderBottomLeftRadius" {
                                          border_radius_bottom_left = Some(value);
                                          border_radius_bottom_left_index = Some(index);
                                        } else if name == "borderBottomRightRadius" {
                                          border_radius_bottom_right = Some(value);
                                          border_radius_bottom_right_index = Some(index);
                                        }
                                      }
                                    }
                                    _ => {}
                                  },
                                  PropOrSpread::Spread(_) => {}
                                });
                                let mut margin = match &margin {
                                  Some(margin) => MarginPadding::from(margin.to_string().as_str()),
                                  None => MarginPadding::from("0"),
                                };
                                if let Some(margin_left) = &margin_left {
                                  margin.set_left(margin_left.to_string().as_str());
                                }
                                if let Some(margin_right) = &margin_right {
                                  margin.set_right(margin_right.to_string().as_str());
                                }
                                if let Some(margin_top) = &margin_top {
                                  margin.set_top(margin_top.to_string().as_str());
                                }
                                if let Some(margin_bottom) = &margin_bottom {
                                  margin.set_bottom(margin_bottom.to_string().as_str());
                                }

                                let mut padding = match &padding {
                                  Some(padding) => {
                                    MarginPadding::from(padding.to_string().as_str())
                                  }
                                  None => MarginPadding::from("0"),
                                };

                                if let Some(padding_left) = &padding_left {
                                  padding.set_left(padding_left.to_string().as_str());
                                }
                                if let Some(padding_right) = &padding_right {
                                  padding.set_right(padding_right.to_string().as_str());
                                }
                                if let Some(padding_top) = &padding_top {
                                  padding.set_top(padding_top.to_string().as_str());
                                }
                                if let Some(padding_bottom) = &padding_bottom {
                                  padding.set_bottom(padding_bottom.to_string().as_str());
                                }

                                let mut border_radius = match &border_radius {
                                  Some(border_radius) => {
                                    BorderRadius::from(border_radius.to_string().as_str())
                                  }
                                  None => BorderRadius::from("0"),
                                };

                                if let Some(border_radius_top_left) = &border_radius_top_left {
                                  border_radius
                                    .set_top_left(border_radius_top_left.to_string().as_str());
                                }
                                if let Some(border_radius_top_right) = &border_radius_top_right {
                                  border_radius
                                    .set_top_right(border_radius_top_right.to_string().as_str());
                                }
                                if let Some(border_radius_bottom_left) = &border_radius_bottom_left
                                {
                                  border_radius.set_bottom_left(
                                    border_radius_bottom_left.to_string().as_str(),
                                  );
                                }
                                if let Some(border_radius_bottom_right) =
                                  &border_radius_bottom_right
                                {
                                  border_radius.set_bottom_right(
                                    border_radius_bottom_right.to_string().as_str(),
                                  );
                                }

                                let mut props = deque
                                  .iter_mut()
                                  .map(|p| {
                                    match p {
                                      PropOrSpread::Prop(prop) => match &mut **prop {
                                        Prop::KeyValue(key_value_prop) => {
                                          let value = match &*key_value_prop.value {
                                            Expr::Lit(lit) => match lit {
                                              Lit::Str(str) => {
                                                to_camel_case(str.value.to_string().as_str(), true)
                                              }
                                              _ => "".to_string(),
                                            },
                                            _ => {
                                              has_dynamic_style = true;
                                              "".to_string()
                                            }
                                          };
                                          let name = match &key_value_prop.key {
                                            PropName::Ident(ident) => Some(to_camel_case(
                                              ident.sym.to_string().as_str(),
                                              false,
                                            )),
                                            PropName::Str(str) => Some(to_camel_case(
                                              str.value.to_string().as_str(),
                                              false,
                                            )),
                                            _ => None,
                                          };
                                          if let Some(name) = name {
                                            if name == "textDecoration" {
                                              if !has_dynamic_style {
                                                let mut text_decoration =
                                                  TextDecoration::from(value.to_string().as_str());
                                                text_decoration.change_color(
                                                  match &color {
                                                    Some(color) => match &**color {
                                                      Expr::Lit(lit) => match lit {
                                                        Lit::Str(str) => str.value.to_string(),
                                                        _ => "black".to_string(),
                                                      },
                                                      _ => "black".to_string(),
                                                    },
                                                    None => "black".to_string(),
                                                  }
                                                  .as_str(),
                                                );
                                                key_value_prop.value =
                                                  text_decoration.to_object_expr().into();
                                              }
                                            }
                                          }
                                        }
                                        _ => {}
                                      },
                                      PropOrSpread::Spread(_) => {}
                                    }
                                    (*p).clone()
                                  })
                                  .collect::<Vec<_>>();
                                if !has_dynamic_style {
                                  let mut delete_indexs = vec![];

                                  if let Some(margin_index) = margin_index {
                                    delete_indexs.push(margin_index);
                                  }
                                  if let Some(margin_left_index) = margin_left_index {
                                    delete_indexs.push(margin_left_index);
                                  }
                                  if let Some(margin_right_index) = margin_right_index {
                                    delete_indexs.push(margin_right_index);
                                  }
                                  if let Some(margin_top_index) = margin_top_index {
                                    delete_indexs.push(margin_top_index);
                                  }
                                  if let Some(margin_bottom_index) = margin_bottom_index {
                                    delete_indexs.push(margin_bottom_index);
                                  }

                                  if let Some(padding_index) = padding_index {
                                    delete_indexs.push(padding_index);
                                  }
                                  if let Some(padding_left_index) = padding_left_index {
                                    delete_indexs.push(padding_left_index);
                                  }
                                  if let Some(padding_right_index) = padding_right_index {
                                    delete_indexs.push(padding_right_index);
                                  }
                                  if let Some(padding_top_index) = padding_top_index {
                                    delete_indexs.push(padding_top_index);
                                  }
                                  if let Some(padding_bottom_index) = padding_bottom_index {
                                    delete_indexs.push(padding_bottom_index);
                                  }

                                  if let Some(border_radius_index) = border_radius_index {
                                    delete_indexs.push(border_radius_index);
                                  }
                                  if let Some(border_radius_top_left_index) =
                                    border_radius_top_left_index
                                  {
                                    delete_indexs.push(border_radius_top_left_index);
                                  }
                                  if let Some(border_radius_top_right_index) =
                                    border_radius_top_right_index
                                  {
                                    delete_indexs.push(border_radius_top_right_index);
                                  }
                                  if let Some(border_radius_bottom_left_index) =
                                    border_radius_bottom_left_index
                                  {
                                    delete_indexs.push(border_radius_bottom_left_index);
                                  }
                                  if let Some(border_radius_bottom_right_index) =
                                    border_radius_bottom_right_index
                                  {
                                    delete_indexs.push(border_radius_bottom_right_index);
                                  }

                                  delete_items(&mut props, delete_indexs);

                                  if !margin.is_zero() {
                                    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                      KeyValueProp {
                                        key: PropName::Ident(Ident::new("margin".into(), DUMMY_SP)),
                                        value: margin.to_object_expr().into(),
                                      },
                                    ))));
                                  }
                                  if !padding.is_zero() {
                                    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                      KeyValueProp {
                                        key: PropName::Ident(Ident::new(
                                          "padding".into(),
                                          DUMMY_SP,
                                        )),
                                        value: padding.to_object_expr().into(),
                                      },
                                    ))));
                                  }
                                  if !border_radius.is_zero() {
                                    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(
                                      KeyValueProp {
                                        key: PropName::Ident(Ident::new(
                                          "borderRadius".into(),
                                          DUMMY_SP,
                                        )),
                                        value: border_radius.to_object_expr().into(),
                                      },
                                    ))));
                                  }
                                }
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
            let mut properties = properties
              .iter()
              .map(|property| (property.0.to_string(), property.1.clone()))
              .collect::<HashMap<_, _>>();
            let properties_entries: BTreeMap<_, _> = properties.iter_mut().collect();
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
