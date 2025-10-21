use crate::{data::Node, run_utils::run};
use orx_parallel::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// all

pub fn run_all(roots: &[Node]) {
    println!("\n\n# IMMUTABLE REDUCTION USING MUTABLE VARIABLE");
    let log = |sum: u64| println!("  sum = {sum}");

    run("sequential", || sequential(roots), log);
    run("orx_rec_exact", || orx_rec_exact(roots), log);
    run("orx_rec_1024", || orx_rec_1024(roots, 1024), log);
    run("orx_rec_into_eager", || orx_rec_into_eager(roots), log);

    println!();
}

// seq

fn seq_compute_node(node: &Node, rng: &mut impl Rng) -> u64 {
    node.value
        .iter()
        .map(|x| Node::compute_using_mut_var(*x, rng))
        .sum::<u64>()
        + node
            .children
            .iter()
            .map(|x| seq_compute_node(x, rng))
            .sum::<u64>()
}

pub fn sequential(roots: &[Node]) -> u64 {
    let mut rng = ChaCha8Rng::seed_from_u64(64);
    roots.iter().map(|x| seq_compute_node(x, &mut rng)).sum()
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
        .using(|thread_idx| ChaCha8Rng::seed_from_u64(64 + thread_idx as u64))
        .map(|rng, x| {
            x.value
                .iter()
                .map(|x| Node::compute_using_mut_var(*x, rng))
                .sum::<u64>()
        })
        .sum()
}

pub fn orx_rec_1024(roots: &[Node], chunk_size: usize) -> u64 {
    roots
        .into_par_rec(extend)
        .using(|thread_idx| ChaCha8Rng::seed_from_u64(64 + thread_idx as u64))
        .chunk_size(chunk_size)
        .map(|rng, x| {
            x.value
                .iter()
                .map(|x| Node::compute_using_mut_var(*x, rng))
                .sum::<u64>()
        })
        .sum()
}

pub fn orx_rec_into_eager(roots: &[Node]) -> u64 {
    roots
        .into_par_rec(extend)
        .into_eager()
        .using(|thread_idx| ChaCha8Rng::seed_from_u64(64 + thread_idx as u64))
        .map(|rng, x| {
            x.value
                .iter()
                .map(|x| Node::compute_using_mut_var(*x, rng))
                .sum::<u64>()
        })
        .sum()
}
