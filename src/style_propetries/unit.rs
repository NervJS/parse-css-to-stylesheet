use lightningcss::{values::length::LengthValue, traits::ToCss, stylesheet::PrinterOptions};
use pcre2::bytes::Regex;

use swc_core::ecma::ast::*;
use swc_core::common::DUMMY_SP;
use crate::{constants::{CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_PX_FN, RN_CONVERT_STYLE_VU_FN}, generate_expr_lit_num, generate_expr_lit_str};

use super::style_property_type::CSSPropertyType;

#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
  ReactNative,
  Harmony
}

pub enum PropertyTuple {
  // 一对一属性：height: 100px 解析 => (height, "100px")
  One(CSSPropertyType, Expr),
  // 一对多属性：flex: 1 解析 => vec![(flexGrow, "1"), (flexShrink, "1"), (flexBasis, "0%")]
  Array(Vec<(CSSPropertyType, Expr)>)
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
          return generate_expr_lit_num!(*num as f64)
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
          return generate_expr_lit_num!((*num * 16.0) as f64)
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
          return generate_expr_lit_str!(format!("{}vh", num))
          // handler = Some(CONVERT_STYLE_PX_FN.to_string());
          // args.push(generate_expr_lit_num!(*num as f64));
          // args.push(generate_expr_lit_str!("vh"));
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
          return generate_expr_lit_str!(format!("{}vw", num))
          // handler = Some(CONVERT_STYLE_PX_FN.to_string());
          // args.push(generate_expr_lit_num!(*num as f64));
          // args.push(generate_expr_lit_str!("vw"));
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
          return generate_expr_lit_str!(format!("{}vmin", num))
          // handler = Some(CONVERT_STYLE_PX_FN.to_string());
          // args.push(generate_expr_lit_num!(*num as f64));
          // args.push(generate_expr_lit_str!("vmin"));
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
          return generate_expr_lit_str!(format!("{}vmax", num))
          // handler = Some(CONVERT_STYLE_PX_FN.to_string());
          // args.push(generate_expr_lit_num!(*num as f64));
          // args.push(generate_expr_lit_str!("vmax"));
        }
      }
    },
    LengthValue::Ch(num) => {
      match platform {
        Platform::ReactNative => {
          handler = Some(RN_CONVERT_STYLE_VU_FN.to_string());
          args.push(generate_expr_lit_num!(*num as f64));
          args.push(generate_expr_lit_str!("PX"));
        },
        Platform::Harmony => {
          return generate_expr_lit_str!(format!("{}px", num))
          // handler = Some(CONVERT_STYLE_PX_FN.to_string());
          // args.push(generate_expr_lit_num!(*num as f64));
          // args.push(generate_expr_lit_str!("PX"));
        }
      }
    }
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


pub fn generate_expr_with_css_input(input: String, platform: Platform) -> Expr {
  // 定义匹配 '16px' 的正则表达式
  let re = Regex::new(r"(-?(?P<num>\d+(\.\d*)?|\.\d+))(?P<unit>(px|vw|vh|pX|PX|Px)?)").unwrap();
  let bytes = input.as_bytes();
  // 使用正则表达式进行匹配
  if let Ok(caps) = re.captures(bytes) {
    if let Some(caps) = caps {
      // 提取匹配到的数字部分
      let input_str =  std::str::from_utf8(&caps["num"]);
      let unit = match std::str::from_utf8(&caps["unit"]) {
        Ok(s) => s,
        Err(_) => "vp",
      };
      if let Ok(input_str) = input_str {
        if let Ok(number) = input_str.parse::<f64>() {
          match unit {
            "vw" | "vh" | "vmin" | "vmax" => {
              return generate_expr_lit_str!(format!("{}{}", number, unit))
            },
            "px" => {
              return generate_expr_lit_num!(number)
            },
            "rem" => {
              return generate_expr_lit_num!(number * 16.0)
            },
            "pX" | "PX" | "Px" => {
              return generate_expr_lit_str!(format!("{}px", number))
            },
            _ => {
              // 如果没有单位，则认为是纯数字，返回 Expr::Num
              return generate_expr_lit_num!(number);
            }
          };
        } 
      }
    }
  }
  // 如果没有匹配到，则返回原始字符串
  Expr::Lit(Lit::Str(input.into()))
}


// 处理将color关键字转换为hex
// 参考颜色关键字：https://www.w3.org/TR/css-color-3/#svg-color
pub fn convert_color_keywords_to_hex(color: String) -> String {
  match color.as_str() {
    "aliceblue" => "#F0F8FF".to_string(),
    "antiquewhite" => "#FAEBD7".to_string(),
    "aqua" => "#00FFFF".to_string(),
    "aquamarine" => "#7FFFD4".to_string(),
    "azure" => "#F0FFFF".to_string(),
    "beige" => "#F5F5DC".to_string(),
    "bisque" => "#FFE4C4".to_string(),
    "black" => "#000000".to_string(),
    "blanchedalmond" => "#FFEBCD".to_string(),
    "blue" => "#0000FF".to_string(),
    "blueviolet" => "#8A2BE2".to_string(),
    "brown" => "#A52A2A".to_string(),
    "burlywood" => "#DEB887".to_string(),
    "cadetblue" => "#5F9EA0".to_string(),
    "chartreuse" => "#7FFF00".to_string(),
    "chocolate" => "#D2691E".to_string(),
    "coral" => "#FF7F50".to_string(),
    "cornflowerblue" => "#6495ED".to_string(),
    "cornsilk" => "#FFF8DC".to_string(),
    "crimson" => "#DC143C".to_string(),
    "cyan" => "#00FFFF".to_string(),
    "darkblue" => "#00008B".to_string(),
    "darkcyan" => "#008B8B".to_string(),
    "darkgoldenrod" => "#B8860B".to_string(),
    "darkgray" => "#A9A9A9".to_string(),
    "darkgreen" => "#006400".to_string(),
    "darkgrey" => "#A9A9A9".to_string(),
    "darkkhaki" => "#BDB76B".to_string(),
    "darkmagenta" => "#8B008B".to_string(),
    "darkolivegreen" => "#556B2F".to_string(),
    "darkorange" => "#FF8C00".to_string(),
    "darkorchid" => "#9932CC".to_string(),
    "darkred" => "#8B0000".to_string(),
    "darksalmon" => "#E9967A".to_string(),
    "darkseagreen" => "#8FBC8F".to_string(),
    "darkslateblue" => "#483D8B".to_string(),
    "darkslategray" => "#2F4F4F".to_string(),
    "darkslategrey" => "#2F4F4F".to_string(),
    "darkturquoise" => "#00CED1".to_string(),
    "darkviolet" => "#9400D3".to_string(),
    "deeppink" => "#FF1493".to_string(),
    "deepskyblue" => "#00BFFF".to_string(),
    "dimgray" => "#696969".to_string(),
    "dimgrey" => "#696969".to_string(),
    "dodgerblue" => "#1E90FF".to_string(),
    "firebrick" => "#B22222".to_string(),
    "floralwhite" => "#FFFAF0".to_string(),
    "forestgreen" => "#228B22".to_string(),
    "fuchsia" => "#FF00FF".to_string(),
    "gainsboro" => "#DCDCDC".to_string(),
    "ghostwhite" => "#F8F8FF".to_string(),
    "gold" => "#FFD700".to_string(),
    "goldenrod" => "#DAA520".to_string(),
    "gray" => "#808080".to_string(),
    "green" => "#008000".to_string(),
    "greenyellow" => "#ADFF2F".to_string(), 
    "grey" => "#808080".to_string(),
    "honeydew" => "#F0FFF0".to_string(),
    "hotpink" => "#FF69B4".to_string(),
    "indianred" => "#CD5C5C".to_string(),
    "indigo" => "#4B0082".to_string(),
    "ivory" => "#FFFFF0".to_string(),
    "khaki" => "#F0E68C".to_string(),
    "lavender" => "#E6E6FA".to_string(),
    "lavenderblush" => "#FFF0F5".to_string(),
    "lawngreen" => "#7CFC00".to_string(),
    "lemonchiffon" => "#FFFACD".to_string(),
    "lightblue" => "#ADD8E6".to_string(),
    "lightcoral" => "#F08080".to_string(),
    "lightcyan" => "#E0FFFF".to_string(),
    "lightgoldenrodyellow" => "#FAFAD2".to_string(),
    "lightgray" => "#D3D3D3".to_string(),
    "lightgreen" => "#90EE90".to_string(),
    "lightgrey" => "#D3D3D3".to_string(),
    "lightpink" => "#FFB6C1".to_string(),
    "lightsalmon" => "#FFA07A".to_string(),
    "lightseagreen" => "#20B2AA".to_string(),
    "lightskyblue" => "#87CEFA".to_string(),
    "lightslategray" => "#778899".to_string(),
    "lightslategrey" => "#778899".to_string(),
    "lightsteelblue" => "#B0C4DE".to_string(),
    "lightyellow" => "#FFFFE0".to_string(),
    "lime" => "#00FF00".to_string(),
    "limegreen" => "#32CD32".to_string(),
    "linen" => "#FAF0E6".to_string(),
    "magenta" => "#FF00FF".to_string(),
    "maroon" => "#800000".to_string(),
    "mediumaquamarine" => "#66CDAA".to_string(),
    "mediumblue" => "#0000CD".to_string(),
    "mediumorchid" => "#BA55D3".to_string(),
    "mediumpurple" => "#9370DB".to_string(),
    "mediumseagreen" => "#3CB371".to_string(),
    "mediumslateblue" => "#7B68EE".to_string(),
    "mediumspringgreen" => "#00FA9A".to_string(),
    "mediumturquoise" => "#48D1CC".to_string(),
    "mediumvioletred" => "#C71585".to_string(),
    "midnightblue" => "#191970".to_string(),
    "mintcream" => "#F5FFFA".to_string(),
    "mistyrose" => "#FFE4E1".to_string(),
    "moccasin" => "#FFE4B5".to_string(),
    "navajowhite" => "#FFDEAD".to_string(),
    "navy" => "#000080".to_string(),
    "oldlace" => "#FDF5E6".to_string(),
    "olive" => "#808000".to_string(),
    "olivedrab" => "#6B8E23".to_string(),
    "orange" => "#FFA500".to_string(),
    "orangered" => "#FF4500".to_string(),
    "orchid" => "#DA70D6".to_string(),
    "palegoldenrod" => "#EEE8AA".to_string(),
    "palegreen" => "#98FB98".to_string(),
    "paleturquoise" => "#AFEEEE".to_string(),
    "palevioletred" => "#DB7093".to_string(),
    "papayawhip" => "#FFEFD5".to_string(),
    "peachpuff" => "#FFDAB9".to_string(),
    "peru" => "#CD853F".to_string(),
    "pink" => "#FFC0CB".to_string(),
    "plum" => "#DDA0DD".to_string(),
    "powderblue" => "#B0E0E6".to_string(),
    "purple" => "#800080".to_string(),
    "red" => "#FF0000".to_string(),
    "rosybrown" => "#BC8F8F".to_string(),
    "royalblue" => "#4169E1".to_string(),
    "saddlebrown" => "#8B4513".to_string(),
    "salmon" => "#FA8072".to_string(),
    "sandybrown" => "#F4A460".to_string(),
    "seagreen" => "#2E8B57".to_string(),
    "seashell" => "#FFF5EE".to_string(),
    "sienna" => "#A0522D".to_string(),
    "silver" => "#C0C0C0".to_string(),
    "skyblue" => "#87CEEB".to_string(),
    "slateblue" => "#6A5ACD".to_string(),
    "slategray" => "#708090".to_string(),
    "slategrey" => "#708090".to_string(),
    "snow" => "#FFFAFA".to_string(),
    "springgreen" => "#00FF7F".to_string(),
    "steelblue" => "#4682B4".to_string(),
    "tan" => "#D2B48C".to_string(),
    "teal" => "#008080".to_string(),
    "thistle" => "#D8BFD8".to_string(),
    "tomato" => "#FF6347".to_string(),
    "turquoise" => "#40E0D0".to_string(),
    "violet" => "#EE82EE".to_string(),
    "wheat" => "#F5DEB3".to_string(),
    "white" => "#FFFFFF".to_string(),
    "whitesmoke" => "#F5F5F5".to_string(),
    "yellow" => "#FFFF00".to_string(),
    "yellowgreen" => "#9ACD32".to_string(),
    "currentColor" => "".to_string(),
    _ => color
  }
}