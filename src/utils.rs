use html5ever::{namespace_url, ns, LocalName, QualName};
use swc_ecma_ast::{JSXMemberExpr, JSXObject};

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
