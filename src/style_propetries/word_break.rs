use lightningcss::properties::{ text, Property };
use crate::{ generate_expr_enum, style_propetries::style_property_enum };

use super::{ style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple };
#[derive(Debug, Clone)]
pub struct WordBreak {
    pub id: String,
    pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
    NORMAL,
    BREAKALL,
    BREAKWORD,
}

impl From<(String, &Property<'_>)> for WordBreak {
    fn from(value: (String, &Property<'_>)) -> Self {
        WordBreak {
            id: value.0,
            value: {
                if let Property::WordBreak(value) = &value.1 {
                    match &value {
                        text::WordBreak::Normal => { EnumValue::NORMAL }
                        text::WordBreak::BreakAll => { EnumValue::BREAKALL }
                        text::WordBreak::BreakWord => { EnumValue::BREAKWORD }
                        _ => EnumValue::NORMAL,
                    }
                } else {
                    EnumValue::NORMAL
                }
            },
        }
    }
}

impl ToExpr for WordBreak {
    fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(CSSPropertyType::WordBreak, match &self.value {
            EnumValue::NORMAL =>
                generate_expr_enum!(style_property_enum::ArkUI_WordBreak::ARKUI_WORD_BREAK_NORMAL),
            EnumValue::BREAKALL =>
                generate_expr_enum!(
                    style_property_enum::ArkUI_WordBreak::ARKUI_WORD_BREAK_BREAK_ALL
                ),
            EnumValue::BREAKWORD =>
                generate_expr_enum!(
                    style_property_enum::ArkUI_WordBreak::ARKUI_WORD_BREAK_BREAK_WORD
                ),
        })
    }
}
