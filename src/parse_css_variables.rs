use std::{collections::HashMap, rc::Rc};

use lightningcss::{properties::{custom::{Token, TokenOrValue}, Property}, traits::ToCss, values::time::Time};
use swc_common::{comments::SingleThreadedComments, sync::Lrc, SourceMap, DUMMY_SP};
use swc_ecma_ast::{AssignExpr, AssignOp, BindingIdent, CallExpr, Callee, ComputedPropName, Decl, Expr, ExprOrSpread, ExprStmt, KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleItem, Number, ObjectLit, Pat, PatOrExpr, Program, Prop, PropName, PropOrSpread, Stmt, VarDecl, VarDeclKind, VarDeclarator};
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_utils::quote_ident;

use crate::style_propetries::unit::{convert_color_keywords_to_hex, generate_expr_by_length_value, generate_expr_with_css_input, Platform};


pub fn parse(properties: Vec<(String, Property<'_>)>) -> HashMap<String, Expr> {

  let mut css_variables = HashMap::new();
  properties.iter().for_each(|(key, value)| {
    let mut expr: Option<Expr> = None;
    match value.clone() {
      Property::Custom(custom) => {
        let token_or_value = custom.value.0.get(0);
        if let Some(token_or_value) = token_or_value {
          expr = Some(get_token_or_value(token_or_value.to_owned()));
        }
      },
      _ => {}
    };
    if let Some(expr) = expr {
      css_variables.insert(key.to_string(), expr);
    }
  });
  css_variables
}


pub fn write(css_variables: HashMap<String, Expr>) -> String {

  if css_variables.len() == 0 {
    return "".to_string();
  }

  let obj = Expr::Object(ObjectLit {
    span: DUMMY_SP,
    props: css_variables.iter().map(|(key, value)| {
      PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: Box::new(Expr::Lit(Lit::Str(key.to_string().into()))),
        }),
        value: Box::new(value.clone()),
      })))
    }).collect::<Vec<PropOrSpread>>().into(),
  });
  
  let cm: Lrc<SourceMap> = Default::default();
  let comments = SingleThreadedComments::default();
  let program  = Program::Module(Module {
    span: DUMMY_SP,
    body: vec![
      ModuleItem::Stmt(
        Stmt::Decl(
          Decl::Var(
            Box::new(
              VarDecl {
                span: DUMMY_SP,
                kind: VarDeclKind::Const,
                declare: false,
                decls: vec![
                  VarDeclarator {
                    span: DUMMY_SP,
                    name: Pat::Ident(BindingIdent::from(quote_ident!("css_var_map"))),
                    init: Some(Box::new(obj)),
                    definite: false,
                  }
                ]
              }
            )
          )
        )
      )
    ],
    shebang: None,
  });
  // 生成代码
  let mut buf = vec![];
  {
    let mut emitter = Emitter {
      cfg: swc_ecma_codegen::Config::default(),
      cm: cm.clone(),
      comments: Some(&comments),
      wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };
    emitter.emit_program(&program).unwrap();
  }
  String::from_utf8(buf).unwrap().replace("\r\n", "\n")
}

pub fn get_token_or_value (token_or_value: TokenOrValue<'_>) -> Expr {
  match token_or_value {
    TokenOrValue::Token(token) => {
      match token {
        Token::Number { has_sign, value, int_value } => {
          Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: value as f64,
            raw: None,
          }))
        },
        val => {Expr::Lit(Lit::Str(val.to_css_string(Default::default()).unwrap().into()))}
      }
    },
    TokenOrValue::Color(color) => {
      let color_string = convert_color_keywords_to_hex(color.to_css_string(lightningcss::stylesheet::PrinterOptions {
        minify: false,
        targets: lightningcss::targets::Targets {
          include: lightningcss::targets::Features::HexAlphaColors,
          ..lightningcss::targets::Targets::default()
        },
        ..lightningcss::stylesheet::PrinterOptions::default()
      }).unwrap());
      Expr::Lit(Lit::Str(color_string.into()))
    },
    TokenOrValue::UnresolvedColor(_) => {
      // 解析不到的颜色
      Expr::Lit(Lit::Str("".into()))
    },
    TokenOrValue::Url(url) => {
      // url("https://www.example.com") => { src: "https://www.example.com" }
      let url_string = url.url.to_string();
      Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![
          PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(quote_ident!("src")),
            value: Box::new(Expr::Lit(Lit::Str(url_string.into()))),
          })))
        ]
        .into(),
      })
    },
    TokenOrValue::Var(var) => {
      // var(--color-primary, #000000) => var_fn(css_var_map["--color-primary"], "#000000")
      let mut expr_or_spead = vec![];
      let ident_string = var.name.to_css_string(Default::default()).unwrap();
      expr_or_spead.push(ExprOrSpread {
        spread: None,
        expr: Box::new(
          Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(quote_ident!("css_var_map"))),
            prop: MemberProp::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Box::new(Expr::Lit(Lit::Str(ident_string.into()))),
            })
          })
        )
      });
      if let Some(fallback) = &var.fallback {
        fallback.0.iter().for_each(|token_or_value| {
          expr_or_spead.push(ExprOrSpread {
            spread: None,
            expr: Box::new(get_token_or_value(token_or_value.to_owned())),
          })
        });
      }
      Expr::Call(CallExpr {
        span: Default::default(),
        callee: Callee::Expr(Box::new(Expr::Ident(quote_ident!("var_fn")))),
        args: expr_or_spead,
        type_args: None,
      })
     
    },
    TokenOrValue::Env(_) => {
      // 环境变量
      Expr::Lit(Lit::Str("".into()))
    },
    TokenOrValue::Function(_) => {
      // 函数
      Expr::Lit(Lit::Str("".into()))
    },
    TokenOrValue::Length(length_value) => {
      generate_expr_by_length_value(&length_value, Platform::Harmony)
    },
    TokenOrValue::Angle(angle) => {
      let angle_string = angle.to_css_string(Default::default()).unwrap();
      Expr::Lit(Lit::Str(angle_string.into()))
    },
    TokenOrValue::Time(time) => {
      let time_num = match time {
        Time::Seconds(s) => s,
        Time::Milliseconds(m) => m * 60.0,
      };
      Expr::Lit(Lit::Num(Number {
        span: DUMMY_SP,
        value: time_num as f64,
        raw: None,
      }))
    },
    TokenOrValue::Resolution(resolution) => {
      let string = resolution.to_css_string(Default::default()).unwrap();
      Expr::Lit(Lit::Str(string.into()))
    },
    TokenOrValue::DashedIdent(dashed_ident) => {
      let ident_string = dashed_ident.to_css_string(Default::default()).unwrap();
      Expr::Member(MemberExpr {
        span: DUMMY_SP,
        obj: Box::new(Expr::Ident(quote_ident!("css_var_map"))),
        prop: MemberProp::Computed(ComputedPropName {
          span: DUMMY_SP,
          expr: Box::new(Expr::Lit(Lit::Str(ident_string.into()))),
        })
      })
    },
  }
}