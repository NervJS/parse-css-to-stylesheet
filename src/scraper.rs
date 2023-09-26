// inpired by https://github.com/causal-agent/scraper

use core::fmt;
use std::{cell::OnceCell, collections::HashMap, ops::Deref, slice::Iter as SliceIter, collections::hash_map::Iter as HashMapIter, fmt::Display, error::Error};

use cssparser::{serialize_string, ToCss, Token, ParseError, ParseErrorKind, BasicParseErrorKind, Parser as CSSParser, ParserInput};
use ego_tree::NodeRef;
use html5ever::{QualName, tendril::{StrTendril, fmt::imp}, LocalName, Namespace, ns, namespace_url, Attribute};
use selectors::{SelectorImpl, parser::{self, SelectorParseErrorKind}, Element as SelectorElement, OpaqueElement, attr::{NamespaceConstraint, AttrSelectorOperation, CaseSensitivity}, matching::{self}, SelectorList};
use smallvec::SmallVec;

use crate::utils::create_qualname;

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
    self.attrs.get(&&create_qualname(attr)).map(Deref::deref)
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
pub struct Fragment {
  pub name: Option<QualName>
}

impl Fragment {
  pub fn new(name: Option<QualName>) -> Self {
    Fragment { name }
  }
}

impl fmt::Debug for Fragment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(name) = &self.name {
      write!(f, "<Fragment name={:?}>", name)
    } else {
      write!(f, "<Fragment>")
    }
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
  Fragment(Fragment),
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

  pub fn is_fragment(&self) -> bool {
    matches!(*self, Node::Fragment(_))
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
      Node::Fragment(ref F) => write!(f, "Fragment({:?})", F),
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

#[derive(Debug, Clone)]
pub enum SelectorErrorKind<'a> {
  UnexpectedToken(Token<'a>),
  EndOfLine,
  InvalidAtRule(String),
  InvalidAtRuleBody,
  QualRuleInvalid,
  ExpectedColonOnPseudoElement(Token<'a>),
  ExpectedIdentityOnPseudoElement(Token<'a>),
  UnexpectedSelectorParseError(SelectorParseErrorKind<'a>)
}

impl<'a> From<ParseError<'a, SelectorParseErrorKind<'a>>> for SelectorErrorKind<'a> {
  fn from(value: ParseError<'a, SelectorParseErrorKind<'a>>) -> Self {
    match value.kind {
      ParseErrorKind::Basic(err) => SelectorErrorKind::from(err),
      ParseErrorKind::Custom(err) => SelectorErrorKind::from(err)
    }
  }
}

impl<'a> From<BasicParseErrorKind<'a>> for SelectorErrorKind<'a> {
  fn from(value: BasicParseErrorKind<'a>) -> Self {
    match value {
      BasicParseErrorKind::UnexpectedToken(token) => Self::UnexpectedToken(token),
      BasicParseErrorKind::EndOfInput => Self::EndOfLine,
      BasicParseErrorKind::AtRuleInvalid(name) => Self::InvalidAtRule(name.to_string()),
      BasicParseErrorKind::AtRuleBodyInvalid => Self::InvalidAtRuleBody,
      BasicParseErrorKind::QualifiedRuleInvalid => Self::QualRuleInvalid,
    }
  }
}

impl<'a> From<SelectorParseErrorKind<'a>> for SelectorErrorKind<'a> {
  fn from(value: SelectorParseErrorKind<'a>) -> Self {
    match value {
      SelectorParseErrorKind::PseudoElementExpectedColon(token) => Self::ExpectedColonOnPseudoElement(token),
      SelectorParseErrorKind::PseudoElementExpectedIdent(token) => Self::ExpectedIdentityOnPseudoElement(token),
      other => Self::UnexpectedSelectorParseError(other)
    }
  }
}

impl<'a> Display for SelectorErrorKind<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::UnexpectedToken(token) => format!("Unexpected token {:?}", render_token(token)),
        Self::EndOfLine => String::from("Unexpected end of line"),
        Self::InvalidAtRule(name) => format!("Invalid @ rule {}", name),
        Self::InvalidAtRuleBody => String::from("Invalid @ rule body"),
        Self::QualRuleInvalid => String::from("Invalid qualified rule"),
        Self::ExpectedColonOnPseudoElement(token) => format!("Expected colon on pseudo-element, found {:?}", render_token(token)),
        Self::ExpectedIdentityOnPseudoElement(token) => format!("Expected identity on pseudo-element, found {:?}", render_token(token)),
        Self::UnexpectedSelectorParseError(err) => format!("Unexpected selector parse error: {:#?}", err)
      }
    )
  }
}

impl<'a> Error for SelectorErrorKind<'a> {
  fn description(&self) -> &str {
    match self {
      Self::UnexpectedToken(_) => "Unexpected token",
      Self::EndOfLine => "Unexpected end of line",
      Self::InvalidAtRule(_) => "Invalid @ rule",
      Self::InvalidAtRuleBody => "Invalid @ rule body",
      Self::QualRuleInvalid => "Invalid qualified rule",
      Self::ExpectedColonOnPseudoElement(_) => "Expected colon on pseudo-element",
      Self::ExpectedIdentityOnPseudoElement(_) => "Expected identity on pseudo-element",
      Self::UnexpectedSelectorParseError(_) => "Unexpected selector parse error"
    }
  }
}

fn render_token(token: &Token<'_>) -> String {
  match token {
    Token::Ident(ident) => format!("{}", ident.clone()),
    Token::AtKeyword(value) => format!("@{}", value.clone()),
    Token::Hash(name) | Token::IDHash(name) => format!("#{}", name.clone()),
    Token::QuotedString(value) => format!("\"{}\"", value.clone()),
    Token::Number {
      has_sign: signed,
      value: num,
      int_value: _,
    }
    | Token::Percentage {
      has_sign: signed,
      unit_value: num,
      int_value: _,
    } => render_number(*signed, *num, token),
    Token::Dimension {
      has_sign: signed,
      value: num,
      int_value: _,
      unit,
    } => format!("{}{}", render_int(*signed, *num), unit),
    Token::WhiteSpace(_) => String::from(" "),
    Token::Comment(comment) => format!("/* {} */", comment),
    Token::Function(name) => format!("{}()", name.clone()),
    Token::BadString(string) => format!("<Bad String {:?}>", string.clone()),
    Token::BadUrl(url) => format!("<Bad URL {:?}>", url.clone()),
    // Single-character token
    sc_token => render_single_char_token(sc_token),
  }
}

fn render_single_char_token(token: &Token) -> String {
  String::from(match token {
      Token::Colon => ":",
      Token::Semicolon => ";",
      Token::Comma => ",",
      Token::IncludeMatch => "~=",
      Token::DashMatch => "|=",
      Token::PrefixMatch => "^=",
      Token::SuffixMatch => "$=",
      Token::SubstringMatch => "*=",
      Token::CDO => "<!--",
      Token::CDC => "-->",
      Token::ParenthesisBlock => "<(",
      Token::SquareBracketBlock => "<[",
      Token::CurlyBracketBlock => "<{",
      Token::CloseParenthesis => "<)",
      Token::CloseSquareBracket => "<]",
      Token::CloseCurlyBracket => "<}",
      other => panic!(
          "Token {:?} is not supposed to match as a single-character token!",
          other
      ),
  })
}

fn render_number(signed: bool, num: f32, token: &Token) -> String {
  let num = render_int(signed, num);

  match token {
    Token::Number { .. } => num,
    Token::Percentage { .. } => format!("{}%", num),
    _ => panic!("render_number is not supposed to be called on a non-numerical token"),
  }
}

fn render_int(signed: bool, num: f32) -> String {
  if signed {
    render_int_signed(num)
  } else {
    render_int_unsigned(num)
  }
}

fn render_int_signed(num: f32) -> String {
  if num > 0.0 {
    format!("+{}", num)
  } else {
    format!("-{}", num)
  }
}

fn render_int_unsigned(num: f32) -> String {
  format!("{}", num)
}

struct SelectorParser;

impl<'i> parser::Parser<'i> for SelectorParser {
  type Impl = SimpleImpl;
  type Error = SelectorParseErrorKind<'i>;
}

pub struct Selector {
  pub selectors: SmallVec<[parser::Selector<SimpleImpl>; 1]>
}

impl Selector {
  pub fn parse(selectors: &'_ str) -> Result<Self, SelectorErrorKind> {
    let mut parser_input = ParserInput::new(selectors);
    let mut parser = CSSParser::new(&mut parser_input);
    SelectorList::parse(&SelectorParser, &mut parser, parser::ParseRelative::No)
      .map(|list| Selector { selectors: list.0 })
      .map_err(SelectorErrorKind::from)
  }

  pub fn matches(&self, element: &ElementRef) -> bool {
    self.matches_with_scope(element, None)
  }

  pub fn matches_with_scope(&self, element: &ElementRef, scope: Option<ElementRef>) -> bool {
    let mut nth_index_cache = Default::default();
    let mut context = matching::MatchingContext::new(
      matching::MatchingMode::Normal,
      None,
      &mut nth_index_cache,
      matching::QuirksMode::Quirks,
      matching::NeedsSelectorFlags::No,
      matching::IgnoreNthChildForInvalidation::No
    );

    context.scope_element = scope.map(|element| selectors::Element::opaque(&element));
    self.selectors
      .iter()
      .any(|s| matching::matches_selector(s, 0, None, element, &mut context))
  }
}
