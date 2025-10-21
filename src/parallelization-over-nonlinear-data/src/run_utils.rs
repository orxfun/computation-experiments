use std::time::Instant;

pub fn run<F, L, T>(name: &'static str, fun: F, log: L)
where
    F: Fn() -> T,
    L: Fn(T),
{
    println!("> {name}");
    let start = Instant::now();

    let result = fun();

    let elapsed = start.elapsed();

    println!("  elapsed = {elapsed:?}");
    log(result);
    println!();
}
