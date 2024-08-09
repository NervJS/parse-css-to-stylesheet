use crate::generate_number_property;

use super::style_property_type::CSSPropertyType;

generate_number_property![
  NumberProperty,
  (CSSPropertyType::FlexShrink, FlexShrink),
  (CSSPropertyType::FlexGrow, FlexGrow)
];
