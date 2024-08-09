use lightningcss::properties::Property;

use crate::generate_expr_lit_num;

use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct AspectRatio {
  pub id: String,
  pub value: EAspectRatio,
}

#[derive(Debug, Clone)]
pub enum EAspectRatio {
  Auto,
  Ratio(f64, f64),
}

impl From<(String, &Property<'_>)> for AspectRatio {
  fn from(prop: (String, &Property<'_>)) -> Self {
    match prop.1 {
      Property::AspectRatio(value) => match value.auto {
        true => AspectRatio {
          id: prop.0,
          value: EAspectRatio::Auto,
        },
        false => {
          if let Some(ratio) = &value.ratio {
            AspectRatio {
              id: prop.0,
              value: EAspectRatio::Ratio(ratio.0.into(), ratio.1.into()),
            }
          } else {
            AspectRatio {
              id: prop.0,
              value: EAspectRatio::Auto,
            }
          }
        }
      },
      _ => AspectRatio {
        id: prop.0,
        value: EAspectRatio::Auto,
      },
    }
  }
}

impl ToExpr for AspectRatio {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::AspectRatio,
      match self.value {
        EAspectRatio::Ratio(first, second) => generate_expr_lit_num!(first / second),
        _ => generate_expr_lit_num!(1.0),
      },
    )
  }
}
