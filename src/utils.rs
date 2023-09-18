use html5ever::{QualName, ns, LocalName, namespace_url};
use swc_ecma_ast::{JSXObject, JSXMemberExpr};

pub fn recursion_jsx_menber(expr: &JSXMemberExpr) -> String {
  match &expr.obj {
    JSXObject::JSXMemberExpr(expr) => {
      format!("{}.{}", recursion_jsx_menber(expr), expr.prop.sym.to_string())
    },
    JSXObject::Ident(ident) => {
      format!("{}.{}", ident.sym.to_string(), expr.prop.sym.to_string())
    }
  }
}

pub fn create_qualname(str: &str) -> QualName {
  QualName::new(None, ns!(), LocalName::from(str))
}
