use std::collections::HashSet;

use html5ever::{namespace_url, ns, LocalName, QualName};
use lightningcss::properties::PropertyId;
use once_cell::sync::Lazy;
use swc_ecma_ast::{JSXMemberExpr, JSXObject};

static INHERITABLE_STYLES: Lazy<HashSet<PropertyId<'static>>> = Lazy::new(|| {
  let mut styles = HashSet::new();
  styles.insert(PropertyId::from("color"));
  styles.insert(PropertyId::from("font-size"));
  styles.insert(PropertyId::from("font-family"));
  styles.insert(PropertyId::from("font-weight"));

  styles.insert(PropertyId::from("font-style"));
  styles.insert(PropertyId::from("font-variant"));
  styles.insert(PropertyId::from("font"));
  styles.insert(PropertyId::from("font-size-adjust"));
  styles.insert(PropertyId::from("font-stretch"));
  styles.insert(PropertyId::from("font-smoothing"));
  styles.insert(PropertyId::from("font-synthesis"));
  styles.insert(PropertyId::from("font-feature-settings"));
  styles.insert(PropertyId::from("font-kerning"));
  styles.insert(PropertyId::from("font-variant-caps"));
  styles.insert(PropertyId::from("font-variant-numeric"));
  styles.insert(PropertyId::from("font-variant-east-asian"));
  styles.insert(PropertyId::from("font-variant-ligatures"));
  styles.insert(PropertyId::from("font-variant-position"));
  styles.insert(PropertyId::from("line-height"));
  styles.insert(PropertyId::from("visibility"));
  styles.insert(PropertyId::from("white-space"));
  styles.insert(PropertyId::from("word-spacing"));
  styles.insert(PropertyId::from("letter-spacing"));
  styles.insert(PropertyId::from("text-align"));
  styles.insert(PropertyId::from("text-emphasize"));
  styles.insert(PropertyId::from("text-rendering"));
  styles.insert(PropertyId::from("text-indent"));
  styles.insert(PropertyId::from("text-transform"));
  styles.insert(PropertyId::from("text-decoration-thickness"));
  styles.insert(PropertyId::from("text-decoration-offset"));
  styles.insert(PropertyId::from("cursor"));
  styles.insert(PropertyId::from("direction"));
  styles.insert(PropertyId::from("quotes"));
  styles.insert(PropertyId::from("caption-side"));
  styles.insert(PropertyId::from("border-collapse"));
  styles.insert(PropertyId::from("border-spacing"));
  styles.insert(PropertyId::from("empty-cells"));
  styles.insert(PropertyId::from("table-layout"));
  styles.insert(PropertyId::from("list-style-type"));
  styles.insert(PropertyId::from("list-style-image"));
  styles.insert(PropertyId::from("list-style-position"));
  styles.insert(PropertyId::from("list-style"));
  styles.insert(PropertyId::from("page-break-inside"));
  styles.insert(PropertyId::from("page"));
  styles.insert(PropertyId::from("orphans"));
  styles.insert(PropertyId::from("windows"));
  styles.insert(PropertyId::from("speak"));
  styles.insert(PropertyId::from("speak-punctuation"));
  styles.insert(PropertyId::from("speak-numeral"));
  styles.insert(PropertyId::from("speak-header"));
  styles.insert(PropertyId::from("speech-rate"));
  styles.insert(PropertyId::from("volume"));
  styles.insert(PropertyId::from("voice-family"));
  styles.insert(PropertyId::from("pitch"));
  styles.insert(PropertyId::from("pitch-range"));
  styles.insert(PropertyId::from("stress"));
  styles.insert(PropertyId::from("richness"));
  styles.insert(PropertyId::from("azimuth"));
  styles.insert(PropertyId::from("elevation"));

  styles
});

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

pub fn is_style_inheritable(style: PropertyId<'_>) -> bool {
  INHERITABLE_STYLES.contains(&style)
}

pub fn is_starts_with_uppercase(str: &str) -> bool {
  str.chars().next().unwrap().is_uppercase()
}
