use crate::generate_length_value_property;

use super::style_property_type::CSSPropertyType;

// 生成property_name的value类型为 LengthValue的属性
generate_length_value_property![
  LengthValueProperty,
  (CSSPropertyType::MarginTop, MarginTop),
  (CSSPropertyType::MarginBottom, MarginBottom),
  (CSSPropertyType::MarginLeft, MarginLeft),
  (CSSPropertyType::MarginRight, MarginRight),
  (CSSPropertyType::PaddingTop, PaddingTop),
  (CSSPropertyType::PaddingBottom, PaddingBottom),
  (CSSPropertyType::PaddingLeft, PaddingLeft),
  (CSSPropertyType::PaddingRight, PaddingRight),
  (CSSPropertyType::Left, Left),
  (CSSPropertyType::Right, Right),
  (CSSPropertyType::Top, Top),
  (CSSPropertyType::Bottom, Bottom)
];
