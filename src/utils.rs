use std::collections::HashMap;

use html5ever::{namespace_url, ns, LocalName, QualName};
use pcre2::bytes::Regex;
// use lightningcss::values::number::CSSNumber;
use swc_core::ecma::{ast::{ArrayLit, CallExpr, Expr, Function, JSXMemberExpr, JSXObject, ObjectLit, Prop, PropName, PropOrSpread}, visit::{Visit, VisitWith}};

use crate::style_propetries::unit::Platform;

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

// pub fn to_kebab_case(s: &str) -> String {
//   let mut result = String::new();
//   for c in s.chars() {
//     if c.is_uppercase() {
//       result.push('-');
//       result.extend(c.to_lowercase());
//     } else {
//       result.push(c);
//     }
//   }
//   result
// }

pub fn prefix_style_key(s: String, platform: Platform) -> String {
  match platform {
    // Platform::Harmony => {
    //   format!("{}{}", CONVERT_STYLE_PREFIX, s)
    // },
    _ => s.to_string()
  }
}

struct ObjectVisitor {
  pub attributes: HashMap<String, Box<Expr>>,
}

impl ObjectVisitor {
  fn new() -> Self {
      ObjectVisitor {
          attributes: HashMap::new(),
      }
  }
}

impl Visit for ObjectVisitor {
  fn visit_object_lit(&mut self, object: &ObjectLit) {
      for prop in &object.props {
          if let PropOrSpread::Prop(prop) = prop {
              if let Prop::KeyValue(kv) = &**prop {
                  if let PropName::Ident(ref ident) = kv.key {
                      let key = ident.sym.to_string();
                      // Clone the value to store in the attributes map
                      let value = kv.value.clone();
                      self.attributes.insert(key, value);
                  }
              }
          }
      }
      // Continue traversing the AST
      object.visit_children_with(self);
  }

  fn visit_call_expr(&mut self,n: &CallExpr) {
      // 可以在这里添加特定的逻辑
      n.args.iter().for_each(|arg| {
        arg.expr.visit_with(self);
      });
  }

  // 确保所有其他类型的节点也被遍历
  fn visit_expr(&mut self, expr: &Expr) {
    // 判断expr的类型
    if let Expr::Object(obj) = &*expr {
      self.visit_object_lit(obj);
    } else {
      expr.visit_children_with(self);
    }

  }
  

}

pub fn get_callee_attributes (callee: &CallExpr) -> HashMap<String, Box<Expr>> {
  let mut attributes = HashMap::new();

  if let Some(arg) = callee.args.get(1) {

    let mut visitor = ObjectVisitor::new();
    (&*arg.expr).visit_children_with(&mut visitor);

 
    return visitor.attributes;
  }

  attributes
}

pub fn fix_rgba(input: String) -> String {
  // 定义匹配 rgba 格式的正则表达式
  let re = Regex::new(r"rgba\((?P<r>\d+), (?P<g>\d+), (?P<b>\d+), (?P<a>\.\d+)\)").unwrap();
  // let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
  let bytes: &[u8] = input.as_bytes();
  for result in re.captures_iter(bytes) {
    if let Ok(caps) = result {
      if let Ok(a) = std::str::from_utf8(&caps["a"]) {
        let r = std::str::from_utf8(&caps["r"]).unwrap();
        let g = std::str::from_utf8(&caps["g"]).unwrap();
        let b = std::str::from_utf8(&caps["b"]).unwrap();
        let corrected_alpha = format!("0{:.6}", a);
        return format!("rgba({}, {}, {}, {})", r, g, b, corrected_alpha)
      }
    }
  }
  input
}

#[derive(Debug, Clone)]
pub enum TSelector {
  String(String),
  Array(Vec<String>),
}

// 分割选择器
pub fn split_selector(selector: &str) -> Vec<TSelector> {
  let mut result = Vec::new();
    let mut current_word = String::new();
    let mut buffer = String::new();

    for c in selector.chars() {
        if c == ' ' || c == '>' || c == '+' || c == '~' {
            if !current_word.is_empty() {
                result.push(split_classes(current_word.as_str()));
                current_word.clear();
            }
            buffer.push(c);
            if buffer == " > " || buffer == " + " || buffer == " ~ " {
                result.push(TSelector::String(buffer.clone()));
                buffer.clear();
            }
        } else {
            current_word.push(c);
            if buffer == ' '.to_string() {
              result.push(TSelector::String(buffer.clone()));
              buffer.clear();
            }
        }
    }

    if !current_word.is_empty() {
        result.push(split_classes(current_word.as_str()));
    }

    if !buffer.is_empty() {
        result.push(TSelector::String(buffer.clone()));
    }

    result
}

// 分割类名 .a.b.c => ["a", "b", "c"]
fn split_classes(input: &str) -> TSelector {
  let mut matches = Vec::new();
  let mut current_class = String::new();
  for char in input.chars() {
      if char == '.' {
        if !current_class.is_empty() {
            matches.push(current_class.clone());
            current_class.clear();
        }
      } else {
        current_class.push(char);
      }
  }
  if !current_class.is_empty() {
      matches.push(current_class);
  }
  if matches.len() > 1 {
      TSelector::Array(matches)
  } else {
      TSelector::String(input.replace(".", ""))
  }
}
