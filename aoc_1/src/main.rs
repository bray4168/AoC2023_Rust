mod solution;
mod input;
use std::time::Instant;

fn main() {
    println!("Starting solution");
    let now = Instant::now();

    solution::solve();

    let elapsed_time = now.elapsed();
    println!("Execution took {:?}.", elapsed_time);
}