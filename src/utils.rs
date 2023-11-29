use std::collections::HashMap;

use html5ever::{namespace_url, ns, LocalName, QualName};
use regex::Regex;
use swc_common::DUMMY_SP;
// use lightningcss::values::number::CSSNumber;
use swc_ecma_ast::{JSXMemberExpr, JSXObject, Callee, Expr, CallExpr, Ident, Lit, Number, PropOrSpread, Prop, PropName};

use crate::constants::{CONVERT_STYLE_PREFIX, CONVERT_STYLE_PX_FN};

pub fn recursion_jsx_member(expr: &JSXMemberExpr) -> String {
  match &expr.obj {
    JSXObject::JSXMemberExpr(expr) => {
      format!(
        "{}.{}",
        recursion_jsx_member(expr),
        expr.prop.sym.to_string()
      )
    }
    JSXObject::Ident(ident) => {
      format!("{}.{}", ident.sym.to_string(), expr.prop.sym.to_string())
    }
  }
}

pub fn create_qualname(str: &str) -> QualName {
  QualName::new(None, ns!(), LocalName::from(str))
}

pub fn is_starts_with_uppercase(str: &str) -> bool {
  match str.chars().next() {
    Some(c) => c.is_uppercase(),
    None => false,
  }
}

pub fn to_camel_case(s: &str, is_first: bool) -> String {
  let mut result = String::new();
  let mut next_cap = if is_first { true } else { false };
  for c in s.chars() {
    if c == '-' || c == '_' {
      next_cap = true;
    } else if next_cap {
      result.extend(c.to_uppercase());
      next_cap = false;
    } else {
      result.push(c);
    }
  }
  result
}

pub fn to_kebab_case(s: &str) -> String {
  let mut result = String::new();
  for c in s.chars() {
    if c.is_uppercase() {
      result.push('-');
      result.extend(c.to_lowercase());
    } else {
      result.push(c);
    }
  }
  result
}

pub fn prefix_style_key(s: &str) -> String {
  let mut result = String::with_capacity(CONVERT_STYLE_PREFIX.len() + s.len());
  result.push_str(CONVERT_STYLE_PREFIX);
  result.push_str(s);
  result
}

pub fn convert_px_to_units(input: String) -> Expr {
  // 定义匹配 '16px' 的正则表达式
  let re = Regex::new(r"(-?(\d+(\.\d*)?|\.\d+))px").unwrap();
  // 使用正则表达式进行匹配
  if let Some(captures) = re.captures(&input) {
      // 提取匹配到的数字部分
      let input_str = captures.get(1).unwrap().as_str();
      if let Ok(number) = input_str.parse::<f64>() {
        let fun_call_expr = Expr::Call(CallExpr {
          span: DUMMY_SP,
          callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            CONVERT_STYLE_PX_FN.into(),
            DUMMY_SP,
          )))),
          args: vec![
            Expr::Lit(Lit::Num(Number::from(number))).into()
          ],
          type_args: None,
        });

        // 替换原始字符串
        return fun_call_expr;
      } 
  }
  // 如果没有匹配到，则返回原始字符串
  Expr::Lit(Lit::Str(input.into()))
}

pub fn get_callee_attributes (callee: &CallExpr) -> HashMap<String, Box<Expr>> {
  let mut attributes = HashMap::new();

  if let Some(arg) = callee.args.get(1) {
    if let Expr::Object(object) = &*arg.expr {
      for prop in object.props.iter() {
        if let PropOrSpread::Prop(prop) = prop {
          if let Prop::KeyValue(key_value_prop) = &**prop {
            let name = match &key_value_prop.key {
              PropName::Ident(ident) => ident.sym.to_string(),
              PropName::Str(str) => str.value.to_string(),
              _ => "".to_string(),
            };
            
            attributes.insert(name, key_value_prop.value.clone());
          }
        }
      }
    }
  }

  attributes
}
