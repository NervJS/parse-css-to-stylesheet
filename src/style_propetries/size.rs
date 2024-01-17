use lightningcss::{
  properties::{Property, size::Size::{ Auto, LengthPercentage }},
  values::{length::LengthValue, percentage::{DimensionPercentage, Percentage}}, traits::ToCss, stylesheet::PrinterOptions,
};
use swc_ecma_ast::Expr;

use crate::{generate_expr_lit_str, generate_size_property};

use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};

// 生成property_name的value类型为 Size的属性
generate_size_property![SizeProperty, Height, Width];