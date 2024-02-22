use lightningcss::properties::Property;

use crate::{generate_invalid_expr, generate_prop_name, generate_tpl_expr};

use super::{flex_basis::FlexBasis, number::NumberProperty, traits::ToExpr, unit::PropertyTuple};



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
      match value.basis.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push((generate_prop_name!("flexBasis"), val)),
        _ => {}
      };
      match value.grow.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push((generate_prop_name!("flexGrow"), val)),
        _ => {}
      };
      match value.shrink.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push((generate_prop_name!("flexShrink"), val)),
        _ => {}
      };
      PropertyTuple::Array(props)
    } else {
      PropertyTuple::One(
        generate_prop_name!("flex"),
        generate_invalid_expr!()
      )
    }
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    let mut props = vec![];

    if let Some(value) = &self.value {
      match value.grow.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push(val),
        _ => {}
      };
      match value.shrink.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push(val),
        _ => {}
      };
      match value.basis.to_rn_expr() {
        PropertyTuple::One(_, val) => props.push(val),
        _ => {}
      };
      PropertyTuple::One(
        generate_prop_name!("flex"),
        generate_tpl_expr!(props)
      )
    } else {
      PropertyTuple::One(
        generate_prop_name!("flex"),
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
            grow: NumberProperty::from_value(("flexGrow".to_string(), flex.grow)),
            shrink: NumberProperty::from_value(("flexShrink".to_string(),flex.shrink))
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