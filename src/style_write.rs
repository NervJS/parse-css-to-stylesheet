use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lightningcss::properties::Property;
use swc_ecma_ast::Program;
use swc_ecma_visit::VisitMutWith;

use crate::{
  style_parser::StyleValue,
  visitor::{JSXMutVisitor, JSXRecord, ModuleMutVisitor, SpanKey},
};

pub struct StyleWrite<'i> {
  pub module: Rc<RefCell<Program>>,
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub style_record: Rc<RefCell<HashMap<SpanKey, HashMap<String, Property<'i>>>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

impl<'i> StyleWrite<'i> {
  pub fn new(
    module: Rc<RefCell<Program>>,
    jsx_record: Rc<RefCell<JSXRecord>>,
    style_record: Rc<RefCell<HashMap<SpanKey, HashMap<String, Property<'i>>>>>,
    all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
  ) -> Self {
    StyleWrite {
      module,
      jsx_record,
      style_record,
      all_style,
    }
  }

  pub fn write(&mut self) {
    {
      let mut jsx_mut_visitor =
        JSXMutVisitor::new(self.jsx_record.clone(), self.style_record.clone());
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut jsx_mut_visitor);
    }
    {
      let mut insert_mut_visitor = ModuleMutVisitor::new(self.all_style.clone());
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut insert_mut_visitor);
    }
  }
}
