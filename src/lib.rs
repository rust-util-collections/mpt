//
// #![deny(warnings)]
//
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::new_without_default)]

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::mem;

type NodeHash = Vec<u8>;

type Key = Vec<u8>;
type Value = Vec<u8>;

fn hashmsg<'a>(data: impl Iterator<Item = &'a [u8]>) -> NodeHash {
    todo!()
}

pub struct MPT {
    root_hash: NodeHash,
    root_node: Node,
    cache: BTreeMap<Key, Value>,
}

impl MPT {
    pub fn new() -> Self {
        let n = Node::Null;
        Self {
            root_hash: n.hash_owned(),
            root_node: n,
            cache: BTreeMap::new(),
        }
    }

    pub fn from_node(n: Node) -> Self {
        Self {
            root_hash: n.hash_owned(),
            root_node: n,
            cache: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k: Key, v: Value) {
        self.cache(k, v)
    }

    fn cache(&mut self, k: Key, v: Value) {
        self.cache.insert(k, v);
    }

    pub fn get(&self, k: &Key) -> Option<&Value> {
        if let Some(v) = self.cache.get(k) {
            return Some(v);
        }

        todo!()
    }

    fn get_routes(&self, k: &Key) -> Option<Vec<NodeHash>> {
        todo!()
    }

    // The most important function.
    fn insert_to_tree(&mut self, k: Key, v: Value) {
        todo!()
    }

    pub fn commit(&mut self) -> &NodeHash {
        let chgs = mem::take(&mut self.cache);
        for (k, v) in chgs.into_iter() {
            self.insert_to_tree(k, v);
        }
        self.set_root();
        &self.root_hash
    }

    fn set_root(&mut self) {
        self.root_hash = self.hash();
    }

    fn hash(&self) -> NodeHash {
        self.root_node.hash_owned()
    }
}

pub struct ExtNode {
    // zipped prefixes
    path: Vec<u8>,
    // The hash of the child node
    child: NodeHash,
}

impl ExtNode {
    fn hash(&self) -> Cow<NodeHash> {
        Cow::Borrowed(&self.child)
    }
}

pub struct BranchNode {
    // A single prefix ==> NodeHash
    prefix_map: BTreeMap<u8, NodeHash>,
}

impl BranchNode {
    fn hash(&self) -> Cow<NodeHash> {
        let msg = self.prefix_map.values().map(|h| h.as_ref());
        Cow::Owned(hashmsg(msg))
    }
}

pub struct LeafNode {
    // zipped prefixes
    path: Vec<u8>,
    value: Value,
}

impl LeafNode {
    fn hash(&self) -> Cow<NodeHash> {
        Cow::Owned(hashmsg([self.value.as_slice()].into_iter()))
    }
}

pub enum Node {
    Ext(ExtNode),
    Leaf(LeafNode),
    Branch(BranchNode),
    Null,
}

impl Node {
    fn hash(&self) -> Cow<NodeHash> {
        match self {
            Node::Ext(n) => n.hash(),
            Node::Leaf(n) => n.hash(),
            Node::Branch(n) => n.hash(),
            Node::Null => Cow::Owned(hashmsg([].into_iter())),
        }
    }

    fn hash_owned(&self) -> NodeHash {
        self.hash().into_owned()
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
