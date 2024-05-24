use crate::input;
use crate::solve::Solve;

pub struct Solution1;

impl Solve for Solution1 {
    fn solve(&self) -> u32 {
        let mut input: Vec<String> = vec![];
        
        input::read_file(&"src/day2/input.txt", &mut input).unwrap();
        solution(input)
    }
}

pub fn solution(input: Vec<String>) -> u32 {
    let mut values: Vec<u32> = vec![];
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for each in input {
        let split: Vec<&str> = each.split(":").collect::<Vec<_>>();
        let mut id: u32 = split[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        let pulls: Vec<&str> = split[1].split([';',',']).collect::<Vec<_>>();

        for pull in pulls {
            let pull_int: u32 = pull.split(" ").collect::<Vec<_>>()[1].parse().unwrap();
            if pull.contains("green") {
                if pull_int > max_green {
                    id = 0;
                }
            }
            else if pull.contains("red") {
                if pull_int > max_red {
                    id = 0;
                }
            }
            else if pull.contains("blue") {
                if pull_int > max_blue {
                    id = 0;
                }
            }
        }

        values.push(id);
    }

    let sum: u32 = values.iter().sum();
    println!("The solution for solution 1 is: {}", sum);
    sum
}