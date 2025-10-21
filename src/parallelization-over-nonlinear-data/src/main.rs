use crate::data::Node;

mod data;
mod immutable_collection;
mod immutable_reduction;
mod run_utils;

pub const AMOUNT_OF_WORK: usize = 1;

fn main() {
    let seed = 42;
    let roots = Node::example_roots(seed);

    immutable_reduction::run_all(&roots);
    immutable_collection::run_all(&roots);
}
