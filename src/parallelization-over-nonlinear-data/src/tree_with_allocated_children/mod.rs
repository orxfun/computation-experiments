use crate::tree_with_allocated_children::data::Node;

mod data;
mod immutable_collection;
mod immutable_reduction;
mod mutable_collection;
mod using_immutable_reduction;

pub fn run(seed: u64) {
    let roots = Node::example_roots(seed);

    immutable_reduction::run_all(&roots);
    immutable_collection::run_all(&roots);
    mutable_collection::run_all(&roots);

    using_immutable_reduction::run_all(&roots);
}
