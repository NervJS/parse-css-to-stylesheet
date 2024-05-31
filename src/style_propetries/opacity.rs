use lightningcss::properties::{Property, text};

use crate::{generate_expr_enum, generate_expr_lit_num, generate_invalid_expr, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct Opacity {
  pub id: String,
  pub value: Option<f32>
}


impl ToExpr for Opacity {
  fn to_expr(&self) -> PropertyTuple {
    if let Some(value) = self.value {
      return PropertyTuple::One(
        CSSPropertyType::Opacity,
        generate_expr_lit_num!(value as f64)
      )
    } else {
      return PropertyTuple::One(
        CSSPropertyType::Invalid,
        generate_invalid_expr!()
      )
    }
  }
}

impl From<(String, &Property<'_>)> for Opacity {
  fn from(prop: (String, &Property<'_>)) -> Self {
    Opacity {
      id: prop.0,
      value: match prop.1 {
        Property::Opacity(value) => {
          Some(value.0)
        },
        _ => None
      }
    }
  }
}
