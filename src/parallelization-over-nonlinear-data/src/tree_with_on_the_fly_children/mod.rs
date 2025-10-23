use crate::tree_with_on_the_fly_children::node_storage::NodesStorage;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod computation_reduce;
mod load_status;
mod node;
mod node_storage;

pub fn run(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let storage = NodesStorage::new(50_000, &mut rng);
    let roots = storage.get_roots(20, &mut rng);

    computation_reduce::run_all(&storage, &roots);
}
