use std::vec;

use lightningcss::{
  properties::Property, values::length::LengthPercentageOrAuto
};
use crate::{generate_expr_by_length_percentage, generate_expr_by_length_percentage_or_auto, generate_expr_lit_str, generate_invalid_expr, style_propetries::traits::ToExpr};

use super::{style_property_type::CSSPropertyType, unit::{Platform, PropertyTuple}};


#[derive(Debug, Clone)]
pub struct MarginPadding {
  pub id: String,
  top: Option<LengthPercentageOrAuto>,
  right: Option<LengthPercentageOrAuto>,
  bottom: Option<LengthPercentageOrAuto>,
  left: Option<LengthPercentageOrAuto>,
}

impl MarginPadding {
  
  pub fn new(id: String) -> Self {
    MarginPadding {
      id,
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }
}

impl ToExpr for MarginPadding {
  fn to_expr(&self) -> PropertyTuple {
    if self.top.is_none() || self.right.is_none() || self.bottom.is_none() || self.left.is_none() {
      return PropertyTuple::One(
        CSSPropertyType::Invaild,
        generate_invalid_expr!()
      )
    }

    // 判断self.id是否padding开头
    let is_padding = self.id.starts_with("padding");
    
    PropertyTuple::Array(vec![
      (if is_padding { CSSPropertyType::PaddingTop } else { CSSPropertyType::MarginTop }, generate_expr_by_length_percentage_or_auto!(self.top.as_ref().unwrap(), Platform::Harmony)),
      (if is_padding { CSSPropertyType::PaddingRight } else { CSSPropertyType::MarginRight }, generate_expr_by_length_percentage_or_auto!(self.right.as_ref().unwrap(), Platform::Harmony)),
      (if is_padding { CSSPropertyType::PaddingBottom } else { CSSPropertyType::MarginBottom }, generate_expr_by_length_percentage_or_auto!(self.bottom.as_ref().unwrap(), Platform::Harmony)),
      (if is_padding { CSSPropertyType::PaddingLeft } else { CSSPropertyType::MarginLeft }, generate_expr_by_length_percentage_or_auto!(self.left.as_ref().unwrap(), Platform::Harmony)),
    ])
  }
}

impl From<(String, &Property<'_>)> for MarginPadding {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut margin_padding = MarginPadding::new(prop.0);
    match prop.1 {
      Property::Margin(value) => {
        margin_padding.top = Some(value.top.clone());
        margin_padding.right = Some(value.right.clone());
        margin_padding.bottom = Some(value.bottom.clone());
        margin_padding.left = Some(value.left.clone());
      },
      Property::Padding(value) => {
        margin_padding.top = Some(value.top.clone());
        margin_padding.right = Some(value.right.clone());
        margin_padding.bottom = Some(value.bottom.clone());
        margin_padding.left = Some(value.left.clone());
      },
      _ => {}
    };
    margin_padding
  }
}
