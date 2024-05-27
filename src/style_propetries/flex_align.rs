use lightningcss::properties::{
  align::{
    AlignContent as LNAlignContent, ContentDistribution, ContentPosition,
    JustifyContent as LNJustifyContent,
  },
  Property,
};

use crate::{generate_expr_enum, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FlexAlign {
  pub id: CSSPropertyType,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Start,
  Center,
  End,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}


impl From<(String, &Property<'_>)> for FlexAlign {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexAlign {
      id: if prop.0 == "justifyContent" { CSSPropertyType::JustifyContent } else { CSSPropertyType::AlignContent },
      value: match prop.1 {
        Property::JustifyContent(value, _) => match value {
          LNJustifyContent::ContentPosition { value, .. } => match value {
            ContentPosition::Start | ContentPosition::FlexStart => EnumValue::Start,
            ContentPosition::Center => EnumValue::Center,
            ContentPosition::End | ContentPosition::FlexEnd => EnumValue::End,
          },
          LNJustifyContent::ContentDistribution(value) => match value {
            ContentDistribution::SpaceBetween => EnumValue::SpaceBetween,
            ContentDistribution::SpaceAround => EnumValue::SpaceAround,
            ContentDistribution::SpaceEvenly => EnumValue::SpaceEvenly,
            _ => EnumValue::Start,
          },
          _ => EnumValue::Start,
        },
        Property::AlignContent(value, _) => match value {
          LNAlignContent::ContentPosition { value, .. } => match value {
            ContentPosition::Start | ContentPosition::FlexStart => EnumValue::Start,
            ContentPosition::Center => EnumValue::Center,
            ContentPosition::End | ContentPosition::FlexEnd => EnumValue::End,
          },
          LNAlignContent::ContentDistribution(value) => match value {
            ContentDistribution::SpaceBetween => EnumValue::SpaceBetween,
            ContentDistribution::SpaceAround => EnumValue::SpaceAround,
            ContentDistribution::SpaceEvenly => EnumValue::SpaceEvenly,
            _ => EnumValue::Start,
          },
          _ => EnumValue::Start,
        },
        _ => EnumValue::Start,
      }
    }
  }
}

// 转换成鸿蒙样式
impl ToExpr for FlexAlign {

  // 转换成鸿蒙样式
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One (
      self.id,
      match self.value {
        EnumValue::Start => generate_expr_enum!(style_property_enum::FlexAlign::Start),
        EnumValue::Center => generate_expr_enum!(style_property_enum::FlexAlign::Center),
        EnumValue::End => generate_expr_enum!(style_property_enum::FlexAlign::End),
        EnumValue::SpaceBetween => generate_expr_enum!(style_property_enum::FlexAlign::SpaceBetween),
        EnumValue::SpaceAround => generate_expr_enum!(style_property_enum::FlexAlign::SpaceAround),
        EnumValue::SpaceEvenly => generate_expr_enum!(style_property_enum::FlexAlign::SpaceEvenly),
      }
    )
  }

}
