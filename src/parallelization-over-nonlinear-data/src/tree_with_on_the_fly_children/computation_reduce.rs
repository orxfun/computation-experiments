use crate::{
    run_utils::run,
    tree_with_on_the_fly_children::{
        load_status::{NodeStatusPar, NodeStatusSeq},
        node::Node,
        node_storage::NodesStorage,
    },
};
use orx_imp_vec::{ImpVec, PinnedVec};
use orx_parallel::{IntoParIterRec, IntoParIterRecExact, ParIter};

// all

pub fn run_all(storage: &NodesStorage, roots: &[&Node]) {
    println!("\n\n# IMMUTABLE COLLECTION");
    let log = |sum: u64| println!("  sum = {sum}");

    run("sequential", || sequential(storage, roots), log);
    run("orx_rec_exact", || orx_rec_exact(storage, roots), log);
    // run("orx_rec_chunk", || orx_rec_chunk(storage, roots, 1024), log);
    // run(
    //     "orx_rec_into_eager",
    //     || orx_rec_into_eager(storage, roots),
    //     log,
    // );

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

fn extend<'a, 'b>(
    storage: &'b NodesStorage,
    status: &'a NodeStatusPar,
    node: &'a &'a Node,
) -> Vec<&'b Node> {
    let mut children = vec![];
    for s in &node.symbols_out {
        let child = storage.get_relevant_node(s);
        match status.load_child(child) {
            false => continue,
            true => children.push(child),
        }
    }
    children
}

pub fn orx_rec_exact<'a>(storage: &'a NodesStorage, roots: &'a [&'a Node]) -> u64 {
    let status = NodeStatusPar::new(storage.all_nodes.len(), roots);

    let extend = |node: &&Node| extend(storage, &status, node);

    roots
        .iter()
        .copied()
        .into_par_rec_exact(extend, storage.all_nodes.len())
        .map(|x| x.compute())
        .sum()
}

pub fn orx_rec_chunk<'a>(
    storage: &'a NodesStorage,
    roots: &'a [&'a Node],
    chunk_size: usize,
) -> u64 {
    let status = NodeStatusPar::new(storage.all_nodes.len(), roots);

    let extend = |node: &&Node| extend(storage, &status, node);

    roots
        .iter()
        .copied()
        .into_par_rec(extend)
        .chunk_size(chunk_size)
        .map(|x| x.compute())
        .sum()
}

pub fn orx_rec_into_eager<'a>(storage: &'a NodesStorage, roots: &'a [&'a Node]) -> u64 {
    let status = NodeStatusPar::new(storage.all_nodes.len(), roots);

    let extend = |node: &&Node| extend(storage, &status, node);

    roots
        .iter()
        .copied()
        .into_par_rec(extend)
        .into_eager()
        .map(|x| x.compute())
        .sum()
}
