use crate::tree_with_on_the_fly_children::data::NodesStorage;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod data;

pub fn run(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let nodes = NodesStorage::new(4, &mut rng);
    dbg!(nodes);
}
