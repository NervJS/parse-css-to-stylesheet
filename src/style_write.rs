use std::{cell::RefCell, collections::HashMap, rc::Rc};

use lightningcss::properties::Property;

use swc_core::ecma::{
    ast::Program,
    visit::VisitMutWith
  };

use crate::{
  style_parser::StyleValue, style_propetries::unit::Platform, visitor::{JSXMutVisitor, JSXRecord, ModuleMutVisitor, SpanKey}
};

pub struct StyleWrite<'i> {
  pub module: Rc<RefCell<Program>>,
  pub jsx_record: Rc<RefCell<JSXRecord>>,
  pub pesudo_style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Vec<(String, Property<'i>)>)>>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
  pub is_enable_nesting: bool,
}

impl<'i> StyleWrite<'i> {
  pub fn new(
    module: Rc<RefCell<Program>>,
    jsx_record: Rc<RefCell<JSXRecord>>,
    pesudo_style_record: Rc<RefCell<HashMap<SpanKey, Vec<(String, Vec<(String, Property<'i>)>)>>>>,
    all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
    is_enable_nesting: bool
  ) -> Self {
    StyleWrite {
      module,
      jsx_record,
      pesudo_style_record,
      all_style,
      is_enable_nesting,
    }
  }

  pub fn write(&mut self, platform: Platform, taro_components: Vec<String>) {
    // 插入到jsx的style里
    {
      let mut jsx_mut_visitor =
        JSXMutVisitor::new(
          self.jsx_record.clone(), 
          self.all_style.clone(),
          self.pesudo_style_record.clone(),
          taro_components.clone(),
          platform.clone()
        );
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut jsx_mut_visitor);
    }
    // 插入样式表
    {
      let mut insert_mut_visitor = ModuleMutVisitor::new(self.all_style.clone(),platform.clone(), self.is_enable_nesting);
      self
        .module
        .borrow_mut()
        .visit_mut_with(&mut insert_mut_visitor);
    }
  }
}
