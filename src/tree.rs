use std::fmt;
use std::iter;

use pgn_reader::Color;

use crate::chess_move::Move;

// TODO: use struct wrapper for improved type safety
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

    fn get_child(&self, val: &T, parent: NodeIndex) -> Option<NodeIndex> {
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

    fn depth(&self, idx: NodeIndex) -> usize {
        match self.arena[idx].parent {
            Some(p) => 1 + self.depth(p),
            None => 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    fn is_root(&self, idx: NodeIndex) -> bool {
        idx == 0
    }

    fn is_internal(&self, idx: NodeIndex) -> bool {
        !self.is_root(idx) && !self.is_leaf(idx)
    }

    fn is_leaf(&self, idx: NodeIndex) -> bool {
        self.arena[idx].children.is_empty()
    }
}

impl Tree<Move> {
    pub fn inc_frequency(&mut self, idx: NodeIndex) {
        self.arena[idx].val.inc_frequency()
    }

    pub fn prune(&mut self, color: Color) {
        let freqs: Vec<_> = (0..self.arena.len())
            .map(|i| self.arena[i].val.frequency())
            .collect();
        for node in &mut self.arena {
            if color == node.val.color {
                continue;
            }

            let pruned_children: Vec<_> = node
                .children
                .iter()
                .max_by_key(|&&i| freqs[i])
                .iter()
                .map(|&&i| i)
                .collect();

            node.children = pruned_children;
        }
    }

    pub fn pgn(&self, color: Color, inode_max_depth: usize) -> String {
        self.paths(color, inode_max_depth)
            .iter()
            .map(|p| {
                format!(
                    "[Event \"{}\"]\n{}\n",
                    Self::title_from_path(p, color, inode_max_depth),
                    Self::pgn_from_path(p)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn paths(&self, color: Color, inode_max_depth: usize) -> Vec<Vec<&Move>> {
        if self.is_empty() {
            vec![]
        } else {
            self.paths_rec(color, inode_max_depth, 0, &[])
        }
    }

    fn paths_rec<'a>(
        &'a self,
        color: Color,
        inode_max_depth: usize,
        idx: NodeIndex,
        prefix: &[&'a Move],
    ) -> Vec<Vec<&'a Move>> {
        let node = &self.arena[idx];
        let val = &node.val;
        let new_prefix: Vec<&Move> = prefix.iter().map(|&p| p).chain(iter::once(val)).collect();

        let mut paths = if node.val.color == color
            && (self.is_internal(idx) && self.depth(idx) < inode_max_depth || self.is_leaf(idx))
        {
            vec![new_prefix.clone()]
        } else {
            vec![]
        };
        paths.extend(
            node.children
                .iter()
                .flat_map(|&c| self.paths_rec(color, inode_max_depth, c, &new_prefix))
                .collect::<Vec<_>>(),
        );
        paths
    }

    fn title_from_path(path: &[&Move], color: Color, inode_max_depth: usize) -> String {
        path.iter()
            .enumerate()
            .take(inode_max_depth - 1)
            .filter(|(_, x)| x.color != color)
            .last()
            .map(|(i, x)| format!("{}{}{}", i / 2 + 1, x.dots(), x.san_plus))
            .unwrap_or(String::from("Variation"))
    }

    fn pgn_from_path(path: &[&Move]) -> String {
        path.iter()
            .map(|&x| x.san_plus.to_string())
            .collect::<Vec<_>>()
            .chunks(2)
            .enumerate()
            .map(|(i, xs)| format!("{}. {}", i + 1, xs.join(" ")))
            .collect::<Vec<_>>()
            .join(" ")
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
