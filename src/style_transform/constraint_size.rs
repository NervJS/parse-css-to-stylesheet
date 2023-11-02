use lightningcss::properties::Property;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, ObjectLit, KeyValueProp, PropOrSpread, PropName, Prop, Ident};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub struct ConstraintSize {
  pub min_width: Option<String>,
  pub max_width: Option<String>,
  pub min_height: Option<String>,
  pub max_height: Option<String>,
}

impl ConstraintSize {
  pub fn new() -> Self {
    ConstraintSize {
      min_height: None,
      max_height: None,
      min_width: None,
      max_width: None,
    }
  }
}

impl ToExpr for ConstraintSize {
  fn to_expr(&self) -> Expr {
    let mut arr = vec![];
    
    if let Some(min_height) = &self.min_height {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("minHeight".into(), DUMMY_SP)),
        value: min_height.to_string().into(),
      }))))
    }
    if let Some(max_height) = &self.max_height {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("maxHeight".into(), DUMMY_SP)),
        value: max_height.to_string().into(),
      }))))
    }
    if let Some(min_width) = &self.min_width {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("minWidth".into(), DUMMY_SP)),
        value: min_width.to_string().into(),
      }))))
    }
    if let Some(max_width) = &self.max_width {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("maxWidth".into(), DUMMY_SP)),
        value: max_width.to_string().into(),
      }))))
    }
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })

   
  }
}


impl From<&Property<'_>> for ConstraintSize {
  fn from(_: &Property<'_>) -> Self {
    ConstraintSize::new()
  }
}

