use lightningcss::properties::{flex::FlexDirection as LNFlexDirection, Property};

use crate::{generate_expr_enum, style_propetries::style_property_enum};

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct FlexDirection {
  pub id: String,
  pub value: EnumValue
}


#[derive(Debug, Clone)]
pub enum EnumValue {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl From<(String, &Property<'_>)> for FlexDirection {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FlexDirection {
      id: prop.0,
      value: match prop.1 {
        Property::FlexDirection(value, _) => match value {
          LNFlexDirection::Row => EnumValue::Row,
          LNFlexDirection::RowReverse => EnumValue::RowReverse,
          LNFlexDirection::Column => EnumValue::Column,
          LNFlexDirection::ColumnReverse => EnumValue::ColumnReverse,
        },
        _ => EnumValue::Row,
      }
    }
  }
}

impl ToExpr for FlexDirection {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FlexDirection,
      match self.value {
        EnumValue::Row => generate_expr_enum!(style_property_enum::ArkUI_FlexDirection::ARKUI_FLEX_DIRECTION_ROW),
        EnumValue::RowReverse => generate_expr_enum!(style_property_enum::ArkUI_FlexDirection::ARKUI_FLEX_DIRECTION_ROW_REVERSE),
        EnumValue::Column => generate_expr_enum!(style_property_enum::ArkUI_FlexDirection::ARKUI_FLEX_DIRECTION_COLUMN),
        EnumValue::ColumnReverse => generate_expr_enum!(style_property_enum::ArkUI_FlexDirection::ARKUI_FLEX_DIRECTION_COLUMN_REVERSE),
      }
    )
  }
}
