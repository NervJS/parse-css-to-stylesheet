use lightningcss::{
    properties::{ Property, font },
    values::{ length::LengthValue, percentage::Percentage },
    traits::ToCss,
};

use crate::{
    style_propetries::traits::ToExpr,
    generate_dimension_percentage,
    generate_invalid_expr,
};

use super::{
    style_property_type::CSSPropertyType,
    unit::{ generate_expr_by_length_value, generate_expr_with_css_input, Platform, PropertyTuple },
};

#[derive(Debug, Clone)]
pub struct LineHeight {
    pub id: String,
    pub value: EnumValue,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
    LengthValue(LengthValue),
    Percentage(Percentage),
    String(String),
    Invalid,
}

impl ToExpr for LineHeight {
    fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(CSSPropertyType::LineHeight, match &self.value {
            EnumValue::String(val) => {
                generate_expr_with_css_input(val.to_string(), Platform::Harmony).into()
            }
            EnumValue::LengthValue(length_value) =>
                generate_expr_by_length_value(&length_value, Platform::Harmony),
            EnumValue::Percentage(_) => { generate_invalid_expr!() }
            EnumValue::Invalid => generate_invalid_expr!(),
        })
    }
}

impl From<(String, &Property<'_>)> for LineHeight {
    fn from(prop: (String, &Property<'_>)) -> Self {
        LineHeight {
            id: prop.0,
            value: match prop.1 {
                Property::LineHeight(value) => {
                    match value {
                        font::LineHeight::Length(val) => {
                            generate_dimension_percentage!(EnumValue, val)
                        }
                        // RN和鸿蒙都不支持数值类型：https://github.com/NervJS/taro/issues/11620
                        font::LineHeight::Number(val) => {
                            EnumValue::String(format!("{}%", *val * 100.0))
                        }
                        font::LineHeight::Normal => { EnumValue::Invalid }
                    }
                }
                _ => EnumValue::Invalid,
            },
        }
    }
}
