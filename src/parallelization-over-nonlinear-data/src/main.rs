use clap::Parser;
use std::sync::OnceLock;

mod run_utils;
mod tree_with_allocated_children;
mod tree_with_on_the_fly_children;

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

    tree_with_allocated_children::run(seed);
    tree_with_on_the_fly_children::run(seed);
}
