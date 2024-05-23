use crate::input;
use crate::solve::Solve;

pub struct Solution2;

impl Solve for Solution2 {
    fn solve(&self) -> u32 {
        let mut input: Vec<String> = vec![];
        
        input::read_file(&"src/day1/input.txt", &mut input).unwrap();
        solution(input)
    }
}

pub fn solution(input: Vec<String>) -> u32 {
    0
}