

use lightningcss::properties::{display, Property};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct Visibility {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Visible,
  Hidden,
  Invalid
}

impl From<(String, &Property<'_>)> for Visibility {
  fn from(value: (String, &Property<'_>)) -> Self {
    Visibility {
      id: value.0,
      value: {
        if let Property::Visibility(value) = &value.1 {
          match &value {
            display::Visibility::Visible => EnumValue::Visible,
            display::Visibility::Hidden => EnumValue::Hidden,
            _ => EnumValue::Invalid
          }
        } else {
          EnumValue::Invalid
        }
      }
    }
  }
}


impl ToExpr for Visibility {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::Display,
      match &self.value {
        EnumValue::Visible => generate_expr_enum!(style_property_enum::ArkUI_Visibility::ARKUI_VISIBILITY_VISIBLE),
        EnumValue::Hidden => generate_expr_enum!(style_property_enum::ArkUI_Visibility::ARKUI_VISIBILITY_HIDDEN),
        EnumValue::Invalid => generate_invalid_expr!(),
      }
    )
  }

}

