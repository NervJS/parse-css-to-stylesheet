use super::{style_property_type::CSSPropertyType, traits::ToExpr, unit::{generate_expr_with_css_input, Platform, PropertyTuple}};



#[derive(Debug, Clone)]
pub struct Normal(CSSPropertyType, String);

impl Normal {
  pub fn new(id: CSSPropertyType, value: String) -> Self {
    Self(id, value)
  }
}


impl ToExpr for Normal {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      self.0,
      generate_expr_with_css_input(self.1.clone(), Platform::Harmony)
    )
  }
}

