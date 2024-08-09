use crate::{
  generate_expr_by_border_side_width, generate_expr_by_length, generate_expr_by_line_style,
  generate_expr_lit_color, generate_invalid_expr,
};
use lightningcss::{
  printer::PrinterOptions,
  properties::{
    border::{self, BorderSideWidth},
    Property,
  },
  traits::ToCss,
  values::{color, length},
};
use swc_core::ecma::ast::*;

use super::{
  border_color::BorderColor, border_style::BorderStyle, border_width::BorderWidth,
  style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple,
};

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
      width: None,
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

        if value.to_css_string(PrinterOptions::default()).unwrap() == "none" {
          let mut style = BorderStyle::new("borderStyle".to_string());
          style.set_all(border::LineStyle::Solid);
          let mut color = BorderColor::new("borderColor".to_string());
          color.set_all(color::CssColor::default());
          let mut width = BorderWidth::new("borderWidth".to_string());
          width.set_all(BorderSideWidth::Length(length::Length::Value(
            length::LengthValue::Px(0.0),
          )));
          border = Border {
            id: prop.0.clone(),
            style: Some(style),
            color: Some(color),
            width: Some(width),
          };
        }
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
    let mut props: Vec<(CSSPropertyType, Expr)> = vec![];
    if self.width.is_none() || self.style.is_none() || self.color.is_none() {
      return PropertyTuple::One(CSSPropertyType::Invalid, generate_invalid_expr!());
    }
    match prop_name.as_str() {
      "border" => {
        vec![
          CSSPropertyType::BorderTopWidth,
          CSSPropertyType::BorderRightWidth,
          CSSPropertyType::BorderBottomWidth,
          CSSPropertyType::BorderLeftWidth,
        ]
        .iter()
        .for_each(|item| {
          props.push((
            *item,
            generate_expr_by_border_side_width!(
              self.width.clone().unwrap().top.unwrap(),
              Platform::Harmony
            ),
          ));
        });
        vec![
          CSSPropertyType::BorderTopStyle,
          CSSPropertyType::BorderRightStyle,
          CSSPropertyType::BorderBottomStyle,
          CSSPropertyType::BorderLeftStyle,
        ]
        .iter()
        .for_each(|item| {
          props.push((
            *item,
            generate_expr_by_line_style!(self.style.clone().unwrap().top.unwrap()),
          ));
        });
        vec![
          CSSPropertyType::BorderTopColor,
          CSSPropertyType::BorderRightColor,
          CSSPropertyType::BorderBottomColor,
          CSSPropertyType::BorderLeftColor,
        ]
        .iter()
        .for_each(|item| {
          props.push((
            *item,
            generate_expr_lit_color!(self.color.clone().unwrap().top.unwrap()),
          ));
        });
        PropertyTuple::Array(props)
      }
      "borderTop" => {
        props.push((
          CSSPropertyType::BorderTopWidth,
          generate_expr_by_border_side_width!(
            self.width.clone().unwrap().top.unwrap(),
            Platform::Harmony
          ),
        ));
        props.push((
          CSSPropertyType::BorderTopStyle,
          generate_expr_by_line_style!(self.style.clone().unwrap().top.unwrap()),
        ));
        props.push((
          CSSPropertyType::BorderTopColor,
          generate_expr_lit_color!(self.color.clone().unwrap().top.unwrap()),
        ));
        PropertyTuple::Array(props)
      }
      "borderRight" => {
        props.push((
          CSSPropertyType::BorderRightWidth,
          generate_expr_by_border_side_width!(
            self.width.clone().unwrap().right.unwrap(),
            Platform::Harmony
          ),
        ));
        props.push((
          CSSPropertyType::BorderRightStyle,
          generate_expr_by_line_style!(self.style.clone().unwrap().right.unwrap()),
        ));
        props.push((
          CSSPropertyType::BorderRightColor,
          generate_expr_lit_color!(self.color.clone().unwrap().right.unwrap()),
        ));
        PropertyTuple::Array(props)
      }
      "borderBottom" => {
        props.push((
          CSSPropertyType::BorderBottomWidth,
          generate_expr_by_border_side_width!(
            self.width.clone().unwrap().bottom.unwrap(),
            Platform::Harmony
          ),
        ));
        props.push((
          CSSPropertyType::BorderBottomStyle,
          generate_expr_by_line_style!(self.style.clone().unwrap().bottom.unwrap()),
        ));
        props.push((
          CSSPropertyType::BorderBottomColor,
          generate_expr_lit_color!(self.color.clone().unwrap().bottom.unwrap()),
        ));
        PropertyTuple::Array(props)
      }
      "borderLeft" => {
        props.push((
          CSSPropertyType::BorderLeftWidth,
          generate_expr_by_border_side_width!(
            self.width.clone().unwrap().left.unwrap(),
            Platform::Harmony
          ),
        ));
        props.push((
          CSSPropertyType::BorderLeftStyle,
          generate_expr_by_line_style!(self.style.clone().unwrap().left.unwrap()),
        ));
        props.push((
          CSSPropertyType::BorderLeftColor,
          generate_expr_lit_color!(self.color.clone().unwrap().left.unwrap()),
        ));
        PropertyTuple::Array(props)
      }
      _ => PropertyTuple::One(CSSPropertyType::Invalid, generate_invalid_expr!()),
    }
  }
}
