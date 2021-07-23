use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node<T>
where
    T: PartialEq + Copy,
{
    pub neighbors: Option<HashSet<T>>, // keys in hash set are used to access nodes in belonging NodeStorage
    pub centrality: Option<f64>,
}

impl<T> Node<T>
where
    T: PartialEq + Copy + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            neighbors: None,
            centrality: None,
        }
    }
    pub fn add_neighbor(&mut self, neighbor: T) {
        match &mut self.neighbors {
            Some(n) => {
                n.insert(neighbor);
            }
            None => {
                let mut new_neighbors = HashSet::<T>::new();
                new_neighbors.insert(neighbor);
                self.neighbors = Some(new_neighbors)
            }
        }
    }
}

// wrapper around node
// Rc for shared ownership (e.g. multiple edges might reference the same node)
// RefCell inside of smart pointer introduces mutability
// see https://doc.rust-lang.org/std/rc/index.html (accessed 23-7-21)
pub type NodeType<T> = Rc<RefCell<Node<T>>>;

// collection of all nodes; key is node identifier
pub struct NodeStore<T>
where
    T: PartialEq + Copy + Debug,
{
    nodes: HashMap<T, NodeType<T>>,
}

impl<T> NodeStore<T>
where
    T: Eq + Hash + PartialEq + Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            nodes: HashMap::<T, NodeType<T>>::new(),
        }
    }
    fn get(&self, node_id: T) -> Option<NodeType<T>> {
        match self.nodes.get(&node_id) {
            None => None,
            Some(node) => Some(Rc::clone(node)), // new pointer to node
        }
    }
    fn insert(&mut self, node_id: T) -> NodeType<T> {
        let new_node: NodeType<T> = Rc::new(RefCell::new(Node::<T>::new()));
        self.nodes.insert(node_id, new_node);
        self.get(node_id).unwrap() // by this time node is certainly in NodeStorage
    }
    pub fn get_node(&mut self, node_id: T) -> NodeType<T> {
        match self.get(node_id) {
            None => self.insert(node_id),
            Some(n) => n,
        }
    }
    pub fn add_neighbor(&mut self, node_id: T, neighbor: T) {
        let node: NodeType<T> = self.get(node_id).unwrap(); // panics if node node in NodeStorage
        let mut node_mut: RefMut<_> = node.borrow_mut();
        node_mut.add_neighbor(neighbor)
    }
    pub fn get_neighbors(&self, node_id: T) -> Option<HashSet<T>> {
        let nt: NodeType<T> = self.get(node_id).unwrap(); // panics if node node in NodeStorage
        let node = nt.borrow();
        node.clone().neighbors
    }
}
