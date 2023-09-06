use std::num::NonZeroUsize;

pub struct NodeId(NonZeroUsize);

impl NodeId {
  fn to_index(self) -> usize {
    self.0.get()
  }
  unsafe fn from_index(index: usize) -> Self {
    Self(NonZeroUsize::new_unchecked(index))
  }
}

struct Node<T> {
  parent: Option<NodeId>,
  prev_sibling: Option<NodeId>,
  next_sibling: Option<NodeId>,
  chidlren: Option<(NodeId, NodeId)>,
  value: T
}

impl<T> Node<T> {
  fn new(value: T) -> Self {
    Self {
      parent: None,
      prev_sibling: None,
      next_sibling: None,
      chidlren: None,
      value
    }
  }
}

struct NodeRef {

}
