use lightningcss::{
  properties::{Property,text::Spacing},
  values
};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, Number, Callee, Ident, CallExpr};

use crate::style_transform::traits::ToExpr;

#[derive(Debug, Clone)]
pub enum LetterSpacing {
  Px(f32),
  Number(f32)
}

impl ToExpr for LetterSpacing {
  fn to_expr(&self) -> Expr {
    let mut letter_spacing = Expr::Lit(Lit::Null(swc_ecma_ast::Null { span: DUMMY_SP }));
    match self {
      LetterSpacing::Number(_) => {
        // 暂不支持数值类型
        // Expr::Call(CallExpr {
        //   span: DUMMY_SP,
        //   callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
        //     "convertLetterSpacing".into(),
        //     DUMMY_SP,
        //   )))),
        //   args: vec![
        //     Expr::Lit(Lit::Num(Number::from(*value as f64))).into()
        //   ],
        //   type_args: None,
        // })
      },
      LetterSpacing::Px(value) => {
        letter_spacing = Expr::Call(CallExpr {
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
    letter_spacing
  }
}

impl From<&Property<'_>> for LetterSpacing {
  fn from(value: &Property<'_>) -> Self {
    let mut letter_spacing = LetterSpacing::Number(1.0);
    match value {
      Property::LetterSpacing(value) => {
        match value {
          Spacing::Length(val) => {
            match val {
              values::length::Length::Value(value) => {
                match value {
                  values::length::LengthValue::Px(value) => {
                    // 匹配px单位
                    letter_spacing = LetterSpacing::Px(*value)
                  },
                  _ => {}
                }
              },
              _ => {}
            }
          }
          Spacing::Normal => {},
        }
      }
      _ => {}
    }
    letter_spacing
  }
}
