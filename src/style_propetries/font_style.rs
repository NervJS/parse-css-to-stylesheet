use lightningcss::properties::{font, Property};

use crate::{generate_expr_enum, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FontStyle {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Normal,
  Italic,
}

impl ToExpr for FontStyle {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FontStyle,
      match self.value {
        EnumValue::Italic => {
          generate_expr_enum!(style_property_enum::ArkUI_FontStyle::ARKUI_FONT_STYLE_ITALIC)
        }
        EnumValue::Normal => {
          generate_expr_enum!(style_property_enum::ArkUI_FontStyle::ARKUI_FONT_STYLE_NORMAL)
        }
      },
    )
  }
}

impl From<(String, &Property<'_>)> for FontStyle {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontStyle {
      id: prop.0,
      value: match prop.1 {
        Property::FontStyle(value) => match value {
          font::FontStyle::Italic => EnumValue::Italic,
          font::FontStyle::Normal => EnumValue::Normal,
          font::FontStyle::Oblique(_) => EnumValue::Normal,
        },
        _ => EnumValue::Normal,
      },
    }
  }
}
