use crate::tree_with_on_the_fly_children::node_storage::NodesStorage;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

mod computation_reduce;
mod load_status;
mod node;
mod node_storage;

pub fn run(seed: u64) {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    let storage = NodesStorage::new(4, &mut rng);
    let roots = storage.get_roots(2, &mut rng);

    dbg!(&storage, &roots);

    computation_reduce::run_all(&storage, &roots);

    // for n in &storage.all_nodes {
    //     println!("\nnode {}", n.id);
    //     for s in &n.symbols_out {
    //         let m = storage.get_relevant_node(s);
    //         println!("{} - {}", m.id, s);
    //     }
    // }
}
