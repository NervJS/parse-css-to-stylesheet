use swc_common::DUMMY_SP;
use swc_ecma_ast::Number;

use crate::{style_propetries::traits::ToExpr, generate_number_property, generate_ident};

use super::unit::PropertyTuple;

generate_number_property!(NumberProperty, FlexGrow, FlexShrink);