mod solution1;
mod solution2;
mod test;

pub use solution1::Solution1;
pub use solution2::Solution2;
use crate::solve::{Solve, SolveDay};
use std::time::Instant;

pub struct Day2;

impl SolveDay for Day2 {
    fn solve_day(&self) {
        let solutions: [Box<dyn Solve>; 2] = [Box::new(Solution1{}), Box::new(Solution2{})];
        
        println!("Starting day 1 solutions...");

        for solution in solutions.iter() {
            println!("\nStarting solution...");
            let now = Instant::now();
    
            solution.solve();
    
            let elapsed_time = now.elapsed();
            println!("Execution took {:?}.\n", elapsed_time);
        }
    }
}
