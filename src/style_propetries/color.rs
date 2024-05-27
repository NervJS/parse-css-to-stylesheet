
use crate::generate_color_property;

use super::style_property_type::CSSPropertyType;

// 生成property_name的value类型为 Size的属性
// 使用宏生成 ColorProperty 结构体
generate_color_property![
    ColorProperty,
    (CSSPropertyType::Color, Color),
    (CSSPropertyType::BackgroundColor, BackgroundColor)
];