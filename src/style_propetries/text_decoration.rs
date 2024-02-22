use lightningcss::{properties::{Property, text}, traits::ToCss, stylesheet::PrinterOptions, targets::{Targets, Features}};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Ident, KeyValueProp, MemberExpr, MemberProp, ObjectLit, Prop, PropName, PropOrSpread};

use crate::{style_propetries::traits::ToExpr, generate_invalid_expr, generate_expr_lit_str, generate_prop_name};

use super::unit::PropertyTuple;


#[derive(Debug, Clone)]
pub struct TextDecoration {
  pub id: String,
  pub line: Option<TextDecorationLine>,
  pub style: Option<TextDecorationStyle>,
  pub color: Option<TextDecorationColor>
}

#[derive(Debug, Clone)]
pub enum TextDecorationLine {
  Underline,
  LineThrough,
  Overline,
  None
}
#[derive(Debug, Clone)]
pub enum TextDecorationStyle {
  Solid,
  Double,
  Dotted,
  Dashed,
  Wavy
}
#[derive(Debug, Clone)]
pub struct TextDecorationColor(String);

impl ToExpr for TextDecoration {
  fn to_expr(&self) -> PropertyTuple {
    let mut props = vec![];

    if let Some(line) = &self.line {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: generate_prop_name!("type"),
        value: Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(Expr::Ident(Ident::new("TextDecorationType".into(), DUMMY_SP))),
          prop: MemberProp::Ident(Ident {
            span: DUMMY_SP,
            sym: match line {
              TextDecorationLine::Underline => "Underline",
              TextDecorationLine::LineThrough => "LineThrough",
              TextDecorationLine::Overline => "Overline",
              _ => "None",
            }
            .into(),
            optional: false,
          }),
        })
        .into(),
      }))));
    }

    if let Some(color) = &self.color {
      props.push( PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: generate_prop_name!("color"),
        value: generate_expr_lit_str!(color.0.clone()).into()
      }))));
    }

    PropertyTuple::One(
      generate_prop_name!(*self.id),
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: props
      })
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    let mut props: Vec<(PropName, Expr)> = vec![];

    if let Some(line) = &self.line {
      props.push(
        (generate_prop_name!("textDecorationLine"),
        match line {
          TextDecorationLine::Underline => generate_expr_lit_str!("underline"),
          TextDecorationLine::LineThrough => generate_expr_lit_str!("line-through"),
          TextDecorationLine::Overline => generate_invalid_expr!(),
          TextDecorationLine::None => generate_expr_lit_str!("none")
        })
      )
    };
    if let Some(style) = &self.style {
      props.push(
        (generate_prop_name!("textDecorationStyle"),
        match &style {
          TextDecorationStyle::Solid => generate_expr_lit_str!("solid"),
          TextDecorationStyle::Double => generate_expr_lit_str!("double"),
          TextDecorationStyle::Dotted => generate_expr_lit_str!("dotted"),
          TextDecorationStyle::Dashed => generate_expr_lit_str!("dashed"),
          TextDecorationStyle::Wavy => generate_expr_lit_str!("wavy"),
        })
      )
    };
    if let Some(color) = &self.color {
      props.push(
        (generate_prop_name!("textDecorationColor"),
        generate_expr_lit_str!(color.0.clone()))
      )
    }
    PropertyTuple::Array(props)
  }
}

impl From<(String, &Property<'_>)> for TextDecoration {
  fn from(prop: (String, &Property<'_>)) -> Self {
    match prop.1 {
      Property::TextDecoration(value, _) => {
        let line = match value.line {
          text::TextDecorationLine::LineThrough => TextDecorationLine::LineThrough,
          text::TextDecorationLine::Overline => TextDecorationLine::Overline,
          text::TextDecorationLine::Underline => TextDecorationLine::Underline,
          _ => TextDecorationLine::None,
        };
        let style = match value.style {
          text::TextDecorationStyle::Solid => TextDecorationStyle::Solid,
          text::TextDecorationStyle::Double => TextDecorationStyle::Double,
          text::TextDecorationStyle::Dotted => TextDecorationStyle::Dotted,
          text::TextDecorationStyle::Dashed => TextDecorationStyle::Dashed,
          text::TextDecorationStyle::Wavy => TextDecorationStyle::Wavy,
        };
        let color_string = value.color.to_css_string(PrinterOptions {
          minify: false,
          targets: Targets {
            include: Features::HexAlphaColors,
            ..Targets::default()
          },
          ..PrinterOptions::default()
        });
        let color: Option<TextDecorationColor>;
        if let Ok(c) = color_string {
          // 如果c为"currentColor"，则不设置color
          if c == "currentColor" {
            color = None
          } else {
            color = Some(TextDecorationColor(c));
          }
        } else {
          color = None
        }
        TextDecoration {
          id: prop.0,
          line: Some(line),
          style: Some(style),
          color: color
        }
      },
      Property::TextDecorationLine(value, _) => {
        let line = match *value {
          text::TextDecorationLine::LineThrough => TextDecorationLine::LineThrough,
          text::TextDecorationLine::Overline => TextDecorationLine::Overline,
          text::TextDecorationLine::Underline => TextDecorationLine::Underline,
          _ => TextDecorationLine::None,
        };
        TextDecoration {
          id: prop.0,
          line: Some(line),
          style: None,
          color: None
        }
      }
      Property::TextDecorationStyle(value, _) => {
        let style = match *value {
          text::TextDecorationStyle::Solid => TextDecorationStyle::Solid,
          text::TextDecorationStyle::Double => TextDecorationStyle::Double,
          text::TextDecorationStyle::Dotted => TextDecorationStyle::Dotted,
          text::TextDecorationStyle::Dashed => TextDecorationStyle::Dashed,
          text::TextDecorationStyle::Wavy => TextDecorationStyle::Wavy,
        };
        TextDecoration {
          id: prop.0,
          line: None,
          style: Some(style),
          color: None
        }
      }
      Property::TextDecorationColor(value, _) => {
        let color_string = value.to_css_string(PrinterOptions {
          minify: false,
          targets: Targets {
            include: Features::HexAlphaColors,
            ..Targets::default()
          },
          ..PrinterOptions::default()
        });
        let color: Option<TextDecorationColor>;
        if let Ok(c) = color_string {
          // 如果c为"currentColor"，则不设置color
          if c == "currentColor" {
            color = None
          } else {
            color = Some(TextDecorationColor(c));
          }
        } else {
          color = None
        }
        TextDecoration {
          id: prop.0,
          line: None,
          style: None,
          color: color
        }
      }
      _ => {
        TextDecoration {
          id: prop.0,
          line: None,
          style: None,
          color: None
        }
      }
    }
  }
}
