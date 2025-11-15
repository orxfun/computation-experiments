fn main() {
    divan::main();
}

#[divan::bench(sample_count = 5, args = [6, 8, 10])]
// #[divan::bench(sample_count = 5, args = [1])]
fn pipeline_threads(num_threads: usize) {
    length_delimited_codec::run_pipeline("../../s1415-big.bin", "../../bench_output.bin", num_threads)
        .unwrap();
}

