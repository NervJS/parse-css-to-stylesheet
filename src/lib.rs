use core::fmt;
use std::{cell::OnceCell, collections::HashMap, ops::Deref, slice::Iter as SliceIter, collections::hash_map::Iter as HashMapIter};

use cssparser::{serialize_string, ToCss};
use ego_tree::NodeRef;
use html5ever::{QualName, tendril::StrTendril, LocalName, Namespace, ns, namespace_url, Attribute};
use selectors::{SelectorImpl, parser::{self}, Element as SelectorElement, OpaqueElement, attr::{NamespaceConstraint, AttrSelectorOperation, CaseSensitivity}, matching};


pub fn create_qualname(str: &str) -> QualName {
  QualName::new(None, ns!(), LocalName::from(str))
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
    self.inner.next().map(|(k, v)| (k.local.deref(), v.deref()))
  }
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
    self.attrs.get(&create_qualname(attr)).map(Deref::deref)
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
  Document,
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

  pub fn is_document(&self) -> bool {
    matches!(*self, Node::Document)
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
      Node::Document => write!(f, "Document"),
      Node::Comment(ref c) => write!(f, "Comment({:?})", c),
      Node::Element(ref e) => write!(f, "Element({:?})", e),
      Node::Text(ref t) => write!(f, "Text({:?})", t)
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CSSString(pub String);

impl<'a> From<&'a str> for CSSString {
  fn from(value: &'a str) -> Self {
    Self(value.to_owned())
  }
}

impl AsRef<str> for CSSString {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl ToCss for CSSString {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
      W: std::fmt::Write {
    serialize_string(&self.0, dest)
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CSSLocalName(pub LocalName);

impl<'a> From<&'a str> for CSSLocalName {
  fn from(value: &'a str) -> Self {
    Self(value.into())
  }
}

impl ToCss for CSSLocalName {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
      W: std::fmt::Write {
    dest.write_str(&self.0)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl parser::NonTSPseudoClass for NonTSPseudoClass {
  type Impl = SimpleImpl;
  fn is_active_or_hover(&self) -> bool {
    false
  }

  fn is_user_action_state(&self) -> bool {
    false
  }
}

impl ToCss for NonTSPseudoClass {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
      W: std::fmt::Write {
    dest.write_str("")
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoElement {}

impl parser::PseudoElement for PseudoElement {
  type Impl = SimpleImpl;
}

impl ToCss for PseudoElement {
  fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
      W: std::fmt::Write {
    dest.write_str("")
  }    
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleImpl;

impl SelectorImpl for SimpleImpl {
  type AttrValue = CSSString;
  type Identifier = CSSLocalName;
  type LocalName = CSSLocalName;
  type NamespacePrefix = CSSLocalName;
  type NamespaceUrl = Namespace;
  type BorrowedNamespaceUrl = Namespace;
  type BorrowedLocalName = CSSLocalName;
  type NonTSPseudoClass = NonTSPseudoClass;
  type PseudoElement = PseudoElement;

  type ExtraMatchingData<'a> = ();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl<'a> Deref for ElementRef<'a> {
  type Target = NodeRef<'a, Node>;
  fn deref(&self) -> &Self::Target {
    &self.node
  }
}

impl<'a> SelectorElement for ElementRef<'a> {
  type Impl = SimpleImpl;

  fn opaque(&self) -> OpaqueElement {
    OpaqueElement::new(&self.node.value())
  }

  fn parent_element(&self) -> Option<Self> {
    self.parent().and_then(ElementRef::wrap)
  }

  fn parent_node_is_shadow_root(&self) -> bool {
    false
  }

  fn containing_shadow_host(&self) -> Option<Self> {
    None
  }

  fn is_pseudo_element(&self) -> bool {
    false
  }

  fn is_part(&self, _name: &CSSLocalName) -> bool {
    false
  }

  fn is_same_type(&self, other: &Self) -> bool {
    self.value().name == other.value().name
  }

  fn imported_part(&self, _: &CSSLocalName) -> Option<CSSLocalName> {
    None
  }

  fn prev_sibling_element(&self) -> Option<Self> {
    self.prev_siblings()
      .find(|sibling| sibling.value().is_element())
      .map(ElementRef::new)
  }

  fn next_sibling_element(&self) -> Option<Self> {
    self.next_siblings()
      .find(|sibling| sibling.value().is_element())
      .map(ElementRef::new)
  }

  fn first_element_child(&self) -> Option<Self> {
    self.children()
      .find(|child| child.value().is_element())
      .map(ElementRef::new)
  }

  fn is_html_element_in_html_document(&self) -> bool {
    self.value().name.ns == ns!(html)
  }

  fn has_local_name(&self, name: &CSSLocalName) -> bool {
    self.value().name.local == name.0
  }

  fn has_namespace(&self, ns: &Namespace) -> bool {
    &self.value().name.ns == ns
  }

  fn attr_matches(
      &self,
      ns: &NamespaceConstraint<&Namespace>,
      local_name: &CSSLocalName,
      operation: &AttrSelectorOperation<&CSSString>,
    ) -> bool {
    self.value().attrs
      .iter()
      .any(|(name, value)| {
        matches!(*ns, NamespaceConstraint::Specific(url) if *url != name.ns)
          && local_name.0 == name.local
          && operation.eval_str(value)
      })
  }

  fn match_non_ts_pseudo_class(
    &self,
    _pc: &NonTSPseudoClass,
    _context: &mut matching::MatchingContext<'_, Self::Impl>,
  ) -> bool {
    false
  }

  fn match_pseudo_element(
    &self,
    _pe: &PseudoElement,
    _context: &mut matching::MatchingContext<Self::Impl>,
  ) -> bool {
    false
  }

  fn is_link(&self) -> bool {
    self.value().name() == "link"
  }

  fn is_html_slot_element(&self) -> bool {
    true
  }

  fn has_id(&self, id: &CSSLocalName, case_sensitivity: CaseSensitivity) -> bool {
    match self.value().id() {
      Some(val) => case_sensitivity.eq(id.0.as_bytes(), val.as_bytes()),
      None => false,
    }
  }

  fn has_class(&self, name: &CSSLocalName, case_sensitivity: CaseSensitivity) -> bool {
    self.value().has_class(&name.0, case_sensitivity)
  }

  fn is_empty(&self) -> bool {
    !self
      .children()
      .any(|child| child.value().is_element() || child.value().is_text())
  }

  fn is_root(&self) -> bool {
    self.parent()
      .map_or(false, |parent| parent.value().is_document())
  }

  fn apply_selector_flags(&self, _flags: matching::ElementSelectorFlags) {}
}
