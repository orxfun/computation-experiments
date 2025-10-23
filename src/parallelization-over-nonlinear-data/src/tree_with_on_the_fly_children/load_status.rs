use std::sync::atomic::{AtomicBool, Ordering};

pub enum LoadState {
    AlreadyLoaded,
    NotLoadedYet,
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

    pub fn get_load_state(&self, idx: usize) -> LoadState {
        match self.already_loaded[idx]
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            true => LoadState::NotLoadedYet,
            false => LoadState::AlreadyLoaded,
        }
    }
}

pub struct NodeStatusSeq {
    already_loaded: Vec<bool>,
}

impl NodeStatusSeq {
    pub fn new(len: usize) -> Self {
        Self {
            already_loaded: (0..len).map(|_| false).collect(),
        }
    }

    pub fn get_load_state(&mut self, idx: usize) -> LoadState {
        let is_loaded = self.already_loaded.get_mut(idx).unwrap();
        match *is_loaded {
            true => LoadState::AlreadyLoaded,
            false => {
                *is_loaded = true;
                LoadState::NotLoadedYet
            }
        }
    }
}
