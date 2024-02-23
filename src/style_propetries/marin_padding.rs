use std::vec;

use lightningcss::{
  properties::Property, values::length::LengthPercentageOrAuto
};

use crate::{generate_expr_by_length_percentage, generate_expr_by_length_percentage_or_auto, generate_expr_lit_str, generate_invalid_expr, generate_tpl_expr, style_propetries::traits::ToExpr};

use super::unit::{Platform, PropertyTuple};


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
      id: id,
      top: None,
      right: None,
      bottom: None,
      left: None,
    }
  }
}

impl ToExpr for MarginPadding {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::Array(vec![
      ("marginTop".to_string(), generate_expr_by_length_percentage_or_auto!(self.top.as_ref().unwrap(), Platform::Harmony)),
      ("marginRight".to_string(), generate_expr_by_length_percentage_or_auto!(self.right.as_ref().unwrap(), Platform::Harmony)),
      ("marginBottom".to_string(), generate_expr_by_length_percentage_or_auto!(self.bottom.as_ref().unwrap(), Platform::Harmony)),
      ("marginLeft".to_string(), generate_expr_by_length_percentage_or_auto!(self.left.as_ref().unwrap(), Platform::Harmony)),
    
    ])
  }

  fn to_rn_expr(&self) -> PropertyTuple {

    let margin_padding = vec![
      generate_expr_by_length_percentage_or_auto!(self.top.as_ref().unwrap(), Platform::ReactNative), 
      generate_expr_by_length_percentage_or_auto!(self.right.as_ref().unwrap(), Platform::ReactNative), 
      generate_expr_by_length_percentage_or_auto!(self.bottom.as_ref().unwrap(), Platform::ReactNative), 
      generate_expr_by_length_percentage_or_auto!(self.left.as_ref().unwrap(), Platform::ReactNative), 
    ];

    PropertyTuple::One(
      self.id.clone(),
      // 生成`${top} ${right} ${bottom} ${left}`
      generate_tpl_expr!(margin_padding)
    )    
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
