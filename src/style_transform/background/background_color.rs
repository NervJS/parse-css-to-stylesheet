use std::fmt::{Display, Formatter};

use lightningcss::{
  properties::Property,
  stylesheet::PrinterOptions,
  targets::{Features, Targets},
  traits::ToCss,
  values::color::CssColor,
};
use swc_ecma_ast::{Expr, Lit, Str};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub struct BackgroundColor(pub String);

impl Display for BackgroundColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl ToExpr for BackgroundColor {
  fn to_expr(&self) -> Expr {
    Expr::Lit(Lit::Str(Str::from(self.0.to_string()))).into()
  }
}

impl From<&Property<'_>> for BackgroundColor {
  fn from(value: &Property<'_>) -> Self {
    let mut background_color = BackgroundColor("".to_string());
    match value {
      Property::BackgroundColor(value) => {
        if *value != CssColor::default() {
          background_color = BackgroundColor(
            value
              .to_css_string(PrinterOptions {
                minify: false,
                targets: Targets {
                  include: Features::HexAlphaColors,
                  ..Targets::default()
                },
                ..PrinterOptions::default()
              })
              .unwrap(),
          );
        }
      }
      _ => {}
    }
    background_color
  }
}
