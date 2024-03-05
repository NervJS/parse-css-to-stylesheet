use super::{traits::ToExpr, unit::{generate_expr_with_css_input, Platform, PropertyTuple}};



#[derive(Debug, Clone)]
pub struct Normal(String, String);

impl Normal {
  pub fn new(id: String, value: String) -> Self {
    Self(id, value)
  }
}


impl ToExpr for Normal {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.0.clone(),
      generate_expr_with_css_input(self.1.clone(), Platform::Harmony)
    )
  }

  fn to_rn_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.0.clone(),
      generate_expr_with_css_input(self.1.clone(), Platform::ReactNative)
    )
  }
}

