// inpired by https://github.com/causal-agent/scraper

use core::fmt;
use std::{cell::OnceCell, collections::HashMap, ops::Deref, slice::Iter as SliceIter, collections::hash_map::Iter as HashMapIter};

use html5ever::{QualName, tendril::StrTendril, LocalName, Attribute};
use selectors::attr::CaseSensitivity;

use crate::visitor::SpanKey;

pub struct Classes<'a> {
  inner: SliceIter<'a, LocalName>
}

impl<'a> Iterator for Classes<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(Deref::deref)
  }
}

pub type AttributesIter<'a> = HashMapIter<'a, QualName, StrTendril>;

#[derive(Debug, Clone)]
pub struct Attrs<'a> {
  inner: AttributesIter<'a>
}

impl<'a> Iterator for Attrs<'a> {
  type Item = (&'a str, &'a str);
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(|(k, v)| (k.local.deref(), v.deref()))
  }
}

pub type Attributes = HashMap<QualName, StrTendril>;

#[derive(Clone, PartialEq, Eq)]
pub struct Element {
  pub name: QualName,
  pub attrs: Attributes,
  pub span: SpanKey,
  id: OnceCell<Option<StrTendril>>,
  classes: OnceCell<Vec<LocalName>>
}

impl Element {
  pub fn new(name: QualName, span: SpanKey, attributes: Vec<Attribute>) -> Self {
    let attrs: HashMap<QualName, StrTendril> = attributes
      .into_iter()
      .map(|attr| (attr.name, attr.value))
      .collect();
    Element {
      name,
      span,
      attrs,
      id: OnceCell::new(),
      classes: OnceCell::new()
    }
  }

  pub fn name(&self) -> &str {
    self.name.local.deref()
  }

  pub fn id(&self) -> Option<&str> {
    self.id.get_or_init(|| {
      self.attrs
        .iter()
        .find(|(name,_ )| name.local.as_ref() == "id")
        .map(|(_, value)| value.clone())
    }).as_deref()
  }

  pub fn has_class(&self, class: &str, case_sensitive: CaseSensitivity) -> bool {
    self.classes()
      .any(|class_name| case_sensitive.eq(class.as_bytes(), class_name.as_bytes()))
  }

  pub fn classes(&self) -> Classes {
    let classes = self.classes.get_or_init(|| {
      let mut classes: Vec<LocalName> = self.attrs
        .iter()
        .filter(|(name, _)| name.local.as_ref() == "className")
        .flat_map(|(_, value)| value.split_whitespace().map(LocalName::from))
        .collect();
      classes.sort_unstable();
      classes.dedup();
      classes
    });

    Classes { inner: classes.iter() }
  }

  pub fn attrs(&self) -> Attrs {
    Attrs { inner: self.attrs.iter() }
  }
}

impl fmt::Debug for Element {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<{}", self.name())?;
    for (key, value) in self.attrs() {
      write!(f, " {}={:?}", key, value)?;
    }
    write!(f, ">")
  }
}
