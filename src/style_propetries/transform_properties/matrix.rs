use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;

use crate::generate_expr_lit_num;
use crate::{generate_expr_enum, style_propetries::transform_properties::ETransformType};


#[derive(Debug, Clone)]
pub struct Matrix {
  pub m00: f32,
  pub m01: f32,
  pub m02: f32,
  pub m03: f32,
  pub m10: f32,
  pub m11: f32,
  pub m12: f32,
  pub m13: f32,
  pub m20: f32,
  pub m21: f32,
  pub m22: f32,
  pub m23: f32,
  pub m30: f32,
  pub m31: f32,
  pub m32: f32,
  pub m33: f32,
}

impl Matrix {
  
  pub fn new() -> Self {
    Matrix {
      m00: 1.0,
      m01: 0.0,
      m02: 0.0,
      m03: 0.0,
      m10: 0.0,
      m11: 1.0,
      m12: 0.0,
      m13: 0.0,
      m20: 0.0,
      m21: 0.0,
      m22: 1.0,
      m23: 0.0,
      m30: 0.0,
      m31: 0.0,
      m32: 0.0,
      m33: 1.0,
    }
  }

  pub fn to_vec(&self) -> Vec<f32> {
    vec![
        self.m00, self.m01, self.m02, self.m03,
        self.m10, self.m11, self.m12, self.m13,
        self.m20, self.m21, self.m22, self.m23,
        self.m30, self.m31, self.m32, self.m33,
    ]
}

  pub fn to_expr_or_spread(&self) -> Option<ExprOrSpread> {
    let mut props = vec![];

    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
        value: Box::new(generate_expr_enum!(ETransformType::Matrix))
    }))));

    props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("matrix".into(), DUMMY_SP)),
        value: Box::new(Expr::Array(ArrayLit {
          span: Default::default(),
          elems: self.to_vec().iter().map(|x| Some(generate_expr_lit_num!(Into::<f64>::into(*x)).into())).collect(),
        }))
    }))));

    Some(ExprOrSpread {
      spread: None,
      expr: Box::new(Expr::Object(ObjectLit {
          span: Default::default(),
          props: props
      }))
  })
  }
  
  // pub fn to_expr(&self) -> Vec<Expr> {
    
  //   let expr = Expr::Object(ObjectLit {
  //     span: DUMMY_SP,
  //     props: vec![
  //       PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
  //         key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
  //         value: Expr::Lit(Lit::Str(swc_ecma_ast::Str {
  //           span: DUMMY_SP,
  //           value: "Matrix".into(),
  //           raw: None
  //         })).into(),
  //       }))),
  //       PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
  //         key: PropName::Ident(Ident::new("value".into(), DUMMY_SP)),
  //         value: Expr::Array(ArrayLit {
  //           span: DUMMY_SP,
  //           elems: vec![
  //             Some(generate_expr_lit_num!(self.m00 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m01 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m02 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m03 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m10 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m11 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m12 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m13 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m20 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m21 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m22 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m23 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m30 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m31 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m32 as f64).into()),
  //             Some(generate_expr_lit_num!(self.m33 as f64).into()),
  //           ],
  //         })
  //         .into(),
  //       }))),
  //     ]
  //   });
  //   vec![expr]
  // }

  pub fn to_rn_expr(&self) -> Vec<Expr> {
    vec![
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(Ident::new("matrix".into(), DUMMY_SP)),
            value: Expr::Array(ArrayLit {
              span: DUMMY_SP,
              elems: vec![
                Some(generate_expr_lit_num!(self.m00 as f64).into()),
                Some(generate_expr_lit_num!(self.m01 as f64).into()),
                Some(generate_expr_lit_num!(self.m02 as f64).into()),
                Some(generate_expr_lit_num!(self.m03 as f64).into()),
                Some(generate_expr_lit_num!(self.m10 as f64).into()),
                Some(generate_expr_lit_num!(self.m11 as f64).into()),
                Some(generate_expr_lit_num!(self.m12 as f64).into()),
                Some(generate_expr_lit_num!(self.m13 as f64).into()),
                Some(generate_expr_lit_num!(self.m20 as f64).into()),
                Some(generate_expr_lit_num!(self.m21 as f64).into()),
                Some(generate_expr_lit_num!(self.m22 as f64).into()),
                Some(generate_expr_lit_num!(self.m23 as f64).into()),
                Some(generate_expr_lit_num!(self.m30 as f64).into()),
                Some(generate_expr_lit_num!(self.m31 as f64).into()),
                Some(generate_expr_lit_num!(self.m32 as f64).into()),
                Some(generate_expr_lit_num!(self.m33 as f64).into()),
              ],
            })
            .into(),
          }))),
        ]
      })
    ]
  }
}
