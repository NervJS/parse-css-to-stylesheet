use crate::{style_propetries::traits::ToExpr, generate_number_property};

use super::unit::PropertyTuple;

generate_number_property!(NumberProperty, FlexGrow, FlexShrink);