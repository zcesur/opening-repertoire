use std::fmt;

pub struct Tree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn get_child_or_insert(&mut self, val: T, parent: NodeIndex) -> NodeIndex {
        match self.get_child(&val, parent) {
            Some(idx) => idx,
            None => self.insert_child(val, parent),
        }
    }

    pub fn get_root_or_insert(&mut self, val: T) -> NodeIndex {
        match self.get_root() {
            Some(idx) => idx,
            None => self.insert_node(val),
        }
    }

    fn get_child(&mut self, val: &T, parent: NodeIndex) -> Option<NodeIndex> {
        self.arena
            .iter()
            .find(|node| &node.val == val && node.parent == Some(parent))
            .map(|node| node.idx)
    }

    fn get_root(&self) -> Option<NodeIndex> {
        self.arena.get(0).map(|node| node.idx)
    }

    fn insert_child(&mut self, val: T, parent: NodeIndex) -> NodeIndex {
        let idx = self.insert_node(val);
        self.arena[parent].children.push(idx);
        self.arena[idx].parent = Some(parent);
        idx
    }

    fn insert_node(&mut self, val: T) -> NodeIndex {
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }
}

impl<T> Tree<T>
where
    T: PartialEq + std::fmt::Display,
{
    fn fmt_rec(
        &self,
        idx: &NodeIndex,
        f: &mut fmt::Formatter<'_>,
        indent: &str,
        is_last: bool,
    ) -> fmt::Result {
        let node = &self.arena[*idx];
        let new_indent = indent.to_owned() + if is_last { "  " } else { "|  " };

        writeln!(f, "{}+- {}", indent, node.val).and_then(|_| match node.children.split_last() {
            Some((last, init)) => init
                .iter()
                .try_fold((), |_, c| self.fmt_rec(c, f, &new_indent, false))
                .and_then(|_| self.fmt_rec(last, f, &new_indent, true)),
            None => Ok(()),
        })
    }
}

impl<T> fmt::Display for Tree<T>
where
    T: PartialEq + std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_root() {
            None => Ok(()),
            Some(idx) => self.fmt_rec(&idx, f, "", true),
        }
    }
}

pub type NodeIndex = usize;

pub struct Node<T>
where
    T: PartialEq,
{
    idx: NodeIndex,
    val: T,
    parent: Option<NodeIndex>,
    children: Vec<NodeIndex>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: NodeIndex, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}
