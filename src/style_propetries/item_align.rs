use lightningcss::properties::{
  align::AlignItems as LNAlignItems, align::AlignSelf as LNAlignSelf, align::BaselinePosition,
  align::SelfPosition, Property,
};

use crate::{generate_expr_enum, generate_invalid_expr, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct ItemAlign {
  pub id: CSSPropertyType,
  pub value: EnumValue
}


#[derive(Debug, Clone, PartialEq)]
pub enum EnumValue {
  Auto,
  Start,
  Center,
  End,
  Stretch,
  Baseline,
  Ignore,
}

impl From<(String, &Property<'_>)> for ItemAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    ItemAlign {
      id: if prop.0 == "alignItems" { CSSPropertyType::AlignItems } else { CSSPropertyType::AlignSelf },
      value: match prop.1 {
        Property::AlignItems(value, _) => match value {
          LNAlignItems::Stretch => EnumValue::Stretch,
          LNAlignItems::SelfPosition { value, .. } => match value {
            SelfPosition::Start | SelfPosition::FlexStart => EnumValue::Start,
            SelfPosition::Center => EnumValue::Center,
            SelfPosition::End | SelfPosition::FlexEnd => EnumValue::End,
            _ => EnumValue::Ignore,
          },
          LNAlignItems::BaselinePosition(value) => match value {
            BaselinePosition::Last => EnumValue::Ignore,
            _ => EnumValue::Baseline,
          },
          _ => EnumValue::Auto,
        },
        Property::AlignSelf(value, _) => match value {
          LNAlignSelf::Auto => EnumValue::Auto,
          LNAlignSelf::SelfPosition { value, .. } => match value {
            SelfPosition::Start | SelfPosition::FlexStart => EnumValue::Start,
            SelfPosition::Center => EnumValue::Center,
            SelfPosition::End | SelfPosition::FlexEnd => EnumValue::End,
            _ => EnumValue::Ignore,
          },
          LNAlignSelf::Stretch => EnumValue::Stretch,
          LNAlignSelf::BaselinePosition(value) => match value {
            BaselinePosition::Last => EnumValue::Ignore,
            _ => EnumValue::Baseline,
          },
          _ => EnumValue::Auto,
        },
        _ => EnumValue::Auto,
      }
    }
  }
}

impl ToExpr for ItemAlign {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.id,
      match &self.value {
        EnumValue::Auto => generate_expr_enum!(style_property_enum::ItemAlign::Auto),
        EnumValue::Start => generate_expr_enum!(style_property_enum::ItemAlign::Start),
        EnumValue::Center => generate_expr_enum!(style_property_enum::ItemAlign::Center),
        EnumValue::End => generate_expr_enum!(style_property_enum::ItemAlign::End),
        EnumValue::Stretch => generate_expr_enum!(style_property_enum::ItemAlign::Stretch),
        EnumValue::Baseline => generate_expr_enum!(style_property_enum::ItemAlign::Baseline),
        EnumValue::Ignore => generate_invalid_expr!(),
      }
    )
  }

}
