mod solution1;
mod solution2;
mod input;
use std::time::Instant;

fn main() {
    println!("Starting solution 1...");
    let now = Instant::now();

    solution1::solve();

    let elapsed_time = now.elapsed();
    println!("Execution took {:?}.", elapsed_time);
    println!("\nStarting solution 2...");
    let now = Instant::now();

    solution2::solve();

    let elapsed_time = now.elapsed();
    println!("Execution took {:?}.", elapsed_time);
}

#[cfg(test)]
mod tests {
    use crate::solution1;
    use crate::solution2;

    #[test]
    fn solution_1_test() {
       assert_eq!(solution1::solve(), 55477);
    }

    #[test]
    fn solution_2_test() {
       assert_eq!(solution2::solve(), 0);
    }
}