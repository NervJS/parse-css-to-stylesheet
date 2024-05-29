use lightningcss::properties::Property;

use crate::generate_invalid_expr;

use super::{flex_basis::FlexBasis, number::NumberProperty, style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};



#[derive(Debug, Clone)]
pub struct Flex {
  pub id: String,
  pub value: Option<FlexValue>
}

#[derive(Debug, Clone)]
pub struct FlexValue {
  grow: NumberProperty,
  shrink: NumberProperty,
  basis: FlexBasis
}

impl ToExpr for Flex {
  fn to_expr(&self) -> PropertyTuple {
    let mut props = vec![];
    if let Some(value) = &self.value {
      match value.basis.to_expr() {
        PropertyTuple::One(_, val) => props.push((CSSPropertyType::FlexBasis, val)),
        _ => {}
      };
      match value.grow.to_expr() {
        PropertyTuple::One(_, val) => props.push((CSSPropertyType::FlexGrow, val)),
        _ => {}
      };
      match value.shrink.to_expr() {
        PropertyTuple::One(_, val) => props.push((CSSPropertyType::FlexShrink, val)),
        _ => {}
      };
      PropertyTuple::Array(props)
    } else {
      PropertyTuple::One(
        CSSPropertyType::Invalid,
        generate_invalid_expr!()
      )
    }
  }

}

impl From<(String, &Property<'_>)> for Flex {
  fn from(prop: (String, &Property<'_>)) -> Self {
    
    match &prop.1 {
      Property::Flex(flex, _) => {
        Flex {
          id: prop.0,
          value: Some(FlexValue {
            basis: FlexBasis::from_value(("flexBasis".to_string(), flex.basis.clone())),
            grow: NumberProperty::from_value(("flexGrow".to_string(), (CSSPropertyType::FlexGrow, flex.grow))),
            shrink: NumberProperty::from_value(("flexShrink".to_string(), (CSSPropertyType::FlexShrink, flex.shrink)))
          })
        }
      },
      _ => {
        Flex {
          id: prop.0,
          value: None
        }
      }
    }
  }
}