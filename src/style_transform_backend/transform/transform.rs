use lightningcss::{
  properties::{transform::Transform as LNTransform, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{
    length::{Length, LengthValue},
    percentage::{DimensionPercentage, NumberOrPercentage},
  },
};
use swc_ecma_ast::{Expr, ArrayLit, ExprOrSpread};

use crate::style_transform::{utils::{StringNumber, WrapCSSNumber}, traits::ToExpr};

use super::{
  rotate::Rotate, scale::Scale, translate::Translate, matrix::Matrix,
};

pub fn parse_dimension_percentage(value: &DimensionPercentage<LengthValue>) -> Option<StringNumber> {
  value
      .to_css_string(PrinterOptions::default())
      .ok()
      .map(StringNumber::String)
  // match value {
  //   DimensionPercentage::Dimension(value) => {
  //     Some(StringNumber::Number(value.to_unit_value().0))
  //   },
  //   _ => value
  //     .to_css_string(PrinterOptions::default())
  //     .ok()
  //     .map(StringNumber::String),
  // }
}

fn parse_length(value: &Length) -> Option<StringNumber> {
  match value {
    Length::Value(value) => Some(StringNumber::Number(value.to_unit_value().0)),
    _ => value
      .to_css_string(PrinterOptions::default())
      .ok()
      .map(StringNumber::String),
  }
}

#[derive(Debug, Clone)]
pub enum Matrix4 {
  Translates(Translate),
  Rotates(Rotate),
  Scales(Scale),
  Matrix(Matrix),
}

#[derive(Debug, Clone)]
pub struct Transform(pub Vec<Matrix4>);

impl ToExpr for Transform {
  fn to_expr(&self) -> Expr {
    let mut expr = vec![];
    for item in self.0.iter() {
      match item {
        Matrix4::Translates(value) => {
          expr.push(value.to_expr());
        }
        Matrix4::Rotates(value) => {
          expr.push(value.to_expr());
        }
        Matrix4::Scales(value) => {
          expr.push(value.to_expr());
        }
        Matrix4::Matrix(value) => {
          expr.push(value.to_expr());
        }
      }
    };

    Expr::Array(ArrayLit {
      span: Default::default(),
      elems: expr.into_iter().map(Some).map(
        |item| {
          Some(ExprOrSpread {
            spread: None,
            expr: Box::new(item.unwrap()),
          })
        }
      ).collect::<Vec<_>>(),
    })
  }
}

impl From<&Property<'_>> for Transform {
  fn from(value: &Property<'_>) -> Self {
    let mut transform = vec![];
    if let Property::Transform(value, _) = value {
      for item in value.0.iter() {
        match item {
          LNTransform::Translate(x, y) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            translate.y = parse_dimension_percentage(y);
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateX(x) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateY(y) => {
            let mut translate = Translate::new();
            translate.y = parse_dimension_percentage(y);
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateZ(z) => {
            let mut translate = Translate::new();
            translate.z = parse_length(z);
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::Translate3d(x, y, z) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            translate.y = parse_dimension_percentage(y);
            translate.z = parse_length(z);
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::Rotate(angle) | LNTransform::RotateZ(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(0.0));
            rotate.y = Some(WrapCSSNumber(0.0));
            rotate.z = Some(WrapCSSNumber(1.0));
            match extract_degrees(angle.to_css_string(PrinterOptions::default()).unwrap().as_str()) {
              Some(angle) => {
                rotate.angle = StringNumber::Number(angle);
              }
              None => {}
            }
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::RotateX(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(1.0));
            rotate.y = Some(WrapCSSNumber(0.0));
            rotate.z = Some(WrapCSSNumber(0.0));
            match extract_degrees(angle.to_css_string(PrinterOptions::default()).unwrap().as_str()) {
              Some(angle) => {
                rotate.angle = StringNumber::Number(angle);
              }
              None => {}
            }
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::RotateY(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(0.0));
            rotate.y = Some(WrapCSSNumber(1.0));
            rotate.z = Some(WrapCSSNumber(0.0));
            match extract_degrees(angle.to_css_string(PrinterOptions::default()).unwrap().as_str()) {
              Some(angle) => {
                rotate.angle = StringNumber::Number(angle);
              }
              None => {}
            }
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::Rotate3d(x, y, z, angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(*x));
            rotate.y = Some(WrapCSSNumber(*y));
            rotate.z = Some(WrapCSSNumber(*z));
            match extract_degrees(angle.to_css_string(PrinterOptions::default()).unwrap().as_str()) {
              Some(angle) => {
                rotate.angle = StringNumber::Number(angle);
              }
              None => {}
            }
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::Scale(x, y) => {
            let mut scale = Scale::new();
            match x {
              NumberOrPercentage::Number(x) => {
                scale.x = Some(WrapCSSNumber(*x));
              }
              _ => {}
            }
            match y {
              NumberOrPercentage::Number(y) => {
                scale.y = Some(WrapCSSNumber(*y));
              }
              _ => {}
            }
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleX(x) => {
            let mut scale = Scale::new();
            match x {
              NumberOrPercentage::Number(x) => {
                scale.x = Some(WrapCSSNumber(*x));
              }
              _ => {}
            }
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleY(y) => {
            let mut scale = Scale::new();
            match y {
              NumberOrPercentage::Number(y) => {
                scale.y = Some(WrapCSSNumber(*y));
              }
              _ => {}
            }
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleZ(z) => {
            let mut scale = Scale::new();
            match z {
              NumberOrPercentage::Number(z) => {
                scale.z = Some(WrapCSSNumber(*z));
              }
              _ => {}
            }
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::Scale3d(x, y, z) => {
            let mut scale = Scale::new();
            match x {
              NumberOrPercentage::Number(x) => {
                scale.x = Some(WrapCSSNumber(*x));
              }
              _ => {}
            }
            match y {
              NumberOrPercentage::Number(y) => {
                scale.y = Some(WrapCSSNumber(*y));
              }
              _ => {}
            }
            match z {
              NumberOrPercentage::Number(z) => {
                scale.z = Some(WrapCSSNumber(*z));
              }
              _ => {}
            }
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::Matrix(m) => {
            let mut matrix = Matrix::new();
            let matrix3d = m.to_matrix3d();
            matrix.m00 = WrapCSSNumber(matrix3d.m11);
            matrix.m01 = WrapCSSNumber(matrix3d.m12);
            matrix.m02 = WrapCSSNumber(matrix3d.m13);
            matrix.m03 = WrapCSSNumber(matrix3d.m14);
            matrix.m10 = WrapCSSNumber(matrix3d.m21);
            matrix.m11 = WrapCSSNumber(matrix3d.m22);
            matrix.m12 = WrapCSSNumber(matrix3d.m23);
            matrix.m13 = WrapCSSNumber(matrix3d.m24);
            matrix.m20 = WrapCSSNumber(matrix3d.m31);
            matrix.m21 = WrapCSSNumber(matrix3d.m32);
            matrix.m22 = WrapCSSNumber(matrix3d.m33);
            matrix.m23 = WrapCSSNumber(matrix3d.m34);
            matrix.m30 = WrapCSSNumber(matrix3d.m41);
            matrix.m31 = WrapCSSNumber(matrix3d.m42);
            matrix.m32 = WrapCSSNumber(matrix3d.m43);
            matrix.m33 = WrapCSSNumber(matrix3d.m44);
            transform.push(Matrix4::Matrix(matrix));
          }
          LNTransform::Matrix3d(m) => {
            let mut matrix = Matrix::new();
            matrix.m00 = WrapCSSNumber(m.m11);
            matrix.m01 = WrapCSSNumber(m.m12);
            matrix.m02 = WrapCSSNumber(m.m13);
            matrix.m03 = WrapCSSNumber(m.m14);
            matrix.m10 = WrapCSSNumber(m.m21);
            matrix.m11 = WrapCSSNumber(m.m22);
            matrix.m12 = WrapCSSNumber(m.m23);
            matrix.m13 = WrapCSSNumber(m.m24);
            matrix.m20 = WrapCSSNumber(m.m31);
            matrix.m21 = WrapCSSNumber(m.m32);
            matrix.m22 = WrapCSSNumber(m.m33);
            matrix.m23 = WrapCSSNumber(m.m34);
            matrix.m30 = WrapCSSNumber(m.m41);
            matrix.m31 = WrapCSSNumber(m.m42);
            matrix.m32 = WrapCSSNumber(m.m43);
            matrix.m33 = WrapCSSNumber(m.m44);
            transform.push(Matrix4::Matrix(matrix));
          }
          _ => {}
        }
      }
    }
    Transform(transform)
  }
}

fn extract_degrees(input: &str) -> Option<f32> {
  // 去掉字符串中的非数字字符
  let numeric_part: String = input.chars().filter(|c| c.is_digit(10) || *c == '.').collect();

  // 将提取到的字符串转换成f32
  match numeric_part.parse::<f32>() {
      Ok(value) => Some(value),
      Err(_) => None, // 转换失败时返回None
  }
}
