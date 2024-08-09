use super::style_property_type::CSSPropertyType;
use crate::generate_size_property;

use lightningcss::properties::size::Size::LengthPercentage;

// 生成property_name的value类型为 Size的属性

// 使用宏生成 ColorProperty 结构体
generate_size_property![
  SizeProperty,
  (CSSPropertyType::Height, Height),
  (CSSPropertyType::Width, Width),
  (CSSPropertyType::MinWidth, MinWidth),
  (CSSPropertyType::MinHeight, MinHeight)
];
