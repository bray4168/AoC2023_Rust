mod solution;
mod utility;
use std::time::Instant;

fn main() {
    println!("Starting solution");
    let now = Instant::now();

    solution::solve();

    let elapsed_time = now.elapsed();
    println!("Execution took {:?} seconds.", elapsed_time);
}