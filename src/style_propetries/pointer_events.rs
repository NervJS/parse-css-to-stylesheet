use lightningcss::{ printer::PrinterOptions, properties::Property };

use crate::{ generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum };

use super::{ style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple };

#[derive(Debug, Clone)]
pub struct PointerEvents {
    pub id: String,
    pub value: EnumValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
    Auto,
    None,
    Invalid,
}

impl From<(String, &Property<'_>)> for PointerEvents {
    fn from(value: (String, &Property<'_>)) -> Self {
        let val = value.1
            .value_to_css_string(PrinterOptions::default())
            .unwrap_or(String::from(""));
        PointerEvents {
            id: value.0,
            value: match val.as_str() {
                "none" => EnumValue::None,
                "auto" => EnumValue::Auto,
                _ => EnumValue::Invalid,
            },
        }
    }
}

impl ToExpr for PointerEvents {
    fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(CSSPropertyType::PointerEvents, match &self.value {
            EnumValue::None => generate_expr_enum!(style_property_enum::PointerEvents::None),
            EnumValue::Auto => generate_expr_enum!(style_property_enum::PointerEvents::Auto),
            EnumValue::Invalid => generate_invalid_expr!(),
        })
    }
}
