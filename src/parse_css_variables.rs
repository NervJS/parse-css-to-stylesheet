use std::collections::BTreeMap;

use lightningcss::{printer::PrinterOptions, properties::{custom::{Token, TokenOrValue}, Property}, traits::ToCss, values::time::Time};

use swc_core:: {
  common::{comments::SingleThreadedComments, sync::Lrc, SourceMap, DUMMY_SP},
  ecma::{
    ast::{ArrowExpr, AssignExpr, AssignOp, AssignTarget, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Callee, ComputedPropName, Decl, Expr, ExprOrSpread, ExprStmt, Ident, ImportDecl, ImportNamedSpecifier, ImportSpecifier, KeyValueProp, Lit, MemberExpr, MemberProp, Module, ModuleDecl, ModuleItem, Number, ObjectLit, ParenExpr, Pat, Program, Prop, PropName, PropOrSpread, ReturnStmt, SimpleAssignTarget, Stmt, VarDecl, VarDeclKind, VarDeclarator},
    codegen::{text_writer::JsWriter, Config, Emitter},
    utils::{quote_ident, quote_str},
    visit::{VisitAll, VisitAllWith}
  }
};
use crate::{constants::{CONVERT_STYLE_PX_FN, CSS_VARIABLE_MAP, CSS_VAR_FN, LAZY_CSS_VAR_FN}, document::JSXDocument, style_propetries::unit::{convert_color_keywords_to_hex, generate_expr_by_length_value, Platform}};

// 解析CSS变量
pub fn parse(properties: Vec<(String, Property<'_>)>) -> BTreeMap<String, Expr> {

  let mut css_variables = BTreeMap::new();
  properties.iter().for_each(|(key, value)| {
    let mut expr: Option<Expr> = None;
    match value.clone() {
      Property::Custom(custom) => {
        let token_or_value = custom.value.0.get(0);
        if let Some(token_or_value) = token_or_value {
          expr = Some(get_token_or_value(token_or_value.to_owned(), "css"));
        }
      },
      // 解析不出来的_unparsed
      Property::Unparsed(_) => {
        value.value_to_css_string(PrinterOptions::default()).unwrap();
      },
      _ => {}
    };
    if let Some(expr) = expr {
      css_variables.insert(key.to_string(), expr);
    }
  });
  css_variables
}

// 生成CSS变量代码
pub fn write(css_variables: BTreeMap<String, Expr>) -> Option<String> {

  if css_variables.len() == 0 {
    return None
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
        Stmt::Expr(ExprStmt {
          span: DUMMY_SP,
          expr: Box::new(Expr::Paren(ParenExpr {
            span: DUMMY_SP,
            expr: Box::new(obj)
          }))
        })
      )
    ],
    shebang: None,
  });
  // 生成代码
  let mut buf = vec![];
  {
    let mut emitter = Emitter {
      cfg: Config::default(),
      cm: cm.clone(),
      comments: Some(&comments),
      wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };
    emitter.emit_program(&program).unwrap();
  }
  
  Some(String::from_utf8(buf).unwrap().replace("\r\n", "\n"))
}

// 汇总css变量，生成代码
pub fn combine_css_variables (css_variable_strings: Vec<String>) -> Option<String> {

  if css_variable_strings.len() == 0 {
    return None
  }

  let mut document = JSXDocument::new();
  // 解析组件文件
  let cm: Lrc<SourceMap> = Default::default();
  let comments = SingleThreadedComments::default();
  let mut visitor = CssVariableVisitor {
    lits: vec![]
  };

  css_variable_strings.into_iter().for_each(|css_variable_string| {
    // 使用swc解析，并且拿到表达式
    let program = document.jsx_parse(css_variable_string, cm.clone(), &comments);
    program.visit_all_with(&mut visitor);
  });

  // 生成新的代码
  // import { var_fn, convertNumber2VP, globalCss } from "@tarojs/taro-runtime"
  // const CSS_VARIABLE_MAP = {...}
  // globalCss.map = CSS_VARIABLE_MAP
  let program  = Program::Module(Module {
    span: DUMMY_SP,
    body: vec![
      // import ...
      ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span:DUMMY_SP,
        phase: Default::default(),
        specifiers: vec![
          ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident::new(CSS_VAR_FN.into(), DUMMY_SP),
            imported: None,
            is_type_only: false,
          }),
          ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident::new(LAZY_CSS_VAR_FN.into(), DUMMY_SP),
            imported: None,
            is_type_only: false,
          }),
          ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident::new(CONVERT_STYLE_PX_FN.into(), DUMMY_SP),
            imported: None,
            is_type_only: false,
          }),
          ImportSpecifier::Named(ImportNamedSpecifier {
            span: DUMMY_SP,
            local: Ident::new("globalCss".into(), DUMMY_SP),
            imported: None,
            is_type_only: false,
          })
        ],
        src: quote_str!("@tarojs/runtime").into(),
        type_only:false, 
        with: None
      })), 
      ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Var,
        declare: false,
        decls: vec![VarDeclarator {
          span: DUMMY_SP,
          name: Pat::Ident(BindingIdent {
            id: quote_ident!(CSS_VARIABLE_MAP),
            type_ann: None,
          }),
          init: Some(Box::new(Expr::Object(ObjectLit {
            span: DUMMY_SP,
            props: visitor.lits.clone().into(),
          }))),
          definite: false,
        }],
      })))),
      ModuleItem::Stmt(Stmt::Expr(ExprStmt {
        span: DUMMY_SP,
        expr: Box::new(Expr::Assign(AssignExpr {
          span: DUMMY_SP,
          op: AssignOp::Assign,
          left: AssignTarget::Simple(SimpleAssignTarget::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(Expr::Ident(quote_ident!("globalCss"))),
            prop: MemberProp::Ident(quote_ident!("map")),
          })),
          right: Box::new(Expr::Ident(quote_ident!(CSS_VARIABLE_MAP))),
        })),
      })),
    ],
    shebang: None,
  });
  // 生成代码
  let mut buf = vec![];
  {
    let mut emitter = Emitter {
      cfg: Config::default(),
      cm: cm.clone(),
      comments: Some(&comments),
      wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };
    emitter.emit_program(&program).unwrap();
  }
  
  Some(String::from_utf8(buf).unwrap().replace("\r\n", "\n"))
}

struct CssVariableVisitor {
  lits: Vec<PropOrSpread>,
}

impl CssVariableVisitor {}

impl VisitAll for CssVariableVisitor {
  fn visit_object_lit(&mut self, n: &ObjectLit) {
    n.props.iter().for_each(|prop_or_spread| {
      self.lits.push(prop_or_spread.clone());
    });
  }
}

// 获取TokenOrValue的值
pub fn get_token_or_value (token_or_value: TokenOrValue<'_>, import_source: &str) -> Expr {
  match token_or_value {
    TokenOrValue::Token(token) => {
      match token {
        Token::Number { has_sign: _, value, int_value: _ } => {
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
          match import_source {
            "css" => {
              Expr::Arrow(ArrowExpr {
                span: DUMMY_SP,
                params: vec![],
                body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                    span: DUMMY_SP,
                    stmts: vec![
                      Stmt::Return(ReturnStmt {
                        span: DUMMY_SP,
                        arg: Some(Box::new(Expr::Member(MemberExpr {
                          span: DUMMY_SP,
                          obj: Box::new(Expr::Ident(quote_ident!(CSS_VARIABLE_MAP))),
                          prop: MemberProp::Computed(ComputedPropName {
                            span: DUMMY_SP,
                            expr: Box::new(Expr::Lit(Lit::Str(ident_string.clone().into()))),
                          }),
                        }))),
                      }),
                    ],
                  }),
                ),
                is_async: false,
                is_generator: false,
                type_params: None,
                return_type: None,
              })
            },
            _ => {
              Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(
                  Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(quote_ident!("globalCss"))),
                    prop: MemberProp::Ident(quote_ident!("map")),
                  })
                ),
                prop: MemberProp::Computed(ComputedPropName {
                  span: DUMMY_SP,
                  expr: Box::new(Expr::Lit(Lit::Str(ident_string.into()))),
                })
              })
            }
          }
        )
      });
      if let Some(fallback) = &var.fallback {
        // 仅支持两个参数
        fallback.0.iter().take(1).for_each(|token_or_value| {
          expr_or_spead.push(ExprOrSpread {
            spread: None,
            expr: Box::new(get_token_or_value(token_or_value.to_owned(), import_source)),
          })
        });
      }
      Expr::Call(CallExpr {
        span: Default::default(),
        callee: Callee::Expr(Box::new(Expr::Ident(quote_ident!(match import_source {
          "css" => LAZY_CSS_VAR_FN,
          _ => CSS_VAR_FN
            
        })))),
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
      match import_source {
        "css" => {
          Expr::Arrow(ArrowExpr {
            span: DUMMY_SP,
            params: vec![],
            body: Box::new(BlockStmtOrExpr::BlockStmt(BlockStmt {
                span: DUMMY_SP,
                stmts: vec![
                  Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(Expr::Member(MemberExpr {
                      span: DUMMY_SP,
                      obj: Box::new(Expr::Ident(quote_ident!(CSS_VARIABLE_MAP))),
                      prop: MemberProp::Computed(ComputedPropName {
                        span: DUMMY_SP,
                        expr: Box::new(Expr::Lit(Lit::Str(ident_string.clone().into()))),
                      }),
                    }))),
                  }),
                ],
              }),
            ),
            is_async: false,
            is_generator: false,
            type_params: None,
            return_type: None,
          })
        },
        _ => {
          Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(
              Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(Expr::Ident(quote_ident!("globalCss"))),
                prop: MemberProp::Ident(quote_ident!("map")),
              })
            ),
            prop: MemberProp::Computed(ComputedPropName {
              span: DUMMY_SP,
              expr: Box::new(Expr::Lit(Lit::Str(ident_string.into()))),
            })
          })
        }
      }
    },
  }
}