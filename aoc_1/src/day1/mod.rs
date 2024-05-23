mod solution1;
mod solution2;
mod test;

pub use solution1::Solution1;
pub use solution2::Solution2;
use crate::solve::Solve;
use crate::solve::SolveDay;
use std::time::Instant;

pub struct Day1;

impl SolveDay for Day1 {
    fn solve_day(&self) {
        let solutions: [Box<dyn Solve>; 2] = [Box::new(Solution1{}), Box::new(Solution2{})];

        for solution in solutions.iter() {
            println!("\nStarting solution...");
            let now = Instant::now();
    
            solution.solve();
    
            let elapsed_time = now.elapsed();
            println!("Execution took {:?}.\n", elapsed_time);
        }
    }
}
