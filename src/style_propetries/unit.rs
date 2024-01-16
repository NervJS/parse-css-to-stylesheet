use lightningcss::{values::length::LengthValue, traits::ToCss, stylesheet::PrinterOptions};
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ExprOrSpread, Expr,  Callee, Ident, CallExpr};
use crate::{generate_expr_lit_num, generate_expr_lit_str, constants::{RN_CONVERT_STYLE_PX_FN, CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN}};

pub enum Platform {
  ReactNative,
  Harmony
}

// 根据长度单位生成对应的表达式
pub fn generate_expr_by_length_value(length_value: &LengthValue, platform: Platform) -> Expr {
  let mut args: Vec<Expr> = vec![];
  let mut handler: Option<String> = None;
  
  match length_value {
    LengthValue::Px(num,) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        }
      }
    },
    LengthValue::Rem(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        }
      }
    },
    LengthValue::Vh(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vh"));
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vh"));
        }
      }
    },
    LengthValue::Vw(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vw"));
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vw"));
        }
      }
    },
    LengthValue::Vmin(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vmin"));
        }
      }
    },
    LengthValue::Vmax(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64))
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("vmax"));
        }
      }
    },
   _ => {}

  }

  if let Some(handler_name) = handler {
    Expr::Call(CallExpr {
      span: DUMMY_SP,
      callee: Callee::Expr(Box::new(Expr::Ident(Ident::new(
        handler_name.into(),
        DUMMY_SP
      )))),
      args: args.into_iter().map(|arg| ExprOrSpread {
        spread: None,
        expr: Box::new(arg),
      }).collect(),
      type_args: None,
    })
  } else {
    generate_expr_lit_str!(length_value.to_css_string(PrinterOptions::default()).unwrap())
  }


}