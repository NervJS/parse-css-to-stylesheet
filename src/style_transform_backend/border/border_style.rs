
use std::borrow::Borrow;

use lightningcss::properties::{Property, border::LineStyle};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, MemberExpr, MemberProp};

use crate::style_transform::{traits::ToExpr, style_value_type::StyleValueType};

#[derive(Debug, Clone)]
pub enum BorderStyleType {
  Solid,
  Dotted,
  Dashed
}

impl ToExpr for BorderStyleType {
  fn to_expr(&self) -> Expr {
    match &self {
      BorderStyleType::Solid => {
        return get_expr_by_val("Solid")
      },
      BorderStyleType::Dotted => {
        return get_expr_by_val("Dotted")
      },
      BorderStyleType::Dashed => {
        return get_expr_by_val("Dashed")
      }
    }
  }
}

impl From<&Property<'_>> for BorderStyleType {
  fn from(value: &Property<'_>) -> Self {
    match &value {
      Property::BorderTopStyle(value) => {
        match_line_style(&value)
      }
      Property::BorderBottomStyle(value) => {
        match_line_style(&value)
      }
      Property::BorderLeftStyle(value) => {
        match_line_style(&value)
      }
      Property::BorderRightStyle(value) => {
        match_line_style(&value)
      },
      _ => BorderStyleType::Solid
    }
  }
}


impl From<&str> for BorderStyleType {
  fn from(value: &str) -> Self {
    match value {
      "Solid" => {
        BorderStyleType::Solid
      },
      "Dotted" => {
        BorderStyleType::Dotted
      },
      "Dashed" => {
        BorderStyleType::Dashed
      },
      _ => BorderStyleType::Solid
    }
  }
}

#[derive(Debug, Clone)]
pub struct BorderStyle {
  pub left: Option<StyleValueType>,
  pub top: Option<StyleValueType>,
  pub bottom: Option<StyleValueType>,
  pub right: Option<StyleValueType>
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

  pub fn set_all(&mut self, style: &StyleValueType) {
    self.top = Some(style.clone());
    self.left = Some(style.clone());
    self.bottom = Some(style.clone());
    self.right = Some(style.clone());
  }

  pub fn set_top(&mut self, top: StyleValueType) {
    self.top = Some(top);
  }
  pub fn set_right(&mut self, right: StyleValueType) {
    self.right = Some(right);
  }
  pub fn set_bottom(&mut self, bottom: StyleValueType) {
    self.bottom = Some(bottom);
  }
  pub fn set_left(&mut self, left: StyleValueType) {
    self.left = Some(left);
  }

}


impl From<&LineStyle> for BorderStyle {
  fn from(value: &LineStyle) -> Self {
    let mut border_style = BorderStyle::new();
    match &value {
      LineStyle::Dashed => {
        border_style.set_all(StyleValueType::BorderStyleType(BorderStyleType::Dashed).borrow());
      },
      LineStyle::Dotted => {
        border_style.set_all(StyleValueType::BorderStyleType(BorderStyleType::Dotted).borrow());
      },
      LineStyle::Solid => {
        border_style.set_all(StyleValueType::BorderStyleType(BorderStyleType::Solid).borrow());
      },
      _ => {}
    }
    border_style
  }
}

impl From<&Property<'_>> for BorderStyle {
  fn from(value: &Property<'_>) -> Self {

    let mut border_style = BorderStyle::new();

    match value {
      Property::BorderStyle(value) => {
        
        match_key(&value.left).map(|val| {
          border_style.set_left(StyleValueType::BorderStyleType(BorderStyleType::from(val)));
        });
        match_key(&value.bottom).map(|val| {
          border_style.set_bottom(StyleValueType::BorderStyleType(BorderStyleType::from(val)));
        });
        match_key(&value.right).map(|val| {
          border_style.set_right(StyleValueType::BorderStyleType(BorderStyleType::from(val)))
        });
        match_key(&value.top).map(|val| {
          border_style.set_top(StyleValueType::BorderStyleType(BorderStyleType::from(val)))
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
  Expr::Member(MemberExpr {
    span: DUMMY_SP,
    obj: Box::new(Expr::Ident(Ident::new("BorderStyle".into(), DUMMY_SP))),
    prop: MemberProp::Ident(Ident {
      span: DUMMY_SP,
      sym: val.into(),
      optional: false,
    }),
  })
  .into()
}


fn match_line_style (value: &LineStyle) -> BorderStyleType {
  match &value {
    LineStyle::Solid => {
      BorderStyleType::Solid
    },
    LineStyle::Dotted => {
      BorderStyleType::Dotted
    },
    LineStyle::Dashed => {
      BorderStyleType::Dashed
    },
    _ => BorderStyleType::Solid
  }
}