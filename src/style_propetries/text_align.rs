use lightningcss::properties::{text, Property};

use crate::{
  generate_expr_enum,
  style_propetries::{style_property_enum, traits::ToExpr},
};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct TextAlign {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Start,
  Center,
  End,
  Justify,
}

impl ToExpr for TextAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::TextAlign,
      match self.value {
        EnumValue::Start => {
          generate_expr_enum!(style_property_enum::ArkUI_TextAlignment::ARKUI_TEXT_ALIGNMENT_START)
        }
        EnumValue::Center => {
          generate_expr_enum!(style_property_enum::ArkUI_TextAlignment::ARKUI_TEXT_ALIGNMENT_CENTER)
        }
        EnumValue::End => {
          generate_expr_enum!(style_property_enum::ArkUI_TextAlignment::ARKUI_TEXT_ALIGNMENT_END)
        }
        EnumValue::Justify => generate_expr_enum!(
          style_property_enum::ArkUI_TextAlignment::ARKUI_TEXT_ALIGNMENT_JUSTIFY
        ),
      },
    )
  }
}

impl From<(String, &Property<'_>)> for TextAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    TextAlign {
      id: prop.0,
      value: match prop.1 {
        Property::TextAlign(value) => match value {
          text::TextAlign::Left | text::TextAlign::Start => EnumValue::Start,
          text::TextAlign::Right | text::TextAlign::End => EnumValue::End,
          text::TextAlign::Center => EnumValue::Center,
          text::TextAlign::Justify => EnumValue::Justify,
          _ => EnumValue::Start,
        },
        _ => EnumValue::Start,
      },
    }
  }
}
