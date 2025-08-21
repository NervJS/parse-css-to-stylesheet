use indexmap::IndexMap;
use serde_json::Value;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::constants::{Pseudo, SUPPORT_PSEUDO_KEYS};
use crate::parse_style_properties::DeclsAndVars;
use crate::style_propetries::style_value_type::StyleValueType;

use crate::style_parser::{FontFaceItem, KeyFrameItem, RuleItem};
use crate::style_propetries::style_media::StyleMedia;
use crate::style_propetries::unit::Platform;
use crate::visitor::parse_style_values;
use crate::{generate_expr_lit_num, generate_expr_lit_str, utils};

pub struct JsonWriter {
  styles: Vec<RuleItem>,
  keyframes: IndexMap<(u32, String), Vec<KeyFrameItem>>,
  medias: Vec<StyleMedia>,
  fonts: Vec<FontFaceItem>,
  design_width: Option<i32>,
  allow_inherit: Option<bool>,
  design_mode: Option<String>,
}

impl JsonWriter {
  pub fn new(
    styles: Vec<RuleItem>,
    keyframes: IndexMap<(u32, String), Vec<KeyFrameItem>>,
    medias: Vec<StyleMedia>,
    fonts: Vec<FontFaceItem>,
    design_width: Option<i32>,
    allow_inherit: Option<bool>,
    design_mode: Option<String>,
  ) -> Self {
    Self {
      styles,
      keyframes,
      medias,
      fonts,
      design_width,
      allow_inherit,
      design_mode,
    }
  }

  pub fn to_json(&self) -> String {
    let elems: Vec<Expr> = self
      .styles
      .iter()
      .filter_map(|rule_item| {
        Some({
          // 识别伪类
          let mut new_selector = rule_item.selector.selector.clone();
          // 特殊处理:root选择器
          if new_selector != ":root" {
            let key_arr = new_selector.split(":").collect::<Vec<&str>>();
            if key_arr.len() == 2 {
              new_selector = key_arr[0].to_string();
            }
          }
          let nesting_selector = utils::split_selector(&new_selector);
          let mut lit_props = vec![
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("media".into(), DUMMY_SP)),
              value: Box::new(Expr::Lit(Lit::Num(Number::from(rule_item.media as f64)))),
            }))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("selector".into(), DUMMY_SP)),
              value: Box::new(Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: nesting_selector
                  .iter()
                  .map(|t_selector| {
                    match t_selector {
                      // 选择器类型
                      utils::TSelector::Selector(selector_type) => Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Num(Number::from(selector_type.to_f64())))),
                      }),
                      utils::TSelector::String(s) => Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                          span: DUMMY_SP,
                          value: s.clone().into(),
                          raw: None,
                        }))),
                      }),
                      utils::TSelector::Array(arr) => Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Array(ArrayLit {
                          span: DUMMY_SP,
                          elems: arr
                            .iter()
                            .map(|s| {
                              Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                  span: DUMMY_SP,
                                  value: s.clone().into(),
                                  raw: None,
                                }))),
                              })
                            })
                            .collect(),
                        })),
                      }),
                    }
                  })
                  .collect(),
              })),
            }))),
            PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
              key: PropName::Ident(Ident::new("declarations".into(), DUMMY_SP)),
              value: Box::new(Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: parse_style_values(rule_item.declarations.clone(), rule_item.important_declarections.clone(), Platform::Harmony),
              })),
            }))),
          ];
          if rule_item.has_env {
            lit_props.push(
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new("has_env".into(), DUMMY_SP)),
                value: Box::new(Expr::Lit(Lit::Bool(Bool { span: DUMMY_SP, value: rule_item.has_env }))),
              })))
            );
          }
          if rule_item.variables.len() > 0 {
            lit_props.push(
              PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new("variables".into(), DUMMY_SP)),
                value: Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: rule_item.variables.clone().into_iter().map(|css_variable| {
                        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                            key: PropName::Ident(Ident::new(css_variable.id.clone().into(), DUMMY_SP)),
                            value: Box::new(Expr::Lit(Lit::Str(Str {
                                span: DUMMY_SP,
                                value: css_variable.value.clone().into(),
                                raw: None,
                            }))),
                        })))
                    }).collect::<Vec<PropOrSpread>>()
                }))
              })))
            );
          }

          if rule_item.selector.is_pseudo {
            if let Some(pseudo_enum) = &rule_item.selector.pseudo_type {
              if let Pseudo::NthChild(a, b, is_first) = &pseudo_enum {
                let value: String = if *a == 0 && *b == 0 {
                  "".to_string()
                } else if *a == 0 {
                  format!("{}", *b)
                } else if *b == 0 {
                  format!("{}n", *a)
                } else {
                  if *a == 1 {
                    format!("n+{}", *b)
                  } else {
                    format!("{}n+{}", *a, *b)
                  }
                };
                lit_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("pseudo_val".into(), DUMMY_SP)),
                  value: Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: value.into(),
                    raw: None,
                  }))),
                }))));
                lit_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                  key: PropName::Ident(Ident::new("pseudo_key".into(), DUMMY_SP)),
                  value: Box::new(Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: vec![
                      Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Num(Number::from(*a as f64)))),
                      }),
                      Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Num(Number::from(*b as f64)))),
                      }),
                      Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(Expr::Lit(Lit::Bool(Bool { span: DUMMY_SP, value: *is_first }))),
                      }),
                    ],
                  })),
                }))));
              };
              lit_props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                key: PropName::Ident(Ident::new("pseudo".into(), DUMMY_SP)),
                value: Box::new(Expr::Lit(Lit::Num(Number::from(pseudo_enum.to_f64())))),
              }))));
            }
          }
          Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: lit_props,
          })
        })
      })
      .collect();

    // keyframes
    let keyframe_elems: Vec<Expr> = self
      .keyframes
      .iter()
      .map(|((media_index, name), keyframe)| {
        let keyframe_items: Vec<Expr> = keyframe
          .iter()
          .map(|keyframe_item| {
            Expr::Object(ObjectLit {
              span: DUMMY_SP,
              props: keyframe_item.to_expr(),
            })
          })
          .collect();

        let lit_keyframe = vec![
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("name".into(), DUMMY_SP)),
            value: Box::new(generate_expr_lit_str!(name.clone())),
          }))),
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("media".into(), DUMMY_SP)),
            value: Box::new(generate_expr_lit_num!(*media_index as f64)),
          }))),
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("keyframe".into(), DUMMY_SP)),
            value: Box::new(Expr::Array(ArrayLit {
              span: DUMMY_SP,
              elems: keyframe_items
                .into_iter()
                .map(|expr| {
                  Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(expr),
                  })
                })
                .collect(),
            })),
          }))),
        ];
        Expr::Object(ObjectLit {
          span: DUMMY_SP,
          props: lit_keyframe,
        })
      })
      .collect();

    // fonts

    let mut json_value = expr_to_json(&Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: vec![
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("styles".into(), DUMMY_SP)),
          value: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: elems
              .into_iter()
              .map(|expr| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(expr),
                })
              })
              .collect(),
          })),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("keyframes".into(), DUMMY_SP)),
          value: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: keyframe_elems
              .into_iter()
              .map(|expr| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(expr),
                })
              })
              .collect(),
          })),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("medias".into(), DUMMY_SP)),
          value: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: self
              .medias
              .iter()
              .map(|media| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: media.clone().to_expr(),
                  })),
                })
              })
              .collect(),
          })),
        }))),
        PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident::new("fonts".into(), DUMMY_SP)),
          value: Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: self
              .fonts
              .iter()
              .map(|font| {
                Some(ExprOrSpread {
                  spread: None,
                  expr: Box::new(Expr::Object(ObjectLit {
                    span: DUMMY_SP,
                    props: font.clone().to_expr(),
                  })),
                })
              })
              .collect(),
          })),
        }))),
      ],
    }));
    // 如果 design_width 存在，则添加 design_width 到 json_value
    if let Some(design_width) = self.design_width {
      let map = json_value.as_object_mut().unwrap();
      map.insert("design_width".to_string(), Value::Number(serde_json::Number::from(design_width)));
    }
    if let Some(allow_inherit) = self.allow_inherit {
      let map = json_value.as_object_mut().unwrap();
      map.insert("allow_inherit".to_string(), Value::Bool(allow_inherit));
    }
    if let Some(design_mode) = &self.design_mode {
      let map = json_value.as_object_mut().unwrap();
      map.insert("design_mode".to_string(), Value::String(design_mode.clone()));
    }

    // 打印 JSON 值

    // let json_string = serde_json::to_string_pretty(&json_value).unwrap();
    let json_string = serde_json::to_string(&json_value).unwrap();

    json_string
  }
}

// 将 Expr 转换为 JSON Value
fn expr_to_json(value: &Expr) -> Value {
  match value {
    Expr::Lit(Lit::Str(s)) => Value::String(s.value.to_string()),
    Expr::Lit(Lit::Num(n)) => {
      if n.value.fract() == 0.0 {
        // 是整数
        Value::Number(serde_json::Number::from(n.value as i64))
      } else {
        // 是浮点数
        Value::Number(serde_json::Number::from_f64(n.value).unwrap())
      }
    }
    Expr::Lit(Lit::Bool(b)) => Value::Bool(b.value),
    Expr::Object(obj) => object_lit_to_json(obj),
    Expr::Array(arr) => {
      let values: Vec<Value> = arr
        .elems
        .iter()
        .filter_map(|e| e.as_ref().map(|v| expr_to_json(&v.expr)))
        .collect();
      Value::Array(values)
    }
    _ => Value::Null, // 其他情况可以根据需要进行扩展和处理
  }
}

// 将 ObjectLit 转换为 JSON Value
fn object_lit_to_json(obj_lit: &ObjectLit) -> Value {
  let mut map = serde_json::Map::new();

  for prop_or_spread in &obj_lit.props {
    if let PropOrSpread::Prop(prop) = prop_or_spread {
      if let Prop::KeyValue(key_value) = &**prop {
        match &key_value.key {
          PropName::Ident(ident) => {
            let key = ident.sym.to_string();
            let value = expr_to_json(&key_value.value);
            map.insert(key, value);
          }
          PropName::Str(str) => {
            let key = str.clone().value.to_string();
            let value = expr_to_json(&key_value.value);
            map.insert(key, value);
          }
          PropName::Num(num) => {
            let key = num.to_string();
            let value = expr_to_json(&key_value.value);
            map.insert(key, value);
          }
          _ => {}
        }
      }
    }
  }

  Value::Object(map)
}
