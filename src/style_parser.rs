use std::{collections::HashMap, convert::Infallible, rc::Rc, cell::RefCell};

use ego_tree::NodeId;
use lightningcss::{declaration::DeclarationBlock, visitor::{Visitor, VisitTypes, Visit}, rules::CssRule, visit_types, stylesheet::{StyleSheet, ParserOptions}, properties::Property};

use crate::{document::JSXDocument, scraper::Selector};

#[derive(Debug)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>
}

pub struct StyleVisitor<'i> {
  pub style_record: Rc<RefCell<HashMap<NodeId, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument
}

impl<'i> StyleVisitor<'i> {
  pub fn new(document: &'i JSXDocument, style_record: Rc<RefCell<HashMap<NodeId, Vec<StyleDeclaration<'i>>>>>) -> Self {
    StyleVisitor { style_record, document }
  }
}

impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors: Vec<&str> = selectors_str.split(",").collect();
        for index in 0..selectors.len() {
          let selector = Selector::parse(selectors[index]).unwrap();
          let element_refs = self.document.select(&selector);
          for element_ref in element_refs {
            let id = element_ref.id();
            let mut style_record = self.style_record.borrow_mut();
            let declarations = style_record.entry(id).or_insert(vec![]);
            declarations.push(StyleDeclaration {
              specificity: style.selectors.0.get(index).unwrap().specificity(),
              declaration: style.declarations.clone()
            });
          }
        }
      },
      _ => {}
    }
    Ok(())
  }
}

pub struct StyleParser<'i> {
  pub style_record: Rc<RefCell<HashMap<NodeId, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument) -> Self {
    StyleParser {
      style_record: Rc::new(RefCell::new(HashMap::new())),
      document
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();
    let mut style_visitor = StyleVisitor::new(self.document, Rc::clone(&self.style_record));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  pub fn calc(&self) -> HashMap<NodeId, Vec<StyleDeclaration<'i>>> {
    // 遍历 style_record，计算每个节点的最终样式
    let mut style_record = self.style_record.borrow_mut();
    let mut final_style_record: HashMap<NodeId, Vec<StyleDeclaration<'i>>> = HashMap::new();
    for (id, declarations) in style_record.iter_mut() {
      declarations.sort_by(|a, b| a.specificity.cmp(&b.specificity));
      let mut final_properties: Vec<Property<'i>> = Vec::new();
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let declarations = &declaration.declarations;
        for declaration in declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      for declaration in declarations.iter() {
        let declaration = &declaration.declaration;
        let important_declarations = &declaration.important_declarations;
        for declaration in important_declarations.iter() {
          let has_property_index = final_properties
            .iter()
            .position(|property| property.property_id() == declaration.property_id());
          if let Some(index) = has_property_index {
            final_properties[index] = declaration.clone();
          } else {
            final_properties.push(declaration.clone());
          }
        }
      }
      final_style_record.insert(*id, vec![StyleDeclaration {
        specificity: 0,
        declaration: DeclarationBlock {
          declarations: final_properties,
          important_declarations: vec![]
        }
      }]);
    }
    final_style_record
  }
}
