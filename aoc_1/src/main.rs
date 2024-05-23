mod day1;
mod day2;
mod input;
mod solve;

use day1::Day1;
use day2::Day2;
use solve::SolveDay;

fn main() {
    let days: [Box<dyn SolveDay>; 2] = [Box::new(Day1{}), Box::new(Day2{})];
    
    for day in days.iter() {
        day.solve_day();
    }
}