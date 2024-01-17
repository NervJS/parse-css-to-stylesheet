use lightningcss::{properties::Property, values::number::CSSNumber};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number};

use crate::generate_number_property;

use crate::style_propetries::traits::ToExpr;

generate_number_property!(NumberProperty, FlexGrow, FlexShrink);
