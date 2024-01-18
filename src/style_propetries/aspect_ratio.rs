

use lightningcss::properties::Property;

use crate::{generate_expr_lit_num, generate_expr_ident, generate_ident};

use super::{traits::ToExpr, unit::PropertyTuple};

#[derive(Debug, Clone)]
pub struct AspactRatio {
  pub id: String,
  pub value: EAspactRatio
}

#[derive(Debug, Clone)]
pub enum EAspactRatio {
  Auto,
  Ratio(f64, f64),
}

impl From<(String, &Property<'_>)> for AspactRatio {
  fn from(prop: (String, &Property<'_>)) -> Self {
    match prop.1 {
      Property::AspectRatio(value) => {
        match value.auto {
          true => AspactRatio { id: prop.0, value: EAspactRatio::Auto },
          false => {
            if let Some(ratio) = &value.ratio {
              AspactRatio { id: prop.0, value: EAspactRatio::Ratio(ratio.0.into(), ratio.1.into()) }
            } else {
              AspactRatio { id: prop.0, value: EAspactRatio::Auto }
            }
          }
        }
      },
      _ => AspactRatio { id: prop.0, value: EAspactRatio::Auto },
    }
  }
}

impl ToExpr for AspactRatio {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_ident!("aspectRatio"),
      match self.value {
        EAspactRatio::Ratio(first, second) => generate_expr_lit_num!(first / second),
        _ => generate_expr_lit_num!(1.0),
      }
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      generate_ident!("aspectRatio"),
      match self.value {
        EAspactRatio::Ratio(first, second) => generate_expr_lit_num!(first / second),
        _ => generate_expr_ident!("auto"),
      }
    )
  }
}