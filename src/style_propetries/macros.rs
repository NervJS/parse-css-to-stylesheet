#[macro_export]
macro_rules! generate_prop_name {
  ($key: expr) => {{
    use swc_core::common::DUMMY_SP;
    use swc_core::ecma::ast::*;
    PropName::Ident(Ident::new($key.into(), DUMMY_SP))
  }};
}

#[macro_export]
macro_rules! generate_expr_lit_str {
  ($var:expr) => {{
    generate_expr_lit_str!($var, true)
  }};
  ($var:expr, $convert_lpx:expr) => {{
    use swc_core::ecma::ast::*;
    
    let re = regex::Regex::new(r#"\b(\d+(?:px|vw|vh))\b"#).unwrap();
    let var_str = $var.to_string();
    let result = re.replace_all(var_str.as_str(), |caps: &regex::Captures| {
      let value = &caps[1];
      let unit = &value[value.len() - 2..];
      let parsed_value: i32 = value[..value.len() - 2].parse().unwrap();
      if unit == "px" {
        if $convert_lpx {
          format!("{}lpx", parsed_value)
        } else {
          format!("{}vp", parsed_value)
        }
      } else {
        format!("{}{}", parsed_value, unit)
      }
    });

    Expr::Lit(Lit::Str(result.into()))
  }};
}

#[macro_export]
macro_rules! generate_expr_lit_num {
  ($var:expr) => {{
    use swc_core::ecma::ast::*;
    let rounded = ((($var as f64) * 100.0).round() / 100.0);
    Expr::Lit(Lit::Num(Number::from(rounded)))
  }};
}

#[macro_export]
macro_rules! generate_expr_lit_bool {
  ($var:expr) => {{
    use swc_core::ecma::ast::*;
    Expr::Lit(Lit::Bool(Bool {
      span: DUMMY_SP,
      value: $var,
    }))
  }};
}

#[macro_export]
macro_rules! generate_expr_enum {
  ($var:expr) => {{
    use swc_core::ecma::ast::*;
    Expr::Lit(Lit::Num(Number::from(($var as u32) as f64)))
  }};
}

#[macro_export]
macro_rules! generate_expr_lit_color {
  ($color:expr) => {{
    use lightningcss::traits::ToCss;
    use swc_core::ecma::ast::*;
    use $crate::style_propetries::unit::convert_color_keywords_to_hex;
    use $crate::utils::hex_to_argb;

    let color_string = convert_color_keywords_to_hex(
      $color
        .to_css_string(lightningcss::stylesheet::PrinterOptions {
          minify: false,
          targets: lightningcss::targets::Targets::default(),
          ..lightningcss::stylesheet::PrinterOptions::default()
        })
        .unwrap(),
    );

    match hex_to_argb(&color_string) {
      Ok(argb) => Expr::Lit(Lit::Num(Number::from(argb as f64))),
      Err(hex_or_rgba) => Expr::Lit(Lit::Str(hex_or_rgba.clone().into())).into(),
    }
  }};
}

#[macro_export]
macro_rules! generate_expr_lit_calc {
  ($var:expr, $platform:expr) => {{
    use $crate::generate_expr_lit_str;
    // 扫描 $var 中是存在px单位，如果存在，替换为lpx
    let re = regex::Regex::new(r#"(\d+(?:px|vw|vh))"#).unwrap();
    let result = re.replace_all($var.as_str(), |caps: &regex::Captures| {
      let value = &caps[1];
      let unit = &value[value.len() - 2..];
      let parsed_value: i32 = value[..value.len() - 2].parse().unwrap();
      if $platform == Platform::Harmony {
        if unit == "px" {
          format!("{}lpx", parsed_value)
        } else {
          format!("{}{}", parsed_value, unit)
        }
      } else {
        format!("{}px", parsed_value)
      }
    });
    generate_expr_lit_str!(result.to_string())

    // use swc_core::ecma::ast::*;
    // use swc_core::{
    //   common::DUMMY_SP,
    //   atoms::Atom
    // };

    // use $crate::constants::{CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN};

    // let re = regex::Regex::new(r#"(\d+(?:px|vw|vh))"#).unwrap();
    // let result = re.replace_all($var.as_str(), |caps: &regex::Captures| {
    //     let value = &caps[1];
    //     let unit = &value[value.len() - 2..];
    //     let parsed_value: i32 = value[..value.len() - 2].parse().unwrap();
    //     if $platform == Platform::Harmony {
    //       if unit == "px" {
    //         // return format!("{}lpx", parsed_value);
    //         return format!("${{{}({}, 'px')}}", CONVERT_STYLE_PX_FN, parsed_value);
    //       } else {
    //         return format!("${{{}({}, '{}')}}", CONVERT_STYLE_PX_FN, parsed_value, unit);
    //       }
    //     } else {
    //       if unit == "px" {
    //         return format!("${{{}({}, 'px')}}", RN_CONVERT_STYLE_PX_FN, parsed_value);
    //       } else {
    //         return format!("${{{}({}, '{}')}}", RN_CONVERT_STYLE_VU_FN, parsed_value, unit);
    //       }
    //     }
    // });

    // Expr::Tpl(Tpl {
    //   span: DUMMY_SP,
    //   exprs: vec![],
    //   quasis: vec![
    //     TplElement {
    //       span: DUMMY_SP,
    //       tail: false,
    //       cooked: None,
    //       raw: Atom::from(result).into(),
    //     }
    //   ],
    // })
  }};
}

#[macro_export]
macro_rules! generate_expr_ident {
  ($var:expr) => {{
    use swc_core::common::DUMMY_SP;
    use swc_core::ecma::ast::*;
    Expr::Ident(Ident::new($var.into(), DUMMY_SP))
  }};
}

#[macro_export]
macro_rules! generate_expr_by_length {
  ($var:expr, $platform:expr) => {{
    use lightningcss::traits::ToCss;
    use lightningcss::values::length::Length;
    use $crate::generate_expr_lit_calc;
    use $crate::style_propetries::unit::{generate_expr_by_length_value, Platform};
    match $var {
      Length::Value(val) => generate_expr_by_length_value(&val, $platform),
      Length::Calc(val) => {
        let calc_string = val
          .to_css_string(lightningcss::stylesheet::PrinterOptions::default())
          .unwrap();
        generate_expr_lit_calc!(calc_string, $platform)
      }
    }
  }};
}

#[macro_export]
macro_rules! generate_expr_by_length_percentage_or_auto {
  ($var:expr, $platform:expr) => {{
    use $crate::{generate_expr_by_length_percentage, generate_expr_lit_str};
    match $var {
      LengthPercentageOrAuto::LengthPercentage(length_percent) => {
        generate_expr_by_length_percentage!(length_percent, $platform)
      }
      LengthPercentageOrAuto::Auto => generate_expr_lit_str!("auto"),
    }
  }};
}

#[macro_export]
macro_rules! generate_expr_by_length_percentage {
  ($var:expr, $platform:expr) => {{
    use lightningcss::traits::ToCss;
    use $crate::{
      generate_expr_lit_calc, generate_expr_lit_str,
      style_propetries::unit::generate_expr_by_length_value,
    };
    use rust_decimal::prelude::{Decimal, FromPrimitive};

    match $var {
      lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => {
        generate_expr_by_length_value(&dimension, $platform)
      }
      lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => {
        let decimal = Decimal::from_f32(percentage.0 * 100.0);
        let mut str_value = String::new();
        if let Some(d) = decimal {
          str_value = d.to_string();
        } else {
          str_value = (percentage.0 * 100.0).round().to_string();
        }
        generate_expr_lit_str!(str_value + "%")
      }
      lightningcss::values::percentage::DimensionPercentage::Calc(calc) => {
        let calc_string = calc
          .to_css_string(lightningcss::stylesheet::PrinterOptions::default())
          .unwrap();
        generate_expr_lit_calc!(calc_string, $platform)
      }
    }
  }};
}

#[macro_export]
macro_rules! generate_invalid_expr {
  () => {{
    use swc_core::common::DUMMY_SP;
    use swc_core::ecma::ast::*;

    Expr::Invalid(Invalid { span: DUMMY_SP })
  }};
}

// 依赖 use lightningcss::traits::ToCss;
#[macro_export]
macro_rules! generate_dimension_percentage {
  ($class:ident, $val:ident) => {
    match $val {
      lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => {
        $class::LengthValue(dimension.clone())
      }
      lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => {
        $class::Percentage(percentage.clone())
      }
      lightningcss::values::percentage::DimensionPercentage::Calc(calc) => $class::String(
        calc
          .to_css_string(lightningcss::stylesheet::PrinterOptions::default())
          .unwrap(),
      ),
    }
  };
}

#[macro_export]
macro_rules! generate_expr_based_on_platform {
  ($platform:expr, $value:expr) => {
    match $platform {
      Platform::Harmony => $value.to_expr().into(),
      _ => $value.to_expr().into(),
    }
  };
}

// 生成property_name的value类型为 color的属性
// 依赖 use swc_ecma_ast
#[macro_export]
macro_rules! generate_color_property {
  ($class:ident, $( ($property_enum:path, $property_value:ident) ),* ) => {
    use super::{traits::ToExpr, unit::PropertyTuple};
    use $crate::{generate_expr_lit_color, generate_invalid_expr};
    use lightningcss::values::color::CssColor;

    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: Option<(super::style_property_type::CSSPropertyType, CssColor)>
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        if let Some(color) = &self.value {
          PropertyTuple::One(
            color.0,
            generate_expr_lit_color!(&color.1)
          )
        } else {
          PropertyTuple::One(
            super::style_property_type::CSSPropertyType::Invalid,
            generate_invalid_expr!()
          )
        }
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        $class {
          id: prop.0,
          value: match prop.1 {
            $(
              lightningcss::properties::Property::$property_value(val) => {
                Some((
                  $property_enum,
                  val.clone()
                ))
              }
            )*
            _ => None
          }
        }
      }
    }
  }
}

// 生成property_name的value类型为 CSSNumber的属性
// 依赖：use swc_ecma_ast; use lightningcss
#[macro_export]
macro_rules! generate_number_property {
  ($class:ident, $( ($property_enum:path, $property_name:ident) ),* ) => {

    use swc_core::ecma::ast::*;
    use swc_core::common::DUMMY_SP;
    use super::{traits::ToExpr, unit::PropertyTuple};


    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: (super::style_property_type::CSSPropertyType, lightningcss::values::number::CSSNumber)
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.value.0,
          Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: self.value.1 as f64,
            raw: None,
          }))
          .into()
        )
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        match prop.1 {
          $(
            lightningcss::properties::Property::$property_name(value, _) => {
              $class {
                id: prop.0,
                value: (
                  $property_enum,
                  *value
                )
              }
            }
          )*
          _ => $class {
            id: prop.0,
            value: (
              CSSPropertyType::Invalid,
              0.0
            )
          }
        }
      }
    }

    impl $class {
      pub fn from_value (prop: (String, (CSSPropertyType, f32))) -> Self {
        $class {
          id: prop.0,
          value: prop.1
        }
      }
    }
  };
}

// 生成property_name的value类型为 LengthValue的属性
// 依赖：use swc_ecma_ast; use lightningcss
#[macro_export]
macro_rules! generate_length_value_property {
  ($class:ident, $( ($property_enum:path, $property_value:ident) ),* ) => {


    use lightningcss::traits::ToCss;
    use super::unit::PropertyTuple;
    use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};
    use $crate::{generate_dimension_percentage, generate_expr_lit_calc, generate_expr_lit_str, generate_invalid_expr};
    use rust_decimal::prelude::{Decimal, FromPrimitive};

    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: (super::style_property_type::CSSPropertyType, EnumValue)
    }

    #[derive(Debug, Clone)]
    pub enum EnumValue {
      LengthValue(lightningcss::values::length::LengthValue),
      Percentage(lightningcss::values::percentage::Percentage),
      String(String),
      Auto
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.value.0,
          match &self.value.1 {
            EnumValue::String(value) => generate_expr_lit_calc!(value, Platform::Harmony),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            EnumValue::Percentage(value) => {
              let decimal = Decimal::from_f32(value.0 * 100.0);
              let mut str_value = String::new();
              if let Some(d) = decimal {
                str_value = d.to_string();
              } else {
                str_value = (value.0 * 100.0).round().to_string();
              }
              generate_expr_lit_str!(str_value + "%")
            },
            EnumValue::Auto => generate_invalid_expr!()   // harmony 是个非法制，固不会生效
          }
        )
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        match prop.1 {
          $(
            lightningcss::properties::Property::$property_value(value) => {
               $class {
                id: prop.0,
                value: (
                  $property_enum,
                  match value {
                    lightningcss::values::length::LengthPercentageOrAuto::Auto => EnumValue::Auto,
                    lightningcss::values::length::LengthPercentageOrAuto::LengthPercentage(length_percentage) => {
                      generate_dimension_percentage!(EnumValue, length_percentage)
                    },
                  }
                )
              }
            }
          )*
          _ => $class {
            id: prop.0,
            value: (
              super::style_property_type::CSSPropertyType::Invalid,
              EnumValue::String("auto".to_string())
            )
          }
        }
      }
    }
  }
}

// 生成property_name的value类型为 Size的属性
// 依赖：use swc_ecma_ast; use lightningcss
#[macro_export]
macro_rules! generate_size_property {
  ($class:ident, $( ($property_enum:path, $property_value:ident) ),* ) => {

    use lightningcss::{
      traits::ToCss
    };

    use $crate::{generate_expr_lit_str, generate_expr_lit_calc};
    use super::unit::PropertyTuple;
    use super::{traits::ToExpr, unit::{generate_expr_by_length_value, Platform}};
    use rust_decimal::prelude::{Decimal, FromPrimitive};

    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: (super::style_property_type::CSSPropertyType, EnumValue)
    }

    #[derive(Debug, Clone)]
    pub enum EnumValue{
      LengthValue(lightningcss::values::length::LengthValue),
      Percentage(lightningcss::values::percentage::Percentage),
      String(String),
      Auto
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.value.0.clone(),
          match &self.value.1 {
            EnumValue::String(value) => generate_expr_lit_calc!(value, Platform::Harmony),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            EnumValue::Percentage(value) => {
              let decimal = Decimal::from_f32(value.0 * 100.0);
              let mut str_value = String::new();
              if let Some(d) = decimal {
                str_value = d.to_string();
              } else {
                str_value = (value.0 * 100.0).round().to_string();
              }
              generate_expr_lit_str!(str_value + "%")
            },
            EnumValue::Auto => generate_expr_lit_str!("auto")   // harmony 是个非法制，固不会生效
          }
        )
      }

    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        match prop.1 {
          $(
            lightningcss::properties::Property::$property_value(value) => {
              $class {
                id: prop.0,
                value: (
                  $property_enum,
                  match value {
                    LengthPercentage(length_percentage) => {
                        match length_percentage {
                          lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => EnumValue::LengthValue(dimension.clone()),
                          lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => EnumValue::Percentage(percentage.clone()),
                          lightningcss::values::percentage::DimensionPercentage::Calc(calc) => EnumValue::String(calc.to_css_string(lightningcss::stylesheet::PrinterOptions::default()).unwrap())
                        }
                    },
                    _ => EnumValue::Auto
                  }
                )
              }
            }
          )*
          _ =>  $class {
            id: prop.0,
            value: (
              super::style_property_type::CSSPropertyType::Invalid,
              EnumValue::String("auto".to_string())
            )
          }
        }
      }
    }

  };
}

// 生成字符串模版
#[macro_export]
macro_rules! generate_tpl_expr {
  ($items: expr) => {{
    use swc_core::ecma::ast::*;
    use swc_core::{atoms::Atom, common::DUMMY_SP};

    let mut quasis = vec![TplElement {
      span: DUMMY_SP,
      tail: false,
      cooked: None,
      raw: Atom::from("").into(),
    }];
    let mut exprs = vec![];
    $items.iter().for_each(|value| match value {
      Expr::Lit(lit) => {
        match lit {
          Lit::Str(str_lit) => {
            let mut quasi = quasis.pop().unwrap();
            quasi.raw = Atom::from(format!(
              " {} {} ",
              quasi.raw.to_string(),
              str_lit.value.as_ref()
            ));
            quasis.push(quasi);
          }
          Lit::Num(num_lit) => {
            let mut quasi = quasis.pop().unwrap();
            quasi.raw = Atom::from(format!(
              " {} {} ",
              quasi.raw.to_string(),
              num_lit.value.to_string()
            ));
            quasis.push(quasi);
          }
          _ => {}
        };
      }
      _ => {
        exprs.push(Box::new(value.to_owned()));
        quasis.push(TplElement {
          span: DUMMY_SP,
          tail: false,
          cooked: None,
          raw: Atom::from("").into(),
        })
      }
    });

    // 删除无用空格
    for (i, quasi) in quasis.clone().into_iter().enumerate() {
      let cleaned_string: String = quasi
        .raw
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
      if i == 0 {
        quasis[i].raw = Atom::from(format!("{} ", cleaned_string).trim_start()).into();
      } else if i == quasis.len() - 1 {
        quasis[i].raw = Atom::from(format!(" {}", cleaned_string).trim_end()).into();
        quasis[i].tail = true;
      } else {
        quasis[i].raw = Atom::from(format!(" {} ", cleaned_string.trim())).into();
      }
    }
    if quasis.len() == 1 {
      quasis[0].raw = Atom::from(format!("{} ", quasis[0].raw).trim()).into();
    }

    Expr::Tpl(Tpl {
      span: DUMMY_SP,
      exprs,
      quasis: quasis,
    })
  }};
}
