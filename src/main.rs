use std::{fs, convert::Infallible};

use cssparser::{ParserInput, Parser as CSSParser, StyleSheetParser, QualifiedRuleParser, AtRuleParser};
use swc_common::{sync::Lrc, SourceMap, errors::{Handler, ColorConfig}};
use swc_ecma_ast::{EsVersion, JSXElement, JSXAttrOrSpread, JSXAttrName, JSXAttrValue, Lit};
use swc_ecma_parser::{lexer::Lexer, Syntax, TsConfig, StringInput, Parser};
use swc_ecma_visit::{Visit, VisitWith};
use lightningcss::{stylesheet::{StyleSheet, ParserOptions}, visitor::{Visitor, VisitTypes, Visit as CSSVisit}, rules::{CssRule, style::StyleRule}, visit_types};

struct JSXVisitor;

impl Visit for JSXVisitor {
  fn visit_jsx_element(
    &mut self,
    jsx: &JSXElement,
  ) {
    // 打印 JSX 元素的 className 属性
    for attr in &jsx.opening.attrs {
      if let JSXAttrOrSpread::JSXAttr(attr) = attr {
        if let JSXAttrName::Ident(ident) = &attr.name {
          if ident.sym.to_string() == "className" {
            if let Some(value) = &attr.value {
              match value {
                JSXAttrValue::Lit(lit) => {
                  if let Lit::Str(str) = lit {
                    println!("className: {}", str.value);
                  }
                },
                _ => {}
              }
            }
          }
        }
      }
    }
    jsx.visit_children_with(self);
  }
}

// struct CSSRuleParser;

// impl<'i> QualifiedRuleParser<'i> for CSSRuleParser {
//   type Prelude = ();
//   type QualifiedRule = ();
//   type Error = ();

//   fn parse_block<'t>(
//           &mut self,
//           prelude: Self::Prelude,
//           start: &cssparser::ParserState,
//           input: &mut CSSParser<'i, 't>,
//       ) -> Result<Self::QualifiedRule, cssparser::ParseError<'i, Self::Error>> {
//     println!("parse_block");
//     Ok(())
//   }
// }

// impl<'i> AtRuleParser<'i> for CSSRuleParser {
//   type Prelude = ();
//   type AtRule = ();
//   type Error = ();

//   fn parse_block<'t>(
//           &mut self,
//           prelude: Self::Prelude,
//           start: &cssparser::ParserState,
//           input: &mut CSSParser<'i, 't>,
//       ) -> Result<Self::AtRule, cssparser::ParseError<'i, Self::Error>> {
//     println!("parse_block at rule");
//     Ok(())
//   }
// }

struct StyleVisitor;

impl<'i> Visitor<'i> for StyleVisitor {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        println!("{:?}", style.selectors);
        println!();
      },
      _ => {}
    }
    Ok(())
  }
}

struct NodeRef {}

fn main() {
  // 使用 swc 解析 JSX
  let jsx = fs::read_to_string("asset/mod.jsx").unwrap();
  let css = fs::read_to_string("asset/Mod.scss").unwrap();

  // 初始化 swc 的 SourceMap
  let cm: Lrc<SourceMap> = Default::default();
  // 初始化 swc 的错误处理器
  let handler = Handler::with_tty_emitter(
    ColorConfig::Auto,
    true,
    false,
    Some(cm.clone()),
  );

  // 将 JSX 代码转换为 SourceFile
  let fm = cm.new_source_file(
    swc_common::FileName::Anon,
    jsx,
  );

  // 初始化 swc 的词法分析器
  let lexer = Lexer::new(
    Syntax::Typescript(
      TsConfig {
        tsx: true,
        ..Default::default()
      }
    ),
    EsVersion::Es2019,
    StringInput::from(&*fm),
    None
  );
  // 初始化 swc 的语法分析器
  let mut parser = Parser::new_from(lexer);
  for e in parser.take_errors() {
    e.into_diagnostic(&handler).emit();
  }

  let module = parser
    .parse_module()
    .map_err(|e| {
      e.into_diagnostic(&handler).emit()
    })
    .expect("failed to parser module");
  // 遍历语法树，查找 JSX 节点
  let mut vistor = JSXVisitor;
  module.visit_with(&mut vistor);

  // 解析 CSS 代码，解析出所有规则
  // let mut input = ParserInput::new(&css);
  // let mut parser = CSSParser::new(&mut input);

  // let mut custom_parser = CSSRuleParser;
  // let style_sheet_parser = StyleSheetParser::new(input, &mut custom_parser);
  // let style_sheet: Vec<_> = style_sheet_parser.collect();
  // println!("{:?}", style_sheet);

  let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
  stylesheet.visit(&mut StyleVisitor).unwrap();
  
}
