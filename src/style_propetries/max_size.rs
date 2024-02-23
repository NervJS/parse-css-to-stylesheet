use lightningcss::{
  properties::size::MaxSize::LengthPercentage,
  traits::ToCss
};

use crate::{generate_expr_lit_str, generate_size_property};
use super::unit::PropertyTuple;
use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};

// 生成property_name的value类型为 Size的属性
generate_size_property![MaxSizeProperty, MaxHeight, MaxWidth];

