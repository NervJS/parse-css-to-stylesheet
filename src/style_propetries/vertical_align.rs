use lightningcss::properties::{Property, font};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct VerticalAlign {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Baseline,
  Sub,
  Super,
  TextTop,
  TextBottom,
  Middle,
  Top,
  Bottom,
  Invalid
}

impl ToExpr for VerticalAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::VerticalAlign,
      {
        match self.value {
          EnumValue::Baseline | EnumValue::Sub | EnumValue::Super | EnumValue::TextTop | EnumValue::TextBottom | EnumValue::Invalid => generate_invalid_expr!(),
          EnumValue::Middle => generate_expr_enum!(style_property_enum::ArkUI_Alignment::ARKUI_ALIGNMENT_CENTER),
          EnumValue::Top => generate_expr_enum!(style_property_enum::ArkUI_Alignment::ARKUI_ALIGNMENT_TOP),
          EnumValue::Bottom => generate_expr_enum!(style_property_enum::ArkUI_Alignment::ARKUI_ALIGNMENT_BOTTOM),
        }
      }
    )
  }
}

impl From<(String, &Property<'_>)> for VerticalAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    VerticalAlign {
      id: prop.0,
      value: match prop.1 {
        Property::VerticalAlign(value) => {
          match value {
            font::VerticalAlign::Keyword(keyword) => {
              match keyword {
                font::VerticalAlignKeyword::Baseline => EnumValue::Baseline,
                font::VerticalAlignKeyword::Sub => EnumValue::Sub,
                font::VerticalAlignKeyword::Super => EnumValue::Super,
                font::VerticalAlignKeyword::TextTop => EnumValue::TextTop,
                font::VerticalAlignKeyword::TextBottom => EnumValue::TextBottom,
                font::VerticalAlignKeyword::Middle => EnumValue::Middle,
                font::VerticalAlignKeyword::Top => EnumValue::Top,
                font::VerticalAlignKeyword::Bottom => EnumValue::Bottom,
              }
            },
            font::VerticalAlign::Length(_) => EnumValue::Invalid,
        }
        }
        _ => EnumValue::Invalid,
      }
    }
  }
}
