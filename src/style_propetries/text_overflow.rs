use lightningcss::properties::{overflow, Property};

use crate::{generate_expr_enum, style_propetries::traits::ToExpr};
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;

use super::{style_property_enum, style_property_type::CSSPropertyType, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub enum TextOverflow {
  Clip,
  Ellipsis,
  None,
}

impl ToExpr for TextOverflow {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::TextOverflow,
      match self {
        TextOverflow::Clip => {
          generate_expr_enum!(style_property_enum::ArkUI_TextOverflow::ARKUI_TEXT_OVERFLOW_CLIP)
        }
        TextOverflow::Ellipsis => {
          generate_expr_enum!(style_property_enum::ArkUI_TextOverflow::ARKUI_TEXT_OVERFLOW_ELLIPSIS)
        }
        TextOverflow::None => {
          generate_expr_enum!(style_property_enum::ArkUI_TextOverflow::ARKUI_TEXT_OVERFLOW_NONE)
        }
      }
      .into(),
    )
  }
}

impl From<(String, &Property<'_>)> for TextOverflow {
  fn from(value: (String, &Property<'_>)) -> Self {
    let mut text_overflows = TextOverflow::None;
    match value.1 {
      Property::TextOverflow(value, _) => match value {
        overflow::TextOverflow::Clip => text_overflows = TextOverflow::Clip,
        overflow::TextOverflow::Ellipsis => text_overflows = TextOverflow::Ellipsis,
      },
      _ => {}
    }
    text_overflows
  }
}
