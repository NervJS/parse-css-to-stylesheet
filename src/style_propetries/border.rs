use lightningcss::{
  properties::Property,
  traits::ToCss
};
use swc_atoms::Atom;
use swc_ecma_ast::{PropName, Expr, Tpl};
use crate::{generate_prop_name, generate_expr_lit_str, generate_expr_by_length, generate_invalid_expr, generate_expr_by_border_side_width, generate_expr_by_line_style, style_propetries::unit::Platform, generate_string_by_css_color };

use super::{traits::ToExpr, unit::PropertyTuple, border_color::BorderColor, border_style::{BorderStyle, get_expr_by_val}, border_width::BorderWidth};


macro_rules! generate_tpl_expr {
    ($props: expr) => {
      Expr::Tpl(Tpl {
        span: swc_common::DUMMY_SP,
        exprs: $props,
        quasis: vec![
          swc_ecma_ast::TplElement {
            span: swc_common::DUMMY_SP,
            tail: false,
            cooked: None,
            raw: Atom::from("").into(),
          },
          swc_ecma_ast::TplElement {
            span: swc_common::DUMMY_SP,
            tail: false,
            cooked: Some(" ".into()),
            raw: Atom::from(" ").into(),
          },
          swc_ecma_ast::TplElement {
            span: swc_common::DUMMY_SP,
            tail: false,
            cooked: Some(" ".into()),
            raw: Atom::from(" ").into(),
          },
          swc_ecma_ast::TplElement {
            span: swc_common::DUMMY_SP,
            tail: true,
            cooked: None,
            raw: Atom::from("").into(),
          }
        ]
      })
    };
}

#[derive(Debug, Clone)]
pub struct Border {
  pub id: String,
  pub style: Option<BorderStyle>,
  pub color: Option<BorderColor>,
  pub width: Option<BorderWidth>,
}



impl From<(String, &Property<'_>)> for Border {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut border = Border {
      id: prop.0.clone(),
      style: None,
      color: None,
      width: None
    };
    match prop.1 {
      Property::Border(value) => {
        let mut style = BorderStyle::new("borderStyle".to_string());
        style.set_all(value.style);
        let mut color = BorderColor::new("borderColor".to_string());
        color.set_all(value.color.to_owned());
        let mut width = BorderWidth::new("borderWidth".to_string());
        width.set_all(value.width.to_owned());
        border = Border {
          id: prop.0.clone(),
          style: Some(style),
          color: Some(color),
          width: Some(width),
        };
      }
      Property::BorderTop(value) => {
        let mut style = BorderStyle::new("borderTopStyle".to_string());
        style.set_all(value.style);
        let mut color = BorderColor::new("borderTopColor".to_string());
        color.set_all(value.color.to_owned());
        let mut width = BorderWidth::new("borderTopWidth".to_string());
        width.set_all(value.width.to_owned());
        border = Border {
          id: prop.0.clone(),
          style: Some(style),
          color: Some(color),
          width: Some(width),
        };
      }
      Property::BorderRight(value) => {
        let mut style = BorderStyle::new("borderRightStyle".to_string());
        style.set_all(value.style);
        let mut color = BorderColor::new("borderRightColor".to_string());
        color.set_all(value.color.to_owned());
        let mut width = BorderWidth::new("borderRightWidth".to_string());
        width.set_all(value.width.to_owned());
        border = Border {
          id: prop.0.clone(),
          style: Some(style),
          color: Some(color),
          width: Some(width),
        };
      }
      Property::BorderBottom(value) => {
        let mut style = BorderStyle::new("borderBottomStyle".to_string());
        style.set_all(value.style);
        let mut color = BorderColor::new("borderBottomColor".to_string());
        color.set_all(value.color.to_owned());
        let mut width = BorderWidth::new("borderBottomWidth".to_string());
        width.set_all(value.width.to_owned());
        border = Border {
          id: prop.0.clone(),
          style: Some(style),
          color: Some(color),
          width: Some(width),
        };
      }
      Property::BorderLeft(value) => {
        let mut style = BorderStyle::new("borderLeftStyle".to_string());
        style.set_all(value.style);
        let mut color = BorderColor::new("borderLeftColor".to_string());
        color.set_all(value.color.to_owned());
        let mut width = BorderWidth::new("borderLeftWidth".to_string());
        width.set_all(value.width.to_owned());
        border = Border {
          id: prop.0.clone(),
          style: Some(style),
          color: Some(color),
          width: Some(width),
        };
      }
      _ => {}
    }
    border
  }
}

impl ToExpr for Border {
    fn to_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;
      let mut props: Vec<(PropName, Expr)> = vec![];
      match prop_name.as_str() {
        "border" => {
          vec!["borderTopWidth", "borderRightWidth", "borderBottomWidth", "borderLeftWidth"].iter().for_each(|item| {
            props.push((generate_prop_name!(item.to_owned()), generate_expr_by_border_side_width!(self.width.clone().unwrap().top.unwrap())));
          });
          vec!["borderTopStyle", "borderRightStyle", "borderBottomStyle", "borderLeftStyle"].iter().for_each(|item| {
            props.push((generate_prop_name!(item.to_owned()), generate_expr_by_line_style!(self.style.clone().unwrap().top.unwrap(), Platform::Harmony)));
          });
          vec!["borderTopColor", "borderRightColor", "borderBottomColor", "borderLeftColor"].iter().for_each(|item| {
            props.push((generate_prop_name!(item.to_owned()), generate_string_by_css_color!(self.color.clone().unwrap().top.unwrap())));
          });
          PropertyTuple::Array(props)
        },
        "borderTop"  => {
          props.push((generate_prop_name!("borderTopWidth"), generate_expr_by_border_side_width!(self.width.clone().unwrap().top.unwrap())));
          props.push((generate_prop_name!("borderTopStyle"), generate_expr_by_line_style!(self.style.clone().unwrap().top.unwrap(), Platform::Harmony)));
          props.push((generate_prop_name!("borderTopColor"), generate_string_by_css_color!(self.color.clone().unwrap().top.unwrap())));
          PropertyTuple::Array(props)
        },
        "borderRight" => {
          props.push((generate_prop_name!("borderRightWidth"), generate_expr_by_border_side_width!(self.width.clone().unwrap().right.unwrap())));
          props.push((generate_prop_name!("borderRightStyle"), generate_expr_by_line_style!(self.style.clone().unwrap().right.unwrap(), Platform::Harmony)));
          props.push((generate_prop_name!("borderRightColor"), generate_string_by_css_color!(self.color.clone().unwrap().right.unwrap())));
          PropertyTuple::Array(props)
        },
        "borderBottom" => {
          props.push((generate_prop_name!("borderBottomWidth"), generate_expr_by_border_side_width!(self.width.clone().unwrap().bottom.unwrap())));
          props.push((generate_prop_name!("borderBottomStyle"), generate_expr_by_line_style!(self.style.clone().unwrap().bottom.unwrap(), Platform::Harmony)));
          props.push((generate_prop_name!("borderBottomColor"), generate_string_by_css_color!(self.color.clone().unwrap().bottom.unwrap())));
          PropertyTuple::Array(props)
        },
        "borderLeft" => {
          props.push((generate_prop_name!("borderLeftWidth"), generate_expr_by_border_side_width!(self.width.clone().unwrap().left.unwrap())));
          props.push((generate_prop_name!("borderLeftStyle"), generate_expr_by_line_style!(self.style.clone().unwrap().left.unwrap(), Platform::Harmony)));
          props.push((generate_prop_name!("borderLeftColor"), generate_string_by_css_color!(self.color.clone().unwrap().left.unwrap())));
          PropertyTuple::Array(props)
        },
        _ => PropertyTuple::One(generate_prop_name!(prop_name.to_owned()), generate_invalid_expr!())
      }
    }

    fn to_rn_expr(&self) -> PropertyTuple {
      let prop_name = &self.id;
      let mut props: Vec<Box<Expr>> = vec![];
      match prop_name.as_str() {
        "border" | "borderTop"  => {
          props.push(Box::new(generate_expr_by_border_side_width!(self.width.clone().unwrap().top.unwrap())));
          props.push(Box::new(generate_expr_by_line_style!(self.style.clone().unwrap().top.unwrap(), Platform::ReactNative)));
          props.push(Box::new(generate_string_by_css_color!(self.color.clone().unwrap().top.unwrap())));
          let tpl = generate_tpl_expr!(props);
          PropertyTuple::One(
            generate_prop_name!(self.id.clone()),
            tpl
          )
        },
        "borderRight" => {
          props.push(Box::new(generate_expr_by_border_side_width!(self.width.clone().unwrap().right.unwrap())));
          props.push(Box::new(generate_expr_by_line_style!(self.style.clone().unwrap().right.unwrap(), Platform::ReactNative)));
          props.push(Box::new(generate_string_by_css_color!(self.color.clone().unwrap().right.unwrap())));
          let tpl = generate_tpl_expr!(props);
          PropertyTuple::One(
            generate_prop_name!(self.id.clone()),
            tpl
          )
        },
        "borderBottom" => {
          props.push(Box::new(generate_expr_by_border_side_width!(self.width.clone().unwrap().bottom.unwrap())));
          props.push(Box::new(generate_expr_by_line_style!(self.style.clone().unwrap().bottom.unwrap(), Platform::ReactNative)));
          props.push(Box::new(generate_string_by_css_color!(self.color.clone().unwrap().bottom.unwrap())));
          let tpl = generate_tpl_expr!(props);
          PropertyTuple::One(
            generate_prop_name!(self.id.clone()),
            tpl
          )
        },
        "borderLeft" => {
          props.push(Box::new(generate_expr_by_border_side_width!(self.width.clone().unwrap().left.unwrap())));
          props.push(Box::new(generate_expr_by_line_style!(self.style.clone().unwrap().left.unwrap(), Platform::ReactNative)));
          props.push(Box::new(generate_string_by_css_color!(self.color.clone().unwrap().left.unwrap())));
          let tpl = generate_tpl_expr!(props);
          PropertyTuple::One(
            generate_prop_name!(self.id.clone()),
            tpl
          )
        },
        _ => PropertyTuple::One(
          generate_prop_name!(self.id.clone()),
          generate_invalid_expr!()
        )
      }
    }
}