
use lightningcss::properties::{Property, border::LineStyle};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, Prop, KeyValueProp, Ident, PropName, ObjectLit, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;


#[derive(Debug, Clone)]
pub enum EBorderStyle {
  Dotted,
  Dashed,
  Solid
}


#[derive(Debug, Clone)]
pub struct BorderStyle {
  pub left: Option<EBorderStyle>,
  pub top: Option<EBorderStyle>,
  pub bottom: Option<EBorderStyle>,
  pub right: Option<EBorderStyle>
}

impl BorderStyle {
  pub fn new() -> Self {
    BorderStyle {
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }

  pub fn is_zero(&self) -> bool {
    self.top.is_none() && self.right.is_none() && self.bottom.is_none() && self.left.is_none()
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = match_str(top);
  }
  pub fn set_right(&mut self, right: &str) {
    self.right = match_str(right);
  }
  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = match_str(bottom);
  }
  pub fn set_left(&mut self, left: &str) {
    self.left = match_str(left);
  }

}

impl ToExpr for BorderStyle {
  fn to_expr(&self) -> Expr {

    let mut arr = vec![];
    
    if let Some(left) = &self.left {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("left".into(), DUMMY_SP)),
        value: get_expr_by_val(left).into(),
      }))))
    }
    if let Some(right) = &self.right {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("right".into(), DUMMY_SP)),
        value: get_expr_by_val(right).into(),
      }))))
    }
    if let Some(bottom) = &self.bottom {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("bottom".into(), DUMMY_SP)),
        value: get_expr_by_val(bottom).into(),
      }))))
    }
    if let Some(top) = &self.top {
      arr.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("top".into(), DUMMY_SP)),
        value: get_expr_by_val(top).into(),
      }))));
    }

    Expr::Object(ObjectLit {
      span: DUMMY_SP,
      props: arr.into(),
    })
  }
}

impl From<&Property<'_>> for BorderStyle {
  fn from(value: &Property<'_>) -> Self {

    let mut border_style = BorderStyle {
      left: None,
      top: None,
      bottom: None,
      right: None
    };

    match value {
      Property::BorderStyle(value) => {
        
        match_key(&value.left).map(|val| {
          border_style.left = Some(val);
        });
        match_key(&value.bottom).map(|val| {
          border_style.bottom = Some(val);
        });
        match_key(&value.right).map(|val| {
          border_style.right = Some(val);
        });
        match_key(&value.top).map(|val| {
          border_style.top = Some(val);
        });
      }
      _ => {}
    };

    border_style
  }
}

fn match_str (str: &str) -> Option<EBorderStyle>{
  let mut res: Option<EBorderStyle> = None;
  match str {
    "dotted" => res = Some(EBorderStyle::Dotted),
    "solid" => res = Some(EBorderStyle::Solid),
    "dashed" => res = Some(EBorderStyle::Dashed),
    _ => {}
  }
  res
}

fn match_key (line_style: &LineStyle) -> Option<EBorderStyle> {
  let mut res = None;
  match line_style {
    LineStyle::Solid => {
      res = Some(EBorderStyle::Solid)
    },
    LineStyle::Dotted => {
      res = Some(EBorderStyle::Dotted)
    },
    LineStyle::Dashed => {
      res = Some(EBorderStyle::Dashed)
    },
    _ => {}
  }
  res
}

fn get_expr_by_val(val: &EBorderStyle) -> Expr {
  Expr::Member(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(Expr::Ident(Ident::new("BorderStyle".into(), DUMMY_SP))),
    prop: MemberProp::Ident(Ident {
      span: DUMMY_SP,
      sym: match &val {
        EBorderStyle::Dashed => "Dashed",
        EBorderStyle::Dotted => "Dotted",
        EBorderStyle::Solid => "Solid",
      }
      .into(),
      optional: false,
    }),
  })
  .into()
}