use std::collections::HashMap;

//'stream lifetime indicates that reference exists as long as the belonging GraphStream object exists

pub struct NodeStorage<'stream> {
    nodes: HashMap<&'stream str, usize>,
    current_id: usize,
}

impl<'stream> NodeStorage<'stream> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::<&'stream str, usize>::new(),
            current_id: 0,
        }
    }
    pub fn get_id_or_insert(&mut self, node: &'stream str) -> usize {
        match self.nodes.get(node) {
            Some(v) => *v,
            None => {
                let id = self.nodes.insert(node, self.current_id).unwrap();
                self.current_id += 1;
                id
            }
        }
    }
}
