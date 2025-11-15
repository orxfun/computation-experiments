use super::data::Node;
use crate::run_utils::run;
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

    let roots = || roots.to_vec();

    run("sequential", || sequential(roots()), log);

    // rayon miri fails with:
    // Undefined Behavior: trying to retag from <84156795> for SharedReadWrite permission at alloc41643328[0x8],
    // but that tag does not exist in the borrow stack for this location
    #[cfg(not(miri))]
    run("rayon", || rayon(roots()), log);

    run("orx_rec_exact", || orx_rec_exact(roots()), log);

    run("orx_rec_1024", || orx_rec_1024(roots(), 1024), log);

    run("orx_rec_linearize", || orx_rec_linearize(roots()), log);

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

/// A temporary version of [`Node`] to be used for the parallel computation.
struct NodePtr {
    /// Values are immutable throughout the computation.
    value: *const Vec<u64>,
    /// We do not mutate the children structure itself; however,
    /// we mutate the `fib_n` field of each child, hence the `*mut`.
    children: *mut Vec<Node>,
    /// This is our result that we mutate.
    /// Since the parallel iterator will process each node only once,
    /// this field will be written exactly once during the parallel
    /// computation, and hence, there will be no race condition.
    fib_n: *mut Vec<u64>,
}

impl NodePtr {
    /// SAFETY: We create the NodePtr` from a mutually exclusive reference
    /// to make sure that we create only one `NodePtr` for each `Node`.
    fn new(node: &mut Node) -> Self {
        Self {
            value: (&node.value) as *const Vec<u64>,
            children: (&mut node.children) as *mut Vec<Node>,
            fib_n: (&mut node.fib_n) as *mut Vec<u64>,
        }
    }

    fn values(&self) -> &[u64] {
        // SAFETY: Values are immutable behind `*const` throughout the computation.
        // Further, since the `NodePtr` is created from a `&mut Node` reference,
        // it cannot outlive the node it is created for. Therefore, the pointer
        // and the dereferencing is valid.
        let values = unsafe { &*self.value };
        values.as_slice()
    }

    /// SAFETY: This method must be called at most once per `NodePtr`, and hence,
    /// per `Node` concurrently to avoid race conditions.
    ///
    /// SAFETY-PAR-ITER: It is safe to use for the extension of a recursive parallel iterator,
    /// since the iterator guarantees that each node will be processed exactly once.
    fn set_fib_n(self, fib_n: Vec<u64>) {
        // SAFETY: Since this method is called at most once per `NodePtr`, this mutable reference will be mutually exclusive.
        let vec_fib_n = unsafe { &mut *(self.fib_n as *mut Vec<u64>) };
        *vec_fib_n = fib_n;
    }
}

unsafe impl Send for NodePtr {}

/// SAFETY: This method must be called at most once per `NodePtr`, and hence,
/// per `Node` concurrently to avoid race conditions.
///
/// SAFETY-PAR-ITER: It is safe to use for the extension of a recursive parallel iterator,
/// since the iterator guarantees that each node will be extended exactly once.
fn extend(node_ptr: &NodePtr, queue: &Queue<NodePtr>) {
    // SAFETY: Since this method is called at most once per `NodePtr`, this mutable reference will be mutually exclusive.
    let children = unsafe { &mut *node_ptr.children };
    queue.extend(children.iter_mut().map(NodePtr::new));
}

pub fn orx_rec_exact(mut roots: Vec<Node>) -> Vec<Node> {
    let num_nodes: usize = roots.iter().map(|x| x.num_nodes()).sum();

    let root_ptrs: Vec<_> = roots.iter_mut().map(NodePtr::new).collect();

    root_ptrs
        .into_par_rec_exact(extend, num_nodes)
        .for_each(|x| {
            let fib_n = x.values().iter().map(|x| Node::compute(*x)).collect();
            x.set_fib_n(fib_n);
        });

    roots
}

pub fn orx_rec_1024(mut roots: Vec<Node>, chunk_size: usize) -> Vec<Node> {
    let root_ptrs: Vec<_> = roots.iter_mut().map(NodePtr::new).collect();

    root_ptrs
        .into_par_rec(extend)
        .chunk_size(chunk_size)
        .for_each(|x| {
            let fib_n = x.values().iter().map(|x| Node::compute(*x)).collect();
            x.set_fib_n(fib_n);
        });

    roots
}

pub fn orx_rec_linearize(mut roots: Vec<Node>) -> Vec<Node> {
    let root_ptrs: Vec<_> = roots.iter_mut().map(NodePtr::new).collect();

    root_ptrs.into_par_rec(extend).linearize().for_each(|x| {
        let fib_n = x.values().iter().map(|x| Node::compute(*x)).collect();
        x.set_fib_n(fib_n);
    });

    roots
}
