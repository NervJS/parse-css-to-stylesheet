use lightningcss::properties::{Property, font};

use crate::{generate_expr_enum, generate_expr_lit_num, style_propetries::{style_property_enum, traits::ToExpr}};

use super::{style_property_type::CSSPropertyType, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub struct FontWeight {
  pub id: String,
  pub value: EnumValue
}

#[derive(Debug, Clone)]
pub enum EnumValue {
  Number(f32),
  Bold,
  Bolder,
  Lighter,
  Normal
}

impl ToExpr for FontWeight {
  fn to_expr(&self) -> PropertyTuple {
    PropertyTuple::One(
      CSSPropertyType::FontWeight,
      match &self.value {
        EnumValue::Bold => generate_expr_enum!(style_property_enum::FontWeight::Bold),
        EnumValue::Bolder => generate_expr_enum!(style_property_enum::FontWeight::Bolder),
        EnumValue::Lighter => generate_expr_enum!(style_property_enum::FontWeight::Lighter),
        EnumValue::Normal => generate_expr_enum!(style_property_enum::FontWeight::Normal),
        EnumValue::Number(num) => {
          generate_expr_lit_num!(*num as f64)
        },
      }
    )
  }

}

impl From<(String, &Property<'_>)> for FontWeight {
  fn from(prop: (String, &Property<'_>)) -> Self {
    FontWeight {
      id: prop.0,
      value: match prop.1 {
        Property::FontWeight(value) => {
          match value {
            font::FontWeight::Bolder => EnumValue::Bolder,
            font::FontWeight::Lighter => EnumValue::Lighter,
            font::FontWeight::Absolute(val) => {
              match val {
                font::AbsoluteFontWeight::Bold => {
                  EnumValue::Bold
                },
                font::AbsoluteFontWeight::Weight(num) => {
                  EnumValue::Number(*num)
                },
                font::AbsoluteFontWeight::Normal => {
                  EnumValue::Normal
                },
              }
            },
          }
        }
        _ => EnumValue::Normal
      }
    }
  }
}
