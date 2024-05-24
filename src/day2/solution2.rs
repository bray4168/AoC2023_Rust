use crate::input;
use crate::solve::Solve;

pub struct Solution2;

impl Solve for Solution2 {
    fn solve(&self) -> u32 {
        let mut input: Vec<String> = vec![];
        
        input::read_file(&"src/day2/input.txt", &mut input).unwrap();
        solution(input)
    }
}

pub fn solution(input: Vec<String>) -> u32 {
    let mut values: Vec<u32> = vec![];

    for each in input {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let split: Vec<&str> = each.split(":").collect::<Vec<_>>();
        let pulls: Vec<&str> = split[1].split([';',',']).collect::<Vec<_>>();

        for pull in pulls {
            let pull_int: u32 = pull.split(" ").collect::<Vec<_>>()[1].parse().unwrap();
            if pull.contains("green") {
                if pull_int > min_green {
                    min_green = pull_int;
                }
            }
            else if pull.contains("red") {
                if pull_int > min_red {
                    min_red = pull_int;
                }
            }
            else if pull.contains("blue") {
                if pull_int > min_blue {
                    min_blue = pull_int;
                }
            }
        }
        values.push(min_green * min_red * min_blue);
    }

    let sum: u32 = values.iter().sum();
    println!("The solution for solution 2 is: {}", sum);
    sum
}