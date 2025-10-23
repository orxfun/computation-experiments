use super::data::Node;
use crate::run_utils::run;
use orx_parallel::*;
use rayon::iter::*;
use std::sync::atomic::{AtomicU64, Ordering};

// all

pub fn run_all(roots: &[Node]) {
    println!("\n\n# IMMUTABLE REDUCTION");
    let log = |sum: u64| println!("  sum = {sum}");

    run("sequential", || sequential(roots), log);

    // rayon miri fails with:
    // Undefined Behavior: trying to retag from <84156795> for SharedReadWrite permission at alloc41643328[0x8],
    // but that tag does not exist in the borrow stack for this location
    #[cfg(not(miri))]
    run("rayon", || rayon(roots), log);

    run("orx_rec_exact", || orx_rec_exact(roots), log);
    run(
        "orx_rec_exact_flatmap",
        || orx_rec_exact_flatmap(roots),
        log,
    );
    run("orx_rec_1024", || orx_rec_1024(roots, 1024), log);
    run("orx_rec_into_eager", || orx_rec_into_eager(roots), log);
    run(
        "orx_rec_into_eager_flatmap",
        || orx_rec_into_eager_flatmap(roots),
        log,
    );

    println!();
}

// seq

fn seq_compute_node(node: &Node) -> u64 {
    node.value.iter().map(|x| Node::compute(*x)).sum::<u64>()
        + node
            .children
            .iter()
            .map(|x| seq_compute_node(x))
            .sum::<u64>()
}

pub fn sequential(roots: &[Node]) -> u64 {
    roots.iter().map(|x| seq_compute_node(x)).sum()
}

// rayon

fn process_node<'scope>(sum: &'scope AtomicU64, node: &'scope Node, s: &rayon::Scope<'scope>) {
    for child in &node.children {
        s.spawn(move |s| {
            process_node(sum, child, s);
        });
    }
    let val = node.value.par_iter().map(|x| Node::compute(*x)).sum();
    sum.fetch_add(val, Ordering::Relaxed);
}

pub fn rayon(roots: &[Node]) -> u64 {
    let sum = AtomicU64::new(0);
    rayon::in_place_scope(|s| {
        for root in roots {
            process_node(&sum, root, s);
        }
    });
    sum.into_inner()
}

// orx-parallel

fn extend<'a, 'b>(node: &&'a Node) -> &'b [Node]
where
    'a: 'b,
{
    &node.children
}

pub fn orx_rec_exact(roots: &[Node]) -> u64 {
    let num_nodes = roots.iter().map(|x| x.num_nodes()).sum();
    roots
        .into_par_rec_exact(extend, num_nodes)
        .map(|x| x.value.iter().map(|x| Node::compute(*x)).sum::<u64>())
        .sum()
}

pub fn orx_rec_exact_flatmap(roots: &[Node]) -> u64 {
    let num_nodes = roots.iter().map(|x| x.num_nodes()).sum();
    roots
        .into_par_rec_exact(extend, num_nodes)
        .flat_map(|x| x.value.iter().map(|x| Node::compute(*x)))
        .sum()
}

pub fn orx_rec_1024(roots: &[Node], chunk_size: usize) -> u64 {
    roots
        .into_par_rec(extend)
        .chunk_size(chunk_size)
        .map(|x| x.value.iter().map(|x| Node::compute(*x)).sum::<u64>())
        .sum()
}

pub fn orx_rec_into_eager(roots: &[Node]) -> u64 {
    roots
        .into_par_rec(extend)
        .into_eager()
        .map(|x| x.value.iter().map(|x| Node::compute(*x)).sum::<u64>())
        .sum()
}

pub fn orx_rec_into_eager_flatmap(roots: &[Node]) -> u64 {
    roots
        .into_par_rec(extend)
        .into_eager()
        .flat_map(|x| x.value.iter().map(|x| Node::compute(*x)))
        .sum()
}
