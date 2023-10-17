use std::{cell::RefCell, collections::HashMap, rc::Rc};

use swc_common::{
  errors::{ColorConfig, Handler},
  sync::Lrc,
  SourceMap,
};
use swc_ecma_ast::Program;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_visit::VisitMutWith;

use crate::{
  style_parser::StyleDeclaration,
  visitor::{JSXMutVisitor, JSXRecord, ModuleMutVisitor, SpanKey},
};

pub struct StyleWrite<'i> {
  pub module: Rc<RefCell<Program>>,
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleDeclaration<'i>>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleDeclaration<'i>>>>,
}

impl<'i> StyleWrite<'i> {
  pub fn new(
    module: Rc<RefCell<Program>>,
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, StyleDeclaration<'i>>>>,
    all_style: Rc<RefCell<HashMap<String, StyleDeclaration<'i>>>>,
  ) -> Self {
    StyleWrite {
      module,
      jsx_record,
      style_record,
      all_style,
    }
  }

  pub fn write(&mut self) {
    let insert_code = "
function __calc_style__(classnames, styleObj) {
  var styleObjs = [];
  var classes = classnames.split(' ');
  for (var i = 0; i < classes.length; i++) {
    styleObjs.push(classes[i] in window.__inner_style__ ? window.__inner_style__[classes[i]] : {});
  }
  styleObjs.push(styleObj);
  return Object.assign.apply(null, [{}].concat(styleObjs));
}
    ";
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(swc_common::FileName::Anon, insert_code.to_string());
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let lexer = Lexer::new(
      Syntax::default(),
      Default::default(),
      StringInput::from(&*fm),
      None,
    );
    let mut parser = Parser::new_from(lexer);
    let insert_module = Rc::new(RefCell::new(
      parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("解析插入代码失败"),
    ));
    {
      let mut jsx_mut_visitor =
        JSXMutVisitor::new(self.jsx_record.clone(), self.style_record.clone());
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut jsx_mut_visitor);
    }
    {
      let mut insert_mut_visitor =
        ModuleMutVisitor::new(self.all_style.clone(), insert_module.clone());
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut insert_mut_visitor);
    }
  }
}
