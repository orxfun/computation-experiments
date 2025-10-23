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
