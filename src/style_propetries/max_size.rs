use lightningcss::properties::size::MaxSize::LengthPercentage;

use super::style_property_type::CSSPropertyType;
use crate::generate_size_property;
// 生成property_name的value类型为 Size的属性
generate_size_property![
  MaxSizeProperty,
    (CSSPropertyType::MaxHeight, MaxHeight),
    (CSSPropertyType::MaxWidth, MaxWidth)
];