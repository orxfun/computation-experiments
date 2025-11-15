use length_delimited_codec::run_pipeline;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <input_path> <output_path> <num_processors>", args[0]);
        eprintln!("Example: {} ./input.bin ./output.bin 4", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let num_processors: usize = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Error: num_processors must be a valid number");
        std::process::exit(1);
    });

    println!("Running pipeline:");
    println!("  Input: {}", input_path);
    println!("  Output: {}", output_path);
    println!("  Processors: {}", num_processors);

    match run_pipeline(input_path, output_path, num_processors) {
        Ok(()) => {
            println!("Pipeline completed successfully!");
        }
        Err(e) => {
            eprintln!("Pipeline failed: {}", e);
            std::process::exit(1);
        }
    }
}
