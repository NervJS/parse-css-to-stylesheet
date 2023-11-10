use lightningcss::{
  properties::{Property, font},
  values
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number, Callee, Ident, CallExpr};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum LineHeight {
  Px(f32),
  Number(f32)
}

impl ToExpr for LineHeight {
  fn to_expr(&self) -> Expr {
    let mut line_height = Expr::Lit(Lit::Null(swc_ecma_ast::Null { span: DUMMY_SP }));
    match self {
      LineHeight::Number(_) => {
        // 暂不支持数值类型
        // Expr::Call(CallExpr {
        //   span: DUMMY_SP,
        //   callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
        //     "convertLineHeight".into(),
        //     DUMMY_SP,
        //   )))),
        //   args: vec![
        //     Expr::Lit(Lit::Num(Number::from(*value as f64))).into()
        //   ],
        //   type_args: None,
        // })
      },
      LineHeight::Px(value) => {
        line_height = Expr::Call(CallExpr {
          span: DUMMY_SP,
          callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
            "convertPx".into(),
            DUMMY_SP,
          )))),
          args: vec![
            Expr::Lit(Lit::Num(Number::from(*value as f64))).into()
          ],
          type_args: None,
        })
      }
    }
    line_height
  }
}

impl From<&Property<'_>> for LineHeight {
  fn from(value: &Property<'_>) -> Self {
    let mut line_height = LineHeight::Number(1.0);
    match value {
      Property::LineHeight(value) => {
        match value {
          font::LineHeight::Length(val) => {
            match val {
              values::percentage::DimensionPercentage::Dimension(value) => {
                match value {
                  values::length::LengthValue::Px(value) => {
                    // 匹配px单位
                    line_height = LineHeight::Px(*value)
                  },
                  _ => {}
                }
              },
              _ => {}
            }
          },
          font::LineHeight::Number(val) => {
            line_height = LineHeight::Number(*val)
          },
          _ => {}
        }
      }
      _ => {}
    }
    line_height
  }
}
