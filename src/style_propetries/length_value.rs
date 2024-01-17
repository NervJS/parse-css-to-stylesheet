use lightningcss::{
  properties::Property,
  values::{length::{LengthPercentageOrAuto, LengthValue}, percentage::{DimensionPercentage, Percentage}}, traits::ToCss, stylesheet::PrinterOptions,
};
use swc_ecma_ast::Expr;

use crate::{generate_expr_lit_str, generate_length_value_property};

use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};

// 生成property_name的value类型为 LengthValue的属性
generate_length_value_property![LengthValueProperty, MarginTop, MarginBottom, MarginLeft, MarginTop, PaddingTop, PaddingBottom, PaddingLeft, PaddingTop, Left, Right, Top, Bottom];