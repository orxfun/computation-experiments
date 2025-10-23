use crate::{
    run_utils::run,
    tree_with_on_the_fly_children::{
        load_status::NodeStatusSeq, node::Node, node_storage::NodesStorage,
    },
};
use orx_imp_vec::{ImpVec, PinnedVec};

// all

pub fn run_all(storage: &NodesStorage, roots: &[&Node]) {
    println!("\n\n# IMMUTABLE COLLECTION");
    let log = |sum: u64| println!("  sum = {sum}");

    run("sequential", || sequential(storage, roots), log);
    // run("orx_rec_exact", || orx_rec_exact(roots), log);
    // run("orx_rec_1024", || orx_rec_1024(roots, 1024), log);
    // run("orx_rec_into_eager", || orx_rec_into_eager(roots), log);

    println!();
}

// seq

pub fn sequential(storage: &NodesStorage, roots: &[&Node]) -> u64 {
    let mut status = NodeStatusSeq::new(storage.all_nodes.len(), roots);

    let tasks: ImpVec<_> = roots.iter().copied().collect();
    let mut sum = 0;

    for i in 0.. {
        match tasks.get(i) {
            None => break,
            Some(node) => {
                match status.start_processing(node) {
                    false => continue,
                    true => {
                        // extend
                        for s in &node.symbols_out {
                            let child = storage.get_relevant_node(s);
                            match status.load_child(child) {
                                false => continue,
                                true => tasks.imp_push(child),
                            }
                        }

                        // process
                        let value = node.compute();
                        sum += value;
                    }
                }
            }
        }
    }

    sum
}

// orx
