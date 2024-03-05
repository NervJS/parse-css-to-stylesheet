use lightningcss::traits::ToCss;

use crate::{generate_expr_lit_str, generate_length_value_property, generate_dimension_percentage, generate_invalid_expr};
use super::unit::PropertyTuple;
use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};

// 生成property_name的value类型为 LengthValue的属性
generate_length_value_property![LengthValueProperty, MarginTop, MarginBottom, MarginLeft, MarginRight, PaddingTop, PaddingBottom, PaddingLeft, PaddingRight, Left, Right, Top, Bottom];

