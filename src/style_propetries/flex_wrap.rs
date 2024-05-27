use lightningcss::properties::{flex::FlexWrap as LNFlexWrap, Property};

use crate::{generate_expr_enum, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct FlexWrap {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Wrap,
  WrapReverse,
  NoWrap,
}

impl From<(String, &Property<'_>)> for FlexWrap {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexWrap {
      id: prop.0,
      value: match prop.1 {
        Property::FlexWrap(value, _) => match value {
          LNFlexWrap::Wrap => EnumValue::Wrap,
          LNFlexWrap::WrapReverse => EnumValue::WrapReverse,
          LNFlexWrap::NoWrap => EnumValue::NoWrap,
        },
        _ => EnumValue::NoWrap,
      }
    }
  }
}

impl ToExpr for FlexWrap {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FlexWrap,
      match self.value {
        EnumValue::Wrap => generate_expr_enum!(style_property_enum::FlexWrap::Wrap),
        EnumValue::WrapReverse => generate_expr_enum!(style_property_enum::FlexWrap::WrapReverse),
        EnumValue::NoWrap => generate_expr_enum!(style_property_enum::FlexWrap::NoWrap),
      }
    )
  }

}
