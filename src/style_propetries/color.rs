
use crate::{generate_color_property, generate_ident};
use super::{traits::ToExpr, unit::PropertyTuple};
// 生成property_name的value类型为 Size的属性
generate_color_property![ColorProperty, Color];
