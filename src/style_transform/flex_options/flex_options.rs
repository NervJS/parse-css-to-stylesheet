use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, ObjectLit, Prop, PropName, PropOrSpread};

use crate::style_transform::traits::ToExpr;

use super::{
  flex_align::FlexAlign, flex_direction::FlexDirection, flex_wrap::FlexWrap, item_align::ItemAlign,
};

#[derive(Debug, Clone)]
pub struct FlexOptions {
  pub direction: Option<FlexDirection>,
  pub wrap: Option<FlexWrap>,
  pub justify_content: Option<FlexAlign>,
  pub align_items: Option<ItemAlign>,
  pub align_content: Option<FlexAlign>,
}

impl FlexOptions {
  pub fn new() -> Self {
    FlexOptions {
      direction: None,
      wrap: None,
      justify_content: None,
      align_items: None,
      align_content: None,
    }
  }
}

impl ToExpr for FlexOptions {
  fn to_expr(&self) -> Expr {
    let mut props = vec![];
    if let Some(direction) = &self.direction {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("direction".into(), DUMMY_SP)),
        value: direction.to_expr().into(),
      }))));
    }
    if let Some(wrap) = &self.wrap {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("wrap".into(), DUMMY_SP)),
        value: wrap.to_expr().into(),
      }))));
    }
    if let Some(justify_content) = &self.justify_content {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("justifyContent".into(), DUMMY_SP)),
        value: justify_content.to_expr().into(),
      }))));
    }
    if let Some(align_items) = &self.align_items {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("alignItems".into(), DUMMY_SP)),
        value: align_items.to_expr().into(),
      }))));
    }
    if let Some(align_content) = &self.align_content {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("alignContent".into(), DUMMY_SP)),
        value: align_content.to_expr().into(),
      }))));
    }
    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props,
    })
  }
}
