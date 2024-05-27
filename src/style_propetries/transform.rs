use std::vec;

use lightningcss::properties::{transform::Transform as LNTransform, Property};

use swc_core::ecma::ast::*;

use crate::style_propetries::traits::ToExpr;

use super::{style_property_type::CSSPropertyType, transform_properties::{matrix::Matrix, rotate::Rotate, scale::Scale, skew::Skew, translate::Translate}, unit::PropertyTuple};


#[derive(Debug, Clone)]
pub enum Matrix4 {
  Translates(Translate),
  Rotates(Rotate),
  Scales(Scale),
  Skew(Skew),
  Matrix(Matrix),
}

#[derive(Debug, Clone)]
pub struct Transform {
  pub id: String,
  pub value: Vec<Matrix4>
}

impl ToExpr for Transform {
    fn to_expr(&self) -> PropertyTuple {
      let mut props = vec![];
      self.value.iter().for_each(|item| {
        match item {
          Matrix4::Translates(value) => {
            props.push(value.to_expr());
          },
          Matrix4::Rotates(value) => {
            props.push(value.to_expr());
          }
          Matrix4::Scales(value) => {
            props.push(value.to_expr());
          }
          // Matrix4::Matrix(value) => {
          //   props.extend(value.to_expr());
          // },
          _ => {}
        }
      });
      PropertyTuple::One(
        CSSPropertyType::Transform,
        Expr::Object(ObjectLit {
          span: Default::default(),
          props:props,
        })
      )
    }
}


impl From<(String, &Property<'_>)> for Transform {
  fn from(prop: (String, &Property<'_>)) -> Self {
    let mut transform = vec![];
    if let Property::Transform(value, _) = prop.1 {
      for item in value.0.iter() {
        match item {
          LNTransform::Translate(x, y) => {
            let mut translate = Translate::new();
            translate.x = Some(x.clone());
            translate.y = Some(y.clone());
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateX(x) => {
            let mut translate = Translate::new();
            translate.x = Some(x.clone());
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateY(y) => {
            let mut translate = Translate::new();
            translate.y = Some(y.clone());
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::TranslateZ(z) => {
            let mut translate = Translate::new();
            translate.z = Some(z.clone());
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::Translate3d(x, y, z) => {
            let mut translate = Translate::new();
            translate.x = Some(x.clone());
            translate.y = Some(y.clone());
            translate.z = Some(z.clone());
            transform.push(Matrix4::Translates(translate));
          }
          LNTransform::Rotate(angle) | LNTransform::RotateZ(angle) => {
            let mut rotate = Rotate::new();
            rotate.rotate = Some(1.0);
            rotate.angle = angle.clone();
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::RotateX(angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(1.0);
            rotate.angle = angle.clone();
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::RotateY(angle) => {
            let mut rotate = Rotate::new();
            rotate.y = Some(1.0);
            rotate.angle = angle.clone();
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::Rotate3d(x, y, z, angle) => {
            let mut rotate = Rotate::new();
            rotate.x = Some(*x);
            rotate.y = Some(*y);
            rotate.z = Some(*z);
            rotate.angle = angle.clone();
            transform.push(Matrix4::Rotates(rotate));
          }
          LNTransform::Scale(x, y) => {
            let mut scale = Scale::new();
            scale.x = Some(x.clone());
            scale.y = Some(y.clone());
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleX(x) => {
            let mut scale = Scale::new();
            scale.x = Some(x.clone());
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleY(y) => {
            let mut scale = Scale::new();
            scale.x = Some(y.clone());
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::ScaleZ(z) => {
            let mut scale = Scale::new();
            scale.z = Some(z.clone());
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::Scale3d(x, y, z) => {
            let mut scale = Scale::new();
            scale.x = Some(x.clone());
            scale.y = Some(y.clone());
            scale.z = Some(z.clone());
            transform.push(Matrix4::Scales(scale));
          }
          LNTransform::SkewX(x) => {
            let mut skew = Skew::new();
            skew.x = Some(x.clone());
            transform.push(Matrix4::Skew(skew));
          }
          LNTransform::SkewY(y) => {
            let mut skew = Skew::new();
            skew.y = Some(y.clone());
            transform.push(Matrix4::Skew(skew));
          }
          LNTransform::Skew(x, y) => {
            let mut skew = Skew::new();
            skew.x = Some(x.clone());
            skew.y = Some(y.clone());
            transform.push(Matrix4::Skew(skew));
          }
          LNTransform::Matrix(m) => {
            let mut matrix = Matrix::new();
            let matrix3d = m.to_matrix3d();
            matrix.m00 = matrix3d.m11;
            matrix.m01 = matrix3d.m12;
            matrix.m02 = matrix3d.m13;
            matrix.m03 = matrix3d.m14;
            matrix.m10 = matrix3d.m21;
            matrix.m11 = matrix3d.m22;
            matrix.m12 = matrix3d.m23;
            matrix.m13 = matrix3d.m24;
            matrix.m20 = matrix3d.m31;
            matrix.m21 = matrix3d.m32;
            matrix.m22 = matrix3d.m33;
            matrix.m23 = matrix3d.m34;
            matrix.m30 = matrix3d.m41;
            matrix.m31 = matrix3d.m42;
            matrix.m32 = matrix3d.m43;
            matrix.m33 = matrix3d.m44;
            transform.push(Matrix4::Matrix(matrix));
          }
          LNTransform::Matrix3d(m) => {
            let mut matrix = Matrix::new();
            matrix.m00 = m.m11;
            matrix.m01 = m.m12;
            matrix.m02 = m.m13;
            matrix.m03 = m.m14;
            matrix.m10 = m.m21;
            matrix.m11 = m.m22;
            matrix.m12 = m.m23;
            matrix.m13 = m.m24;
            matrix.m20 = m.m31;
            matrix.m21 = m.m32;
            matrix.m22 = m.m33;
            matrix.m23 = m.m34;
            matrix.m30 = m.m41;
            matrix.m31 = m.m42;
            matrix.m32 = m.m43;
            matrix.m33 = m.m44;
            transform.push(Matrix4::Matrix(matrix));
          }
          _ => {}
        }
      }
    }
    Transform {
      id: prop.0,
      value: transform
    }
  }
}
