use crate::data::Node;
use clap::Parser;
use std::sync::OnceLock;

mod data;
mod immutable_collection;
mod immutable_reduction;
mod mutable_collection;
mod run_utils;
mod using_immutable_reduction;

#[derive(Parser, Debug)]
struct Args {
    /// Amount of work (num times Fibonacci will be repeated).
    #[arg(long, default_value_t = 1)]
    amount_of_work: usize,
}

pub fn amount_of_work() -> &'static usize {
    static WORK: OnceLock<usize> = OnceLock::new();
    WORK.get_or_init(|| Args::parse().amount_of_work)
}

fn main() {
    let seed = 42;
    let roots = Node::example_roots(seed);

    // immutable_reduction::run_all(&roots);
    // immutable_collection::run_all(&roots);
    mutable_collection::run_all(&roots);

    // using_immutable_reduction::run_all(&roots);
}
