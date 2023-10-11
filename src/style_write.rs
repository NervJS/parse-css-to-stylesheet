use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ego_tree::NodeId;
use swc_ecma_ast::Program;
use swc_ecma_visit::VisitMutWith;

use crate::{
  style_parser::StyleDeclaration,
  visitor::{AstMutVisitor, JSXRecord},
};

pub struct StyleWrite<'i> {
  pub module: Rc<RefCell<Program>>,
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<NodeId, StyleDeclaration<'i>>>>,
}

impl<'i> StyleWrite<'i> {
  pub fn new(
    module: Rc<RefCell<Program>>,
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<NodeId, StyleDeclaration<'i>>>>,
  ) -> Self {
    StyleWrite {
      module,
      jsx_record,
      style_record,
    }
  }

  pub fn write(&mut self) {
    let mut style_visitor = AstMutVisitor::new(self.jsx_record.clone(), self.style_record.clone());
    self.module.borrow_mut().visit_mut_with(&mut style_visitor);
  }
}
