#[macro_export]
macro_rules! generate_prop_name {
  ($key: expr) => {
    swc_ecma_ast::PropName::Ident(swc_ecma_ast::Ident::new($key.into(), swc_common::DUMMY_SP))
  };
}

#[macro_export]
macro_rules! generate_expr_lit_str {
  ($var:expr) => {
    swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str($var.into()))
  };
}

#[macro_export]
macro_rules! generate_expr_lit_num {
  ($var:expr) => {
    swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Num(swc_ecma_ast::Number::from($var)))
  };
}

#[macro_export]
macro_rules! generate_expr_ident {
  ($var:expr) => {
    swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident::new($var.into(), swc_common::DUMMY_SP))
  };
}

#[macro_export]
macro_rules! generate_string_by_css_color {
  ($color:expr) => {{
    use $crate::style_propetries::unit::convert_color_keywords_to_hex;
    convert_color_keywords_to_hex($color.to_css_string(lightningcss::stylesheet::PrinterOptions {
      minify: false,
      targets: lightningcss::targets::Targets {
        include: lightningcss::targets::Features::HexAlphaColors,
        ..lightningcss::targets::Targets::default()
      },
      ..lightningcss::stylesheet::PrinterOptions::default()
    }).unwrap()).into()
  }};
}

#[macro_export]
macro_rules! generate_expr_by_length  {
  ($var:expr, $platform:expr) => {{
    use $crate::style_propetries::unit::{Platform, generate_expr_by_length_value};
    use lightningcss::values::length::Length;
    match $var {
      Length::Value(val) => generate_expr_by_length_value(&val, $platform),
      Length::Calc(val) => generate_expr_lit_str!(*val.to_css_string(lightningcss::stylesheet::PrinterOptions::default()).unwrap()),
    }
  }};
}

#[macro_export]
macro_rules! generate_expr_by_length_percentage_or_auto {
  ($var:expr, $platform:expr) => {{
    
    match $var {
      LengthPercentageOrAuto::LengthPercentage(length_percent) => {
        generate_expr_by_length_percentage!(length_percent, $platform)
      },
      LengthPercentageOrAuto::Auto => generate_invalid_expr!(),
    }
  }};
}


#[macro_export]
macro_rules! generate_expr_by_length_percentage {
  ($var:expr, $platform:expr) => {{
    use $crate::style_propetries::unit::{generate_expr_by_length_value};
    use lightningcss::traits::ToCss;
    
    match $var {
      lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => generate_expr_by_length_value(&dimension, $platform),
      lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => generate_expr_lit_str!((percentage.0 * 100.0).to_string() + "%"),
      lightningcss::values::percentage::DimensionPercentage::Calc(calc) => generate_expr_lit_str!(calc.to_css_string(lightningcss::stylesheet::PrinterOptions::default()).unwrap()),
    }
  }};
}

#[macro_export]
macro_rules! generate_invalid_expr {
  () => {
    swc_ecma_ast::Expr::Invalid(swc_ecma_ast::Invalid { span: swc_common::DUMMY_SP })
  };
}


// 依赖 use lightningcss::traits::ToCss;
#[macro_export]
macro_rules! generate_dimension_percentage {
  ($class:ident, $val:ident) => {
    match $val {
      lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => $class::LengthValue(dimension.clone()),
      lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => $class::Percentage(percentage.clone()),
      lightningcss::values::percentage::DimensionPercentage::Calc(calc) => $class::String(calc.to_css_string(lightningcss::stylesheet::PrinterOptions::default()).unwrap())
    }
  };
}

#[macro_export]
macro_rules! generate_expr_based_on_platform {
  ($platform:expr, $value:expr) => {
      match $platform {
          Platform::ReactNative => $value.to_rn_expr().into(),
          _ => $value.to_expr().into(),
      }
  };
}

// 生成property_name的value类型为 color的属性
// 依赖 use swc_ecma_ast
#[macro_export]
macro_rules! generate_color_property {
  ($class:ident, $( $property_name:ident ), *) => {
    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: String
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(self.value.clone().into())).into()
        )
      }
      fn to_rn_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Str(self.value.clone().into())).into()
        )
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        $class {
          id: prop.0,
          value: match prop.1 {
            $(
              lightningcss::properties::Property::$property_name(_) => {
                use $crate::style_propetries::unit::convert_color_keywords_to_hex;
                convert_color_keywords_to_hex(prop.1.value_to_css_string(lightningcss::stylesheet::PrinterOptions {
                  minify: false,
                  targets: lightningcss::targets::Targets {
                    include: lightningcss::targets::Features::HexAlphaColors,
                    ..lightningcss::targets::Targets::default()
                  },
                  ..lightningcss::stylesheet::PrinterOptions::default()
                }).unwrap())
              }
            )*
            _ => "".to_string()
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
  ($class:ident, $( $property_name:ident ), *) => {
    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: lightningcss::values::number::CSSNumber
    }

    impl ToExpr for $class {
      fn to_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Num(Number {
            span: DUMMY_SP,
            value: self.value as f64,
            raw: None,
          }))
          .into()
        )
      }
      fn to_rn_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          swc_ecma_ast::Expr::Lit(swc_ecma_ast::Lit::Num(Number {
            span: DUMMY_SP,
            value: self.value as f64,
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
              $class { id: 
                prop.0, 
                value: *value
              }
            }
          )*
          _ => $class {
            id: prop.0,
            value: 0.0
          }
        }
      }
    }
    
    impl $class {
      pub fn from_value (prop: (String, f32)) -> Self {
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
  ($class:ident, $( $property_name:ident ), *) => {
    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: EnumValue
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
          self.id.clone(),
          match &self.value {
            EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            EnumValue::Auto => generate_invalid_expr!()   // harmony 是个非法制，固不会生效
          }
        )
      }

      fn to_rn_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          match &self.value {
            EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
            EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            EnumValue::Auto => generate_expr_lit_str!("auto")
          }
        )
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        match prop.1 {
          $(
            lightningcss::properties::Property::$property_name(value) => {
               $class {
                id: prop.0,
                value: match value {
                  lightningcss::values::length::LengthPercentageOrAuto::Auto => EnumValue::Auto,
                  lightningcss::values::length::LengthPercentageOrAuto::LengthPercentage(length_percentage) => {
                    generate_dimension_percentage!(EnumValue, length_percentage)
                  },
                }
              }
            }
          )*
          _ => $class {
            id: prop.0,
            value: EnumValue::String("auto".to_string())
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
  ($class:ident, $( $property_name:ident ), *) => {

    #[derive(Debug, Clone)]
    pub struct $class {
      pub id: String,
      pub value: EnumValue
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
          self.id.clone(),
          match &self.value {
            EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            EnumValue::Auto => generate_expr_lit_str!("auto")   // harmony 是个非法制，固不会生效
          }
        )
      }

      fn to_rn_expr(&self) -> PropertyTuple {
        PropertyTuple::One(
          self.id.clone(),
          match &self.value {
            EnumValue::String(value) => generate_expr_lit_str!(value.to_owned()),
            EnumValue::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
            EnumValue::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            EnumValue::Auto => generate_expr_lit_str!("auto")
          }
        )
      }
    }

    impl From<(String, &lightningcss::properties::Property<'_>)> for $class {
      fn from(prop: (String, &lightningcss::properties::Property<'_>)) -> Self {
        match prop.1 {
          $(
            lightningcss::properties::Property::$property_name(value) => {
              $class {
                id: prop.0,
                value: match value {
                  LengthPercentage(length_percentage) => {
                      match length_percentage {
                        lightningcss::values::percentage::DimensionPercentage::Dimension(dimension) => EnumValue::LengthValue(dimension.clone()),
                        lightningcss::values::percentage::DimensionPercentage::Percentage(percentage) => EnumValue::Percentage(percentage.clone()),
                        lightningcss::values::percentage::DimensionPercentage::Calc(calc) => EnumValue::String(calc.to_css_string(lightningcss::stylesheet::PrinterOptions::default()).unwrap())
                      }
                  },
                  _ => EnumValue::Auto
                }
              }
            }
          )*
          _ =>  $class {
            id: prop.0,
            value: EnumValue::String("auto".to_string())
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

    let mut quasis = vec![
      swc_ecma_ast::TplElement {
        span: swc_common::DUMMY_SP,
        tail: false,
        cooked: None,
        raw: swc_atoms::Atom::from("").into(),
      },
    ];
    let mut exprs = vec![];
    $items.iter().for_each(|value| {
      match value {
        swc_ecma_ast::Expr::Lit(lit) => {
          match lit {
            swc_ecma_ast::Lit::Str(str_lit) => {
              let mut quasi = quasis.pop().unwrap();
              quasi.raw = swc_atoms::Atom::from(format!(" {} {} ",quasi.raw.to_string(), str_lit.value.as_ref()));
              quasis.push(quasi);
            },
            swc_ecma_ast::Lit::Num(num_lit) => {
              let mut quasi = quasis.pop().unwrap();
              quasi.raw = swc_atoms::Atom::from(format!(" {} {} ",quasi.raw.to_string(), num_lit.value.to_string()));
              quasis.push(quasi);
            },
            _ => {}
          };
        },
        _ => {
          exprs.push(Box::new(value.to_owned()));
          quasis.push(
            swc_ecma_ast::TplElement {
              span: swc_common::DUMMY_SP,
              tail: false,
              cooked: None,
              raw: swc_atoms::Atom::from("").into(),
            },
          )
        }
      }
    });

    // 删除无用空格
    for (i, quasi) in quasis.clone().into_iter().enumerate() {
      let cleaned_string: String = quasi.raw
      .split_whitespace()
      .collect::<Vec<&str>>()
      .join(" ");
      if i == 0 {
        quasis[i].raw = swc_atoms::Atom::from(format!("{} ", cleaned_string).trim_start()).into();
      } else if i == quasis.len() - 1 {
        quasis[i].raw = swc_atoms::Atom::from(format!(" {}", cleaned_string).trim_end()).into();
        quasis[i].tail = true;
      } else {
        quasis[i].raw = swc_atoms::Atom::from(format!(" {} ", cleaned_string.trim())).into();
      }
    }
    if quasis.len() == 1 {
      quasis[0].raw = swc_atoms::Atom::from(format!("{} ", quasis[0].raw).trim()).into();
    }

    swc_ecma_ast::Expr::Tpl(swc_ecma_ast::Tpl {
      span: swc_common::DUMMY_SP,
      exprs,
      quasis: quasis,
    })
  }};
}