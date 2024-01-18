use lightningcss::{values::length::LengthValue, traits::ToCss, stylesheet::PrinterOptions};
use regex::Regex;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ExprOrSpread, Expr,  Callee, Ident, CallExpr, Lit, Number, PropName};
use crate::{generate_expr_lit_num, generate_expr_lit_str, constants::{RN_CONVERT_STYLE_PX_FN, CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN}};

pub enum Platform {
  ReactNative,
  Harmony
}

pub enum PropertyTuple {
  // 一对一属性：height: 100px 解析 => (height, "100px")
  One(PropName, Expr),
  // 一对多属性：flex: 1 解析 => vec![(flexGrow, "1"), (flexShrink, "1"), (flexBasis, "0%")]
  Array(Vec<(PropName, Expr)>)
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
          args.push(generate_expr_lit_num!((*num * 16.0) as f64))
        },
        Platform::Harmony => {
          handler = Some(CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!((*num * 16.0) as f64))
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


pub fn generate_expr_with_css_input(input: String) -> Expr {
  // 定义匹配 '16px' 的正则表达式
  let re: Regex = Regex::new(r"(-?(\d+(\.\d*)?|\.\d+))((px)|(vw)|(vh))").unwrap();
  // 使用正则表达式进行匹配
  if let Some(captures) = re.captures(&input) {
    // 提取匹配到的数字部分
    let input_str = captures.get(1).unwrap().as_str();
    let unit = match captures.get(4) {
      Some(m) => m.as_str(),
      None => "vp"
    };

    if let Ok(number) = input_str.parse::<f64>() {

      let mut args: Vec<Expr> = vec![];
      let mut handler: Option<String> = None;

      match unit {
        "vw" | "vh" | "vmin" | "vmax" => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(number));
          args.push(generate_expr_lit_str!(unit));
        },
        "px" => {
          handler = Some(RN_CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(number));
        },
        "rem" => {
          handler = Some(RN_CONVERT_STYLE_PX_FN.to_string());
          args.push(generate_expr_lit_num!(number * 16.0));
        },
        _ => {}
      }

      // 替换原始字符串
      if let Some(handler_name) = handler {
        return Expr::Call(CallExpr {
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
      }
    } 
  }
  
  
  // 如果匹配到为纯数字，直接返回数字
  if let Ok(number) = input.parse::<f64>() {
    return Expr::Lit(Lit::Num(Number::from(number)));
  }
  // 如果没有匹配到，则返回原始字符串
  Expr::Lit(Lit::Str(input.into()))
}
