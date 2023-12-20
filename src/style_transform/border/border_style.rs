
use lightningcss::properties::{Property, border::LineStyle};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, PropOrSpread, Prop, KeyValueProp, Ident, PropName, ObjectLit, MemberExpr, MemberProp};

use crate::style_transform::traits::ToExpr;

pub fn parse_border_style_item(value: &LineStyle) -> Option<BorderStyle> {
  match &value {
    LineStyle::Dashed => {
      let mut border_style = BorderStyle::new();
      border_style.set_all("Dashed");
      Some(border_style)
    },
    LineStyle::Dotted => {
      let mut border_style = BorderStyle::new();
      border_style.set_all("Dotted");
      Some(border_style)
    },
    LineStyle::Solid => {
      let mut border_style = BorderStyle::new();
      border_style.set_all("Solid");
      Some(border_style)
    },
    _ => None
  }
}

#[derive(Debug, Clone)]
pub struct BorderStyle {
  pub left: Option<String>,
  pub top: Option<String>,
  pub bottom: Option<String>,
  pub right: Option<String>
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

  pub fn set_all(&mut self, style: &str) {
    self.top = Some(style.to_string());
    self.left = Some(style.to_string());
    self.bottom = Some(style.to_string());
    self.right = Some(style.to_string());
  }

  pub fn set_top(&mut self, top: &str) {
    self.top = Some(top.to_string());
  }
  pub fn set_right(&mut self, right: &str) {
    self.right = Some(right.to_string());
  }
  pub fn set_bottom(&mut self, bottom: &str) {
    self.bottom = Some(bottom.to_string());
  }
  pub fn set_left(&mut self, left: &str) {
    self.left = Some(left.to_string());
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
          border_style.left = Some(val.into());
        });
        match_key(&value.bottom).map(|val| {
          border_style.bottom = Some(val.into());
        });
        match_key(&value.right).map(|val| {
          border_style.right = Some(val.into());
        });
        match_key(&value.top).map(|val| {
          border_style.top = Some(val.into());
        });
      }
      _ => {}
    };

    border_style
  }
}


fn match_key (line_style: &LineStyle) -> Option<&str> {
  let mut res = None;
  match line_style {
    LineStyle::Solid => {
      res = Some("Solid")
    },
    LineStyle::Dotted => {
      res = Some("Dotted")
    },
    LineStyle::Dashed => {
      res = Some("Dashed")
    },
    _ => {}
  }
  res
}

fn get_expr_by_val(val: &str) -> Expr {
  let options = ["Solid", "Dotted", "Dashed", "solid", "dotted", "dashed"];
  let sym: &str;
  if is_one_of(&val, &options) {
    sym = val
  } else {
    sym = "Solid"
  }
  Expr::Member(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(Expr::Ident(Ident::new("BorderStyle".into(), DUMMY_SP))),
    prop: MemberProp::Ident(Ident {
      span: DUMMY_SP,
      sym: capitalize_first(sym).into(),
      optional: false,
    }),
  })
  .into()
}

fn capitalize_first(s: &str) -> String {
  if let Some(c) = s.chars().next() {
      let capitalized = c.to_uppercase();
      let rest = &s[c.len_utf8()..];
      format!("{}{}", capitalized, rest)
  } else {
      // 处理空字符串的情况
      String::from(s)
  }
}

fn is_one_of(input: &str, options: &[&str]) -> bool {
  options.iter().any(|&option| option == input)
}