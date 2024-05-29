use lightningcss::{properties::{text, Property}, stylesheet::PrinterOptions, targets::{Features, Targets}, traits::ToCss, values::color::CssColor};

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use crate::{generate_expr_enum, generate_expr_lit_color, generate_prop_name, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};


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
pub struct TextDecorationColor(CssColor);

impl ToExpr for TextDecoration {
  fn to_expr(&self) -> PropertyTuple {
    let mut props = vec![];

    if let Some(line) = &self.line {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: generate_prop_name!("type"),
        value: match line {
          TextDecorationLine::Underline => generate_expr_enum!(style_property_enum::ArkUI_TextDecorationType::ARKUI_TEXT_DECORATION_TYPE_UNDERLINE),
          TextDecorationLine::LineThrough => generate_expr_enum!(style_property_enum::ArkUI_TextDecorationType::ARKUI_TEXT_DECORATION_TYPE_LINE_THROUGH),
          TextDecorationLine::Overline => generate_expr_enum!(style_property_enum::ArkUI_TextDecorationType::ARKUI_TEXT_DECORATION_TYPE_OVERLINE),
          _ => generate_expr_enum!(style_property_enum::ArkUI_TextDecorationType::ARKUI_TEXT_DECORATION_TYPE_NONE),
        }.into()
      }))));
    }

    if let Some(color) = &self.color {
      props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: generate_prop_name!("color"),
        value: generate_expr_lit_color!(color.0.clone()).into()
      }))));
    }

    PropertyTuple::One(
      CSSPropertyType::TextDecoration,
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: props
      })
    )
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
            color = Some(TextDecorationColor(value.color.clone()));
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
            color = Some(TextDecorationColor(value.clone()));
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
