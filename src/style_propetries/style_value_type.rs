use swc_ecma_ast::Expr;

use crate::generate_expr_based_on_platform;

use super::{traits::{ToExpr, ToStyleValue}, flex_align::FlexAlign, item_align::ItemAlign, aspect_ratio::AspactRatio, display::Display, flex_basis::FlexBasis, unit::Platform};


#[derive(Debug, Clone)]
pub enum StyleValueType {
  FlexAlign(FlexAlign),
  AlignItems(ItemAlign),
  FlexBasis(FlexBasis),
  AspectRatio(AspactRatio),
  Display(Display),
}

impl ToStyleValue for StyleValueType {
  fn to_expr(&self, platform: Platform) -> Expr {
    match self {
      StyleValueType::FlexAlign(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::AlignItems(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::FlexBasis(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::AspectRatio(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
      StyleValueType::Display(value) => {
        generate_expr_based_on_platform!(platform, value)
      },
    }
  }
}