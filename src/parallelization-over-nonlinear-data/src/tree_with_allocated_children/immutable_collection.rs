use super::data::Node;
use crate::run_utils::run;
use orx_parallel::*;

// setup

pub struct FibN<'a> {
    node: &'a Node,
    fib_n: Vec<u64>,
}

impl<'a> FibN<'a> {
    pub fn compute(node: &'a Node) -> Self {
        Self {
            node,
            fib_n: node.value.iter().map(|x| Node::compute(*x)).collect(),
        }
    }
}

// all

pub fn run_all(roots: &[Node]) {
    println!("\n\n# IMMUTABLE COLLECTION");
    let log = |vec: Vec<FibN>| {
        let fib_n_of_root0 = vec
            .iter()
            .find(|x| x.node as *const Node == &roots[0] as *const Node)
            .map(|x| &x.fib_n)
            .unwrap();
        println!(
            "  fib-n of root 0: {:?}",
            fib_n_of_root0.iter().take(15).collect::<Vec<_>>()
        )
    };

    run("sequential", || sequential(roots), log);
    run("orx_rec_exact", || orx_rec_exact(roots), log);
    run("orx_rec_1024", || orx_rec_1024(roots, 1024), log);
    run("orx_rec_into_eager", || orx_rec_into_eager(roots), log);

    println!();
}

// seq

fn seq_compute_node<'a>(node: &'a Node, collected: &mut Vec<FibN<'a>>) {
    collected.push(FibN::compute(node));
    for child in &node.children {
        seq_compute_node(child, collected);
    }
}

pub fn sequential<'a>(roots: &'a [Node]) -> Vec<FibN<'a>> {
    let mut collected = vec![];
    for root in roots {
        seq_compute_node(root, &mut collected);
    }

    collected
}

// orx-parallel

fn extend<'a, 'b>(node: &&'a Node) -> &'b [Node]
where
    'a: 'b,
{
    &node.children
}

pub fn orx_rec_exact<'a>(roots: &'a [Node]) -> Vec<FibN<'a>> {
    let num_nodes = roots.iter().map(|x| x.num_nodes()).sum();
    roots
        .into_par_rec_exact(extend, num_nodes)
        .map(FibN::compute)
        .collect()
}

pub fn orx_rec_1024<'a>(roots: &'a [Node], chunk_size: usize) -> Vec<FibN<'a>> {
    roots
        .into_par_rec(extend)
        .chunk_size(chunk_size)
        .map(FibN::compute)
        .collect()
}

pub fn orx_rec_into_eager<'a>(roots: &'a [Node]) -> Vec<FibN<'a>> {
    roots
        .into_par_rec(extend)
        .into_eager()
        .map(FibN::compute)
        .collect()
}
