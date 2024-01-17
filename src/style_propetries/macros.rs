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
    swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident::new($var.into(), DUMMY_SP))
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

// 生成property_name的value类型为 CSSNumber的属性
#[macro_export]
macro_rules! generate_number_property {
  ($class:ident, $( $property_name:ident ), *) => {
    #[derive(Debug, Clone)]
    pub struct $class(pub CSSNumber);

    impl ToExpr for $class {
      fn to_expr(&self) -> Expr {
        Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: self.0 as f64,
          raw: None,
        }))
        .into()
      }
      fn to_rn_expr(&self) -> Expr {
        Expr::Lit(Lit::Num(Number {
          span: DUMMY_SP,
          value: self.0 as f64,
          raw: None,
        }))
        .into()
      }
    }

    impl From<&Property<'_>> for $class {
      fn from(value: &Property<'_>) -> Self {
        let mut result = $class(0.0);
        match value {
          $(
            Property::$property_name(value, _) => {
              result = $class(*value);
            }
          )*
          _ => {}
        }
        result
      }
    }
  };
}

// 生成property_name的value类型为 LengthValue的属性
#[macro_export]
macro_rules! generate_length_value_property {
  ($class:ident, $( $property_name:ident ), *) => {
      #[derive(Debug, Clone)]
      pub enum $class {
        LengthValue(LengthValue),
        Percentage(Percentage),
        String(String),
        Auto
      }

      impl ToExpr for $class {
        fn to_expr(&self) -> Expr {
          match &self {
            $class::String(value) => generate_expr_lit_str!(value.to_owned()),
            $class::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            $class::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            $class::Auto => generate_expr_lit_str!("auto")   // harmony 是个非法制，固不会生效
          }
        }

        fn to_rn_expr(&self) -> Expr {
          match &self {
            $class::String(value) => generate_expr_lit_str!(value.to_owned()),
            $class::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
            $class::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            $class::Auto => generate_expr_lit_str!("auto")
          }
        }
      }

      impl From<&Property<'_>> for $class {
        fn from(value: &Property<'_>) -> Self {
          let mut length_property_value = $class::String("auto".to_string());
          match value {
            $(
              Property::$property_name(prop) => {
                length_property_value = match prop {
                    LengthPercentageOrAuto::Auto => $class::Auto,
                    LengthPercentageOrAuto::LengthPercentage(length_percentage) => {
                        match length_percentage {
                            DimensionPercentage::Dimension(dimension) => $class::LengthValue(dimension.clone()),
                            DimensionPercentage::Percentage(percentage) => $class::Percentage(percentage.clone()),
                            DimensionPercentage::Calc(calc) => $class::String(calc.to_css_string(PrinterOptions::default()).unwrap())
                        }
                    },
                };
              }
            )*
            _ => {}
          }
          length_property_value
        }
      }

    };
}

// 生成property_name的value类型为 Size的属性
#[macro_export]
macro_rules! generate_size_property {
  ($class:ident, $( $property_name:ident ), *) => {
      #[derive(Debug, Clone)]
      pub enum $class {
        LengthValue(LengthValue),
        Percentage(Percentage),
        String(String),
        Auto
      }

      impl ToExpr for $class {
        fn to_expr(&self) -> Expr {
          match &self {
            $class::String(value) => generate_expr_lit_str!(value.to_owned()),
            $class::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::Harmony),
            $class::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            $class::Auto => generate_expr_lit_str!("auto")   // harmony 是个非法制，固不会生效
          }
        }

        fn to_rn_expr(&self) -> Expr {
          match &self {
            $class::String(value) => generate_expr_lit_str!(value.to_owned()),
            $class::LengthValue(length_value) => generate_expr_by_length_value(length_value, Platform::ReactNative),
            $class::Percentage(value) => generate_expr_lit_str!((value.0 * 100.0).to_string() + "%"),
            $class::Auto => generate_expr_lit_str!("auto")
          }
        }
      }

      impl From<&Property<'_>> for $class {
        fn from(value: &Property<'_>) -> Self {
          let mut size = $class::String("auto".to_string());
          match value {
            $(
              Property::$property_name(prop) => {
                size = match prop {
                    LengthPercentage(length_percentage) => {
                        match length_percentage {
                            DimensionPercentage::Dimension(dimension) => $class::LengthValue(dimension.clone()),
                            DimensionPercentage::Percentage(percentage) => $class::Percentage(percentage.clone()),
                            DimensionPercentage::Calc(calc) => $class::String(calc.to_css_string(PrinterOptions::default()).unwrap())
                        }
                    },
                    _ => $class::Auto
                };
              }
            )*
            _ => {}
          }
          size
        }
      }

    };
}