use lightningcss::properties::{text, Property};

use crate::{generate_expr_enum, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct TextTransform {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  None,
  Uppercase,
  Lowercase,
  Capitalize,
}

impl ToExpr for TextTransform {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::TextTransform,
      match self.value {
        EnumValue::None => {
          generate_expr_enum!(style_property_enum::ArkUI_TextCase::ARKUI_TEXT_CASE_NORMAL)
        }
        EnumValue::Lowercase => {
          generate_expr_enum!(style_property_enum::ArkUI_TextCase::ARKUI_TEXT_CASE_LOWER)
        }
        EnumValue::Uppercase => {
          generate_expr_enum!(style_property_enum::ArkUI_TextCase::ARKUI_TEXT_CASE_UPPER)
        }
        EnumValue::Capitalize => {
          generate_expr_enum!(style_property_enum::ArkUI_TextCase::ARKUI_TEXT_CASE_NORMAL)
        }
      },
    )
  }
}

impl From<(String, &Property<'_>)> for TextTransform {
  fn from(prop: (String, &Property<'_>)) -> Self {
    TextTransform {
      id: prop.0,
      value: match prop.1 {
        Property::TextTransform(value) => match value.case {
          text::TextTransformCase::None => EnumValue::None,
          text::TextTransformCase::Uppercase => EnumValue::Uppercase,
          text::TextTransformCase::Lowercase => EnumValue::Lowercase,
          text::TextTransformCase::Capitalize => EnumValue::Capitalize,
        },
        _ => EnumValue::None,
      },
    }
  }
}
