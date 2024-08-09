use lightningcss::properties::{font, Property};

use crate::{
  generate_expr_enum,
  style_propetries::{style_property_enum, traits::ToExpr},
};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FontWeight {
  pub id: String,
  pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Number(f32),
  Bold,
  Bolder,
  Lighter,
  Normal,
}

impl ToExpr for FontWeight {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FontWeight,
      match &self.value {
        EnumValue::Bold => {
          generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_BOLD)
        }
        EnumValue::Bolder => {
          generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_BOLDER)
        }
        EnumValue::Lighter => {
          generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_LIGHTER)
        }
        EnumValue::Normal => {
          generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_NORMAL)
        }
        EnumValue::Number(num) => {
          // 取100～900的整数枚举值
          let new_num = ((num / 100.0).ceil() * 100.0) as i32;
          match new_num {
            100 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W100)
            }
            200 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W200)
            }
            300 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W300)
            }
            400 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W400)
            }
            500 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W500)
            }
            600 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W600)
            }
            700 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W700)
            }
            800 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W800)
            }
            900 => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_W900)
            }
            _ => {
              generate_expr_enum!(style_property_enum::ArkUI_FontWeight::ARKUI_FONT_WEIGHT_NORMAL)
            }
          }
        }
      },
    )
  }
}

impl From<(String, &Property<'_>)> for FontWeight {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontWeight {
      id: prop.0,
      value: match prop.1 {
        Property::FontWeight(value) => match value {
          font::FontWeight::Bolder => EnumValue::Bolder,
          font::FontWeight::Lighter => EnumValue::Lighter,
          font::FontWeight::Absolute(val) => match val {
            font::AbsoluteFontWeight::Bold => EnumValue::Bold,
            font::AbsoluteFontWeight::Weight(num) => EnumValue::Number(*num),
            font::AbsoluteFontWeight::Normal => EnumValue::Normal,
          },
        },
        _ => EnumValue::Normal,
      },
    }
  }
}
