use lightningcss::{
  properties::size::Size::LengthPercentage,
  traits::ToCss
};

use crate::{generate_expr_lit_str, generate_size_property, generate_prop_name};
use super::unit::PropertyTuple;
use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};

// 生成property_name的value类型为 Size的属性
generate_size_property![SizeProperty, Height, Width];