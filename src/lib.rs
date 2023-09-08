use core::fmt;
use std::{cell::OnceCell, collections::HashMap, ops::Deref, slice::Iter as SliceIter, collections::hash_map::Iter as HashMapIter};

use ego_tree::NodeRef;
use html5ever::{QualName, tendril::StrTendril, LocalName, ns};


fn create_qualame(str: &str) -> QualName {
  QualName::new(None, ns!(), LocalName::from(str))
}

#[derive(Debug, Clone, Copy)]
pub enum CaseSensitivity {
  CaseSensitive,
  AsciiCaseInsensitive
}

impl CaseSensitivity {
  pub fn eq(self, a: &[u8], b: &[u8]) -> bool {
    match self {
      CaseSensitivity::CaseSensitive => a == b,
      CaseSensitivity::AsciiCaseInsensitive => a.eq_ignore_ascii_case(b)
    }
  }

  pub fn contains(self, haystack: &str, needle: &str) -> bool {
    match self {
      CaseSensitivity::CaseSensitive => haystack.contains(needle),
      CaseSensitivity::AsciiCaseInsensitive => {
        if let Some((&n_first_byte, n_rest)) = needle.as_bytes().split_first() {
          haystack.bytes().enumerate().any(|(i, b)| {
            if !b.eq_ignore_ascii_case(&n_first_byte) {
              return false;
            }
            let after_this_byte = &haystack.as_bytes()[i + i..];
            match after_this_byte.get(..n_rest.len()) {
              Some(haystack_slice) => haystack_slice.eq_ignore_ascii_case(n_rest),
              None => false
            }
          })
        } else {
          true
        }
      }
    }
  }
}

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
    self.inner.next().map(|(k, v)| (k.local.deref(), k.local.deref()))
  }
}

pub struct Attribute {
  pub name: QualName,
  pub value: StrTendril
}

pub type Attributes = HashMap<QualName, StrTendril>;

#[derive(Clone, PartialEq, Eq)]
pub struct Element {
  pub name: QualName,
  pub attrs: Attributes,
  id: OnceCell<Option<StrTendril>>,
  classes: OnceCell<Vec<LocalName>>
}

impl Element {
  pub fn new(name: QualName, attributes: Vec<Attribute>) -> Self {
    let attrs: HashMap<QualName, StrTendril> = attributes
      .into_iter()
      .map(|attr| (attr.name, attr.value))
      .collect();
    Element {
      name,
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

  pub fn attr(&self, attr: &str) -> Option<&str> {
    self.attrs.get(&create_qualame(attr)).map(Deref::deref)
  }

  pub fn attrs(&self) -> Attrs {
    Attrs { inner: self.attrs.iter() }
  }
}

impl fmt::Debug for Element {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<{}", self.name())?;
    for (key, value) in self.attrs() {
      write!(f, "{}={:?}", key, value)?;
    }
    write!(f, ">")
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Comment {
  pub comment: StrTendril
}

impl Deref for Comment {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    self.comment.deref()
  }
}

impl fmt::Debug for Comment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<!-- {:?} -->", self.deref())
  }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
  pub text: StrTendril
}

impl Deref for Text {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.text.deref()
  }
}

impl fmt::Debug for Text {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.deref())
  }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Node {
  Element(Element),
  Comment(Comment),
  Text(Text)
}

impl Node {
  pub fn is_text(&self) -> bool {
    matches!(*self, Node::Text(_))
  }

  pub fn is_element(&self) -> bool {
    matches!(*self, Node::Element(_))
  }

  pub fn is_comment(&self) -> bool {
    matches!(*self, Node::Comment(_))
  }

  pub fn as_text(&self) -> Option<&Text> {
    match *self {
      Node::Text(ref t) => Some(t),
      _ => None
    }
  }

  pub fn as_element(&self) -> Option<&Element> {
    match *self {
      Node::Element(ref e) => Some(e),
      _ => None
    }
  }

  pub fn as_comment(&self) -> Option<&Comment> {
    match *self {
      Node::Comment(ref c) => Some(c),
      _ => None
    }
  }
}

impl fmt::Debug for Node {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Node::Comment(ref c) => write!(f, "Comment({:?})", c),
      Node::Element(ref e) => write!(f, "Element({:?})", e),
      Node::Text(ref t) => write!(f, "Text({:?})", t)
    }
  }
}

pub struct ElementRef<'a> {
  node: NodeRef<'a, Node>
}

impl<'a> ElementRef<'a> {
  pub fn new(node: NodeRef<'a, Node>) -> Self {
    ElementRef { node }
  }

  pub fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
    if node.value().is_element() {
      Some(ElementRef::new(node))
    } else {
      None
    }
  }

  pub fn value(&self) -> &'a Element {
    self.node.value().as_element().unwrap()
  }

}
