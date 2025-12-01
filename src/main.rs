use aoc2025::cli::{parse_args, run};

fn main() {
    if let Err(e) = parse_args().and_then(run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
