use html5ever::{namespace_url, ns, LocalName, QualName};
// use lightningcss::values::number::CSSNumber;
use swc_ecma_ast::{JSXMemberExpr, JSXObject};

use crate::constants::CONVERT_STYLE_PREFIX;

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
  str.chars().next().unwrap().is_uppercase()
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
  CONVERT_STYLE_PREFIX.to_string() + s
}

// pub fn parse_px_string(input: &str) -> Option<CSSNumber> {
//   // 检查字符串是否以 "px" 结尾
//   if input.ends_with("px") {
//     // 去掉 "px" 单位，然后尝试将其余部分解析为 f32 类型
//     let numeric_part = &input[..input.len() - 2];
//     if let Ok(parsed_value) = numeric_part.parse::<f32>() {
//         return Some(parsed_value);
//     }
//   } else if let Ok(parsed_value) = input.parse::<f32>() {
//       // 如果不以 "px" 结尾但可以解析为 f32 类型，返回纯数字值
//       return Some(parsed_value);
//   }
//   // 如果不以 "px" 结尾或解析失败，返回 None
//   None
// }
