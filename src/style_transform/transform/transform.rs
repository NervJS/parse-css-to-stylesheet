use lightningcss::{
  properties::{transform::Transform as LNTransform, Property},
  stylesheet::PrinterOptions,
  traits::ToCss,
  values::{
    length::{Length, LengthValue},
    percentage::{DimensionPercentage, NumberOrPercentage},
    position::{
      HorizontalPositionKeyword, Position,
      PositionComponent::{self, Center, Side},
      VerticalPositionKeyword,
    },
  },
};

use crate::style_transform::utils::{StringNumber, WrapCSSNumber};

use super::{
  matrix::Matrix, rotate::Rotate, scale::Scale, translate::Translate, Matrices, Rotates, Scales,
  Translates,
};

fn parse_dimension_percentage(value: &DimensionPercentage<LengthValue>) -> Option<StringNumber> {
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
pub struct Transform {
  pub translate: Translates,
  pub rotate: Rotates,
  pub scale: Scales,
  pub matrix: Matrices,
}

impl Transform {
  pub fn new() -> Self {
    Self {
      translate: Translates::new(),
      rotate: Rotates::new(),
      scale: Scales::new(),
      matrix: Matrices::new(),
    }
  }
}

impl From<(&Property<'_>, Option<&Position>)> for Transform {
  fn from(value: (&Property<'_>, Option<&Position>)) -> Self {
    let (value, transform_origin) = value;
    let mut transform = Transform::new();
    let mut translates = vec![];
    let mut rotates = vec![];
    let mut scales = vec![];
    let mut matrixs = vec![];
    if let Property::Transform(value, _) = value {
      for item in value.0.iter() {
        let mut center_x = None;
        let mut center_y = None;
        match transform_origin {
          Some(position) => {
            match &position.x {
              Center => {
                center_x = Some(StringNumber::String("50%".to_string()));
              }
              PositionComponent::Length(length) => {
                center_x = parse_dimension_percentage(&length);
              }
              Side { side, .. } => match &side {
                HorizontalPositionKeyword::Left => {
                  center_x = Some(StringNumber::String("0%".to_string()));
                }
                HorizontalPositionKeyword::Right => {
                  center_x = Some(StringNumber::String("100%".to_string()));
                }
              },
            }
            match &position.y {
              Center => {
                center_y = Some(StringNumber::String("50%".to_string()));
              }
              PositionComponent::Length(length) => {
                center_y = parse_dimension_percentage(&length);
              }
              Side { side, .. } => match &side {
                VerticalPositionKeyword::Top => {
                  center_y = Some(StringNumber::String("0%".to_string()));
                }
                VerticalPositionKeyword::Bottom => {
                  center_y = Some(StringNumber::String("100%".to_string()));
                }
              },
            }
          }
          None => {}
        }
        match item {
          LNTransform::Translate(x, y) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            translate.y = parse_dimension_percentage(y);
            translates.push(translate);
          }
          LNTransform::TranslateX(x) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            translates.push(translate);
          }
          LNTransform::TranslateY(y) => {
            let mut translate = Translate::new();
            translate.y = parse_dimension_percentage(y);
            translates.push(translate);
          }
          LNTransform::TranslateZ(z) => {
            let mut translate = Translate::new();
            translate.z = parse_length(z);
            translates.push(translate);
          }
          LNTransform::Translate3d(x, y, z) => {
            let mut translate = Translate::new();
            translate.x = parse_dimension_percentage(x);
            translate.y = parse_dimension_percentage(y);
            translate.z = parse_length(z);
            translates.push(translate);
          }
          LNTransform::Rotate(angle) | LNTransform::RotateZ(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(0.0));
            rotate.y = Some(WrapCSSNumber(0.0));
            rotate.z = Some(WrapCSSNumber(1.0));
            rotate.angle =
              StringNumber::String(angle.to_css_string(PrinterOptions::default()).unwrap());
            rotate.center_x = center_x.clone();
            rotate.center_y = center_y.clone();
            rotates.push(rotate);
          }
          LNTransform::RotateX(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(1.0));
            rotate.y = Some(WrapCSSNumber(0.0));
            rotate.z = Some(WrapCSSNumber(0.0));
            rotate.angle =
              StringNumber::String(angle.to_css_string(PrinterOptions::default()).unwrap());
            rotate.center_x = center_x.clone();
            rotate.center_y = center_y.clone();
            rotates.push(rotate);
          }
          LNTransform::RotateY(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(0.0));
            rotate.y = Some(WrapCSSNumber(1.0));
            rotate.z = Some(WrapCSSNumber(0.0));
            rotate.angle =
              StringNumber::String(angle.to_css_string(PrinterOptions::default()).unwrap());
            rotate.center_x = center_x.clone();
            rotate.center_y = center_y.clone();
            rotates.push(rotate);
          }
          LNTransform::Rotate3d(x, y, z, angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(WrapCSSNumber(*x));
            rotate.y = Some(WrapCSSNumber(*y));
            rotate.z = Some(WrapCSSNumber(*z));
            rotate.angle =
              StringNumber::String(angle.to_css_string(PrinterOptions::default()).unwrap());
            rotate.center_x = center_x.clone();
            rotate.center_y = center_y.clone();
            rotates.push(rotate);
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
            scale.center_x = center_x.clone();
            scale.center_y = center_y.clone();
            scales.push(scale);
          }
          LNTransform::ScaleX(x) => {
            let mut scale = Scale::new();
            match x {
              NumberOrPercentage::Number(x) => {
                scale.x = Some(WrapCSSNumber(*x));
              }
              _ => {}
            }
            scale.center_x = center_x.clone();
            scale.center_y = center_y.clone();
            scales.push(scale);
          }
          LNTransform::ScaleY(y) => {
            let mut scale = Scale::new();
            match y {
              NumberOrPercentage::Number(y) => {
                scale.y = Some(WrapCSSNumber(*y));
              }
              _ => {}
            }
            scale.center_x = center_x.clone();
            scale.center_y = center_y.clone();
            scales.push(scale);
          }
          LNTransform::ScaleZ(z) => {
            let mut scale = Scale::new();
            match z {
              NumberOrPercentage::Number(z) => {
                scale.z = Some(WrapCSSNumber(*z));
              }
              _ => {}
            }
            scale.center_x = center_x.clone();
            scale.center_y = center_y.clone();
            scales.push(scale);
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
            scale.center_x = center_x.clone();
            scale.center_y = center_y.clone();
            scales.push(scale);
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
            matrixs.push(matrix);
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
            matrixs.push(matrix);
          }
          _ => {}
        }
      }
    }
    transform.translate = Translates(translates);
    transform.rotate = Rotates(rotates);
    transform.scale = Scales(scales);
    transform.matrix = Matrices(matrixs);
    transform
  }
}
