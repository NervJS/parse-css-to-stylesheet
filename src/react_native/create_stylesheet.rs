

use std::{collections::HashMap, fmt::format};

use swc_common::{errors::{Handler, ColorConfig}, comments::SingleThreadedComments, SourceMap, sync::Lrc, Globals, Mark, GLOBALS, Span, DUMMY_SP};
use swc_ecma_ast::{EsVersion, Program, Module, Expr, ObjectLit, PropOrSpread, Prop, KeyValueProp, PropName, Ident, ExportDefaultExpr, CallExpr, ModuleItem, BlockStmt, Stmt, ExprStmt, ExprOrSpread, MemberExpr, Callee, MemberProp, ModuleDecl, ComputedPropName, Lit, Str};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_transforms_base::{resolver, fixer::fixer, hygiene::hygiene};
use swc_ecma_visit::FoldWith;
use swc_ecmascript::transforms::typescript::strip;
use swc_atoms::Atom;

use crate::{style_propetries::{style_value_type::StyleValueType, traits::ToStyleValue, unit::Platform}, generate_expr_lit_str};

pub struct RNStyleSheet {
  pub cm: Option<Lrc<SourceMap>>,
  pub program: Option<Program>,
  pub style_data: HashMap<String, HashMap<String, StyleValueType>>
}

impl RNStyleSheet {
  pub fn new(style_data: HashMap<String, HashMap<String, StyleValueType>>) -> Self {
    Self {
      program: None,
      cm: None,
      style_data: style_data
    }
  }

  // 根据style_data生成StyleSheet的AST代码
  fn generate_ast (&mut self) -> Program {
    // 规则输出
    let mut rules = vec![];
    for (key, value) in self.style_data.iter() {
      let mut prop_or_spread = vec![];
      for (key, value) in value.iter() {
        prop_or_spread.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
          key: PropName::Ident(Ident {
            span: Span::default(),
            sym: Atom::new(key),
            optional: false
          }),
          value: Box::new(value.to_expr(Platform::ReactNative)),
        }))))
      }

      rules.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Computed(ComputedPropName {
          span: Span::default(),
          expr: Box::new(Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: Atom::new(key),
            raw: None
          })))
        }),
        value: Box::new(Expr::Object(ObjectLit {
          span: Span::default(),
          props: prop_or_spread
        })),
      }))))
    }

    // 生成导出语句
    // export default StyleSheet.create({})
    let style_sheet_ast =Program::Module(Module {
      span: Span::default(),
      body: vec![
        ModuleItem::ModuleDecl(
          ModuleDecl::ExportDefaultExpr(ExportDefaultExpr {
            span: Span::default(),
            expr: Box::new(Expr::Call(CallExpr {
              span: Span::default(),
              callee: Callee::Expr(Box::new(
                Expr::Member(MemberExpr {
                  span: Span::default(),
                  obj: Box::new(
                    Expr::Ident(Ident {
                      span: Span::default(),
                      sym: Atom::new("StyleSheet"),
                      optional: false
                    })
                  ),
                  prop: MemberProp::Ident(Ident {
                    span: Span::default(),
                    sym: Atom::new("create"),
                    optional: false
                  }),
                })
              )),
              args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Object(ObjectLit {
                  span: Span::default(),
                  props: rules,
                }))
              }],
              type_args: None
            }))
          })
        )
      ],
      shebang: None
    });

    style_sheet_ast
  }
  

  pub fn create (&mut self, cm: Lrc<SourceMap>, comments: &SingleThreadedComments) {
    self.cm = Some(cm.clone());

    // 初始化 swc 的错误处理器
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
  
    let ast_program = self.generate_ast();
    let export_code = self.program_to_code(ast_program);

  
    let mut template_code = r#"
      import { StyleSheet } from 'react-native'
      import { scalePx2dp, scaleVu2dp } from '@tarojs/runtime-rn'

      // 用来标识 rn-runner transformer 是否读写缓存
      function ignoreStyleFileCache() {}

    "#;

    // 将 JSX 代码转换为 SourceFile
    let fm = cm.new_source_file(swc_common::FileName::Anon, format!("{}{}", template_code, export_code));

    // 初始化 swc 的词法分析器
    let lexer = Lexer::new(
      Syntax::Typescript(TsConfig {
        tsx: true,
        decorators: true,
        ..Default::default()
      }),
      EsVersion::Es2019,
      StringInput::from(&*fm),
      Some(comments),
    );
  
    // 初始化 swc 的语法分析器
    let mut parser = Parser::new_from(lexer);
    for e in parser.take_errors() {
      e.into_diagnostic(&handler).emit();
    }
    let program = parser
      .parse_program()
      .map_err(|e| e.into_diagnostic(&handler).emit())
      .expect("解析 JSX 失败");
    let globals = Globals::default();
    GLOBALS.set(&globals, || {
      let unresolved_mark = Mark::new();
      let top_level_mark = Mark::new();
      let program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, true));
      let program = program.fold_with(&mut strip(top_level_mark));
      let program = program.fold_with(&mut hygiene());
      let program = program.fold_with(&mut fixer(Some(comments)));
      self.program = Some(program);
    });
  
  }

  pub fn codegen (&mut self) -> String {
    let mut buf = Vec::new();
    let cm = self.cm.clone().unwrap();
    let mut emitter = Emitter {
      cfg: swc_ecma_codegen::Config::default(),
      cm: cm.clone(),
      comments: None,
      wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None))
    };
    let program = self.program.as_ref().unwrap();
    emitter.emit_program(program).expect("StyleSheet生成失败");
    let code = String::from_utf8(buf).unwrap().replace("\r\n", "\n");
    code
  }

  // program -> ast -> code
  pub fn program_to_code (&mut self, program: Program) -> String {
    let mut buf = Vec::new();
    let cm = self.cm.clone().unwrap();
    let mut emitter = Emitter {
      cfg: swc_ecma_codegen::Config::default(),
      cm: cm.clone(),
      comments: None,
      wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None))
    };
    emitter.emit_program(&program).expect("StyleSheet生成失败");
    let code = String::from_utf8(buf).unwrap().replace("\r\n", "\n");
    code
  }
}



