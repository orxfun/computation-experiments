use crate::{data::Node, run_utils::run};
use orx_parallel::*;
use rayon::iter::*;
use std::sync::atomic::{AtomicU64, Ordering};

// all

pub fn run_all(roots: &[Node]) {
    println!("\n\n# IMMUTABLE COLLECTION");
    let log = |vec: Vec<u64>| {
        println!(
            "  collected {} elements: [{:?}, ...]",
            vec.len(),
            vec.iter().take(10).collect::<Vec<_>>()
        )
    };

    let f = || sequential(roots);
    run("sequential", f, log);

    println!();
}

// seq

fn seq_compute_node(node: &Node, collected: &mut Vec<u64>) {
    collected.extend(node.value.iter().map(|x| Node::compute(*x)));

    for child in &node.children {
        collected.extend(child.value.iter().map(|x| Node::compute(*x)));
    }
}

pub fn sequential(roots: &[Node]) -> Vec<u64> {
    let mut collected = vec![];
    for root in roots {
        seq_compute_node(root, &mut collected);
    }
    collected
}
