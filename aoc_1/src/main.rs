mod day1;
mod input;
mod solve;

use solve::Solve;
use day1::{Solution1, Solution2};
use std::time::Instant;

fn main() {
    let solutions: [Box<dyn Solve>; 2] = [Box::new(Solution1{}), Box::new(Solution2{})];
    
    for solution in solutions.iter() {
        println!("\nStarting solution...");
        let now = Instant::now();

        solution.solve();

        let elapsed_time = now.elapsed();
        println!("Execution took {:?}.\n", elapsed_time);
    }
}