use std::{cell::RefCell, collections::HashMap, convert::Infallible, hash::Hash, rc::Rc};

use lightningcss::{
  declaration::DeclarationBlock,
  properties::{Property, PropertyId},
  rules::CssRule,
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Features, Targets},
  traits::ToCss,
  visit_types,
  visitor::{Visit, VisitTypes, Visitor},
};

use crate::{document::JSXDocument, utils::to_camel_case, visitor::SpanKey};

struct TextDecoration {
  pub index: Option<usize>,
  pub value: Option<String>,
}

impl TextDecoration {
  pub fn new() -> Self {
    TextDecoration {
      index: None,
      value: None,
    }
  }

  pub fn set_index(&mut self, index: usize) {
    self.index = Some(index);
  }

  pub fn set_value(&mut self, value: String) {
    self.value = Some(value);
  }
}

pub type StyleValue = HashMap<String, String>;

pub struct StyleData {
  pub style_record: Rc<RefCell<HashMap<SpanKey, StyleValue>>>,
  pub all_style: Rc<RefCell<HashMap<String, StyleValue>>>,
}

#[derive(Debug, Clone)]
pub struct StyleDeclaration<'i> {
  pub specificity: u32,
  pub declaration: DeclarationBlock<'i>,
}

pub struct StyleVisitor<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleVisitor<'i> {
  pub fn new(
    document: &'i JSXDocument,
    all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  ) -> Self {
    StyleVisitor {
      all_style,
      document,
    }
  }
}

impl<'i> Visitor<'i> for StyleVisitor<'i> {
  type Error = Infallible;
  const TYPES: VisitTypes = visit_types!(RULES);
  fn visit_rule(&mut self, rule: &mut CssRule<'i>) -> Result<(), Self::Error> {
    match rule {
      CssRule::Style(style) => {
        let selectors_str = style.selectors.to_string();
        let selectors = selectors_str.split(",").collect::<Vec<&str>>();
        for index in 0..selectors.len() {
          let selector = selectors[index].trim().replace(".", "");
          let mut all_style = self.all_style.borrow_mut();
          let declarations: &mut Vec<StyleDeclaration<'_>> =
            all_style.entry(selector.clone()).or_insert(vec![]);
          declarations.push(StyleDeclaration {
            specificity: style.selectors.0.get(index).unwrap().specificity(),
            declaration: style.declarations.clone(),
          });
        }
      }
      _ => {}
    }
    Ok(())
  }
}

pub struct StyleParser<'i> {
  pub all_style: Rc<RefCell<HashMap<String, Vec<StyleDeclaration<'i>>>>>,
  pub document: &'i JSXDocument,
}

impl<'i> StyleParser<'i> {
  pub fn new(document: &'i JSXDocument) -> Self {
    StyleParser {
      all_style: Rc::new(RefCell::new(HashMap::new())),
      document,
    }
  }

  pub fn parse(&mut self, css: &'i str) {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).expect("解析样式失败");
    let mut style_visitor = StyleVisitor::new(self.document, Rc::clone(&self.all_style));
    stylesheet.visit(&mut style_visitor).unwrap();
  }

  fn calc_style_record<T: Hash + Eq + Clone>(
    &self,
    style_record: &mut HashMap<T, Vec<StyleDeclaration<'i>>>,
  ) -> HashMap<T, StyleDeclaration<'i>> {
    let mut final_style_record = HashMap::new();
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
      final_style_record.insert(
        (*id).clone(),
        StyleDeclaration {
          specificity: 0,
          declaration: DeclarationBlock {
            declarations: final_properties,
            important_declarations: vec![],
          },
        },
      );
    }
    final_style_record
  }

  fn parse_style_value(&self, style_value: &mut StyleDeclaration<'i>) {
    let properties = &mut style_value.declaration.declarations;
    let mut text_decoration = TextDecoration::new();
    let mut color = None;
    for (index, property) in properties.iter_mut().enumerate() {
      if property.property_id() == PropertyId::from("text-decoration") {
        text_decoration.set_index(index);
        text_decoration.set_value(
          property
            .value_to_css_string(PrinterOptions {
              minify: false,
              targets: Targets {
                include: Features::HexAlphaColors,
                ..Targets::default()
              },
              ..PrinterOptions::default()
            })
            .unwrap(),
        );
      } else if property.property_id() == PropertyId::from("color") {
        color = Some(
          property
            .value_to_css_string(PrinterOptions {
              minify: false,
              targets: Targets {
                include: Features::HexAlphaColors,
                ..Targets::default()
              },
              ..PrinterOptions::default()
            })
            .unwrap(),
        );
      }
    }

    if text_decoration.index.is_some() {}
  }

  pub fn calc(&self) -> StyleData {
    // 遍历 style_record，计算每个节点的最终样式
    let mut all_style = self.all_style.borrow_mut();
    let mut style_record = HashMap::new();
    let mut final_all_style = self.calc_style_record(&mut all_style);

    let mut final_all_style = final_all_style
      .iter()
      .map(|(selector, style_value)| {
        (
          selector.to_owned(),
          style_value
            .declaration
            .declarations
            .iter()
            .map(|property| {
              (
                to_camel_case(
                  property
                    .property_id()
                    .to_css_string(PrinterOptions::default())
                    .unwrap()
                    .as_str(),
                ),
                property
                  .value_to_css_string(PrinterOptions {
                    minify: false,
                    targets: Targets {
                      include: Features::HexAlphaColors,
                      ..Targets::default()
                    },
                    ..PrinterOptions::default()
                  })
                  .unwrap(),
              )
            })
            .collect::<HashMap<_, _>>(),
        )
      })
      .collect::<HashMap<_, _>>();

    for (selector, style_value) in final_all_style.iter_mut() {
      // self.parse_style_value(style_value);
      let elements = self.document.select(selector);
      for element in elements {
        let declarations = style_record.entry(element.span).or_insert(vec![]);
        declarations.push(style_value.clone());
      }
    }
    let final_style_record = style_record
      .iter_mut()
      .map(|(selector, style_value)| {
        (
          *selector,
          style_value
            .iter_mut()
            .reduce(|a, b| {
              a.extend(b.drain());
              a
            })
            .unwrap()
            .to_owned(),
        )
      })
      .collect::<HashMap<_, _>>();
    StyleData {
      style_record: Rc::new(RefCell::new(final_style_record)),
      all_style: Rc::new(RefCell::new(final_all_style)),
    }
  }
}
