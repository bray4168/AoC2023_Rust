mod solution1;
mod solution2;
mod input;
mod solve;

use solve::Solve;
use solution1::Solution1;
use solution2::Solution2;
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

#[cfg(test)]
mod tests {
    use crate::solve::Solve;
    use crate::solution1::Solution1;
    use crate::solution2::Solution2;

    #[test]
    fn solution_1_test() {
        let solution = Solution1{};
        assert_eq!(solution.solve(), 55477);
    }

    #[test]
    fn solution_2_test() {
        let solution = Solution2{};
        assert_eq!(solution.solve(), 0);
    }
}