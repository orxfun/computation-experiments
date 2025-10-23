use crate::tree_with_on_the_fly_children::node::Node;
use std::sync::atomic::{AtomicBool, Ordering};

pub enum NodeState {
    NotLoaded,
    Loaded,
    Processed,
}

pub struct NodeStatus {
    already_loaded: Vec<AtomicBool>,
}

impl NodeStatus {
    pub fn new(len: usize) -> Self {
        Self {
            already_loaded: (0..len).map(|_| false.into()).collect(),
        }
    }

    pub fn node_state(&self, idx: usize) -> NodeState {
        match self.already_loaded[idx]
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            true => NodeState::NotLoaded,
            false => NodeState::Loaded,
        }
    }
}

pub struct NodeStatusSeq {
    loaded: Vec<bool>,
    processed: Vec<bool>,
}

impl NodeStatusSeq {
    pub fn new(len: usize, roots: &[&Node]) -> Self {
        let loaded = (0..len)
            .map(|idx| roots.iter().any(|x| x.id == idx))
            .collect();

        Self {
            loaded,
            processed: (0..len).map(|_| false).collect(),
        }
    }

    pub fn node_state(&mut self, idx: usize) -> NodeState {
        let is_loaded = self.loaded.get_mut(idx).unwrap();
        match *is_loaded {
            true => NodeState::Loaded,
            false => {
                *is_loaded = true;
                NodeState::NotLoaded
            }
        }
    }

    pub fn start_processing(&mut self, node: &Node) -> bool {
        let processed = self.processed.get_mut(node.id).unwrap();
        match *processed {
            true => false,
            false => {
                *processed = true;
                true
            }
        }
    }

    pub fn load_child(&mut self, child: &Node) -> bool {
        match self.processed[child.id] {
            true => false,
            false => {
                let loaded = self.loaded.get_mut(child.id).unwrap();
                match loaded {
                    true => false,
                    false => {
                        *loaded = true;
                        true
                    }
                }
            }
        }
    }
}
