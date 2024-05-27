
use std::collections::HashMap;
use serde_json::Value;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use crate::style_propetries::style_value_type::StyleValueType;

use crate::style_propetries::unit::Platform;
use crate::visitor::parse_style_values;

pub struct JsonWriter {
    styles: HashMap<String, Vec<StyleValueType>>,
}

impl JsonWriter {

    pub fn new(styles: HashMap<String, Vec<StyleValueType>>) -> Self {
        Self { styles }
    }

    pub fn to_json(&self) -> String {
        let elems: Vec<Expr> = self.styles.iter().filter_map(|(selector, prop_or_spreads)| Some({
            Expr::Object(ObjectLit {
                span: DUMMY_SP,
                props: vec![
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new("selector".into(), DUMMY_SP)),
                        value: Box::new(Expr::Lit(Lit::Str(selector.clone().into()))),
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
                        value: Box::new(Expr::Array(ArrayLit {
                            span: DUMMY_SP,
                            elems: parse_style_values(prop_or_spreads.clone(), Platform::Harmony)
                        })),
                    }))),
                ]
            })
        })).collect();

        let json_value = expr_to_json(&Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::Ident(Ident::new("styles".into(), DUMMY_SP)),
                    value: Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: elems.into_iter().map(|expr| Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(expr),
                        })).collect()
                    })),
                })))
            ],
        }));

        // 打印 JSON 值

        let json_string = serde_json::to_string_pretty(&json_value).unwrap();
      
        json_string
    }
}


// 将 Expr 转换为 JSON Value
fn expr_to_json(value: &Expr) -> Value {
    match value {
        Expr::Lit(Lit::Str(s)) => {
            Value::String(s.value.to_string())
        },
        Expr::Lit(Lit::Num(n)) => {
            if n.value.fract() == 0.0 {
                // 是整数
                Value::Number(serde_json::Number::from(n.value as i64))
            } else {
                // 是浮点数
                Value::Number(serde_json::Number::from_f64(n.value).unwrap())
            }
        },
        Expr::Lit(Lit::Bool(b)) => Value::Bool(b.value),
        Expr::Object(obj) => object_lit_to_json(obj),
        Expr::Array(arr) => {
            let values: Vec<Value> = arr.elems.iter().filter_map(|e| e.as_ref().map(|v| expr_to_json(&v.expr))).collect();
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
                    },
                    PropName::Str(str) => {
                        let key = str.clone().value.to_string();
                        let value = expr_to_json(&key_value.value);
                        map.insert(key, value);
                    },
                    PropName::Num(num) => {
                        let key = num.to_string();
                        let value = expr_to_json(&key_value.value);
                        map.insert(key, value);
                    },
                    _ => {}
                }
            }
        }
    }

    Value::Object(map)
}

