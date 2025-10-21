use crate::{data::Node, run_utils::run};
use orx_parallel::*;
use rayon::iter::*;

// all

pub fn run_all(roots: &[Node]) {
    println!("\n\n# MUTABLE COLLECTION");
    let log = |roots: Vec<Node>| {
        let fib_n_of_root0 = &roots[0].fib_n;
        println!(
            "  fib-n of root 0: {:?}",
            fib_n_of_root0.iter().take(15).collect::<Vec<_>>()
        )
    };

    // let f = || sequential(roots.to_vec());
    // run("sequential", f, log);

    // let f = || rayon(roots.to_vec());
    // run("rayon", f, log);

    let f = || orx_rec_exact(roots.to_vec());
    run("orx_rec_exact", f, log);

    let f = || orx_rec(roots.to_vec(), 1024);
    run("orx_rec", f, log);

    let f = || orx_rec_into_eager(roots.to_vec());
    run("orx_rec_into_eager", f, log);

    println!();
}

// seq

fn seq_compute_node(node: &mut Node) {
    node.fib_n = node.value.iter().map(|x| Node::compute(*x)).collect();
    for child in &mut node.children {
        seq_compute_node(child);
    }
}

pub fn sequential(mut roots: Vec<Node>) -> Vec<Node> {
    for root in roots.iter_mut() {
        seq_compute_node(root);
    }
    roots
}

// rayon

fn process_node<'scope>(node: &'scope mut Node, s: &rayon::Scope<'scope>) {
    for child in &mut node.children {
        s.spawn(move |s| {
            process_node(child, s);
        });
    }

    node.fib_n = node.value.par_iter().map(|x| Node::compute(*x)).collect();
}

pub fn rayon(mut roots: Vec<Node>) -> Vec<Node> {
    rayon::in_place_scope(|s| {
        for root in roots.iter_mut() {
            process_node(root, s);
        }
    });
    roots
}

// orx-parallel

struct NodePtr {
    value: *const Vec<u64>,
    children: *const Vec<Node>,
    fib_n: *mut Vec<u64>,
}

impl NodePtr {
    fn new(node: &Node) -> Self {
        Self {
            value: (&node.value) as *const Vec<u64>,
            children: (&node.children) as *const Vec<Node>,
            fib_n: (&node.fib_n) as *const Vec<u64> as *mut Vec<u64>,
        }
    }

    fn set_fib_n(&self, fib_n: impl Iterator<Item = u64>) {
        let vec_fib_n = unsafe { &mut *self.fib_n };
        *vec_fib_n = fib_n.collect();
    }

    fn children(&self) -> &[Node] {
        let children = unsafe { &*self.children };
        children.as_slice()
    }

    fn values(&self) -> &[u64] {
        let values = unsafe { &*self.value };
        values.as_slice()
    }
}

unsafe impl Send for NodePtr {}

fn extend<'a>(node_ptr: &NodePtr) -> impl ExactSizeIterator<Item = NodePtr> + use<'a> {
    let node_ptr = unsafe { &*(node_ptr as *const NodePtr) };
    node_ptr.children().iter().map(NodePtr::new)
}

pub fn orx_rec_exact(roots: Vec<Node>) -> Vec<Node> {
    let num_nodes: usize = roots.iter().map(|x| x.num_nodes()).sum();

    let root_ptrs: Vec<_> = roots.iter().map(NodePtr::new).collect();

    root_ptrs
        .into_par_rec_exact(extend, num_nodes)
        .for_each(|x| {
            let fib_n = x.values().iter().map(|x| Node::compute(*x));
            x.set_fib_n(fib_n);
        });

    roots
}

pub fn orx_rec(roots: Vec<Node>, chunk_size: usize) -> Vec<Node> {
    let root_ptrs: Vec<_> = roots.iter().map(NodePtr::new).collect();

    root_ptrs
        .into_par_rec(extend)
        .chunk_size(chunk_size)
        .for_each(|x| {
            let fib_n = x.values().iter().map(|x| Node::compute(*x));
            x.set_fib_n(fib_n);
        });

    roots
}

pub fn orx_rec_into_eager(roots: Vec<Node>) -> Vec<Node> {
    let root_ptrs: Vec<_> = roots.iter().map(NodePtr::new).collect();

    root_ptrs.into_par_rec(extend).into_eager().for_each(|x| {
        let fib_n = x.values().iter().map(|x| Node::compute(*x));
        x.set_fib_n(fib_n);
    });

    roots
}
