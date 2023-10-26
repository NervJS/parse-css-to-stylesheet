use lightningcss::{properties::Property, values::number::CSSNumber};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number};

use crate::generate_flex_number;

use crate::style_transform::traits::ToExpr;

generate_flex_number!(FlexGrow, FlexGrow);
