use crate::input;
use crate::solve::Solve;
use std::collections::HashMap;

pub struct Solution2;

impl Solve for Solution2 {
    fn solve(&self) -> u32 {
        let mut input: Vec<String> = vec![];
        
        input::read_file(&"src/day1/input.txt", &mut input).unwrap();
        solution(input)
    }
}

// Pub function so that we can test with other inputs
pub fn solution(input: Vec<String>) -> u32 {
    let mut values: Vec<u32> = vec![];

    for line in input {
        get_values(&line, &mut values);
    }

    let sum: u32 = values.iter().sum();
    println!("The solution for solution 2 is: {}", sum);
    sum
}

fn check_strings_for_int(string: &str) -> u32 {
    let ret = 0;
    let int_strings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for (each, int) in &int_strings {
        if string.contains(each) {
            return *int;
        }
    }
    ret
}

fn get_values(line: &String, values: &mut Vec<u32>) {
    let mut array: [u32; 2] = [0, 0];
    let mut string: String = Default::default();

    // Get the first value
    for character in line.chars() {
        if character.is_numeric() && character.is_ascii() {
            array[0] = character.to_digit(10).unwrap();
            break;
        }
        else {
            string += &character.to_string();
            let num = check_strings_for_int(&string);
            if num != 0 {
                array[0] = num;
                break;
            }
        }
    }

    // Get the first value going backwards
    let mut string: String = Default::default();
    for character in line.chars().rev() {
        if character.is_numeric() && character.is_ascii() {
            array[1] = character.to_digit(10).unwrap();
            break;
        }
        else {
            string += &character.to_string();
            let rev_string: String = string.chars().rev().collect();
            let num = check_strings_for_int(&rev_string);
            if num != 0 {
                array[1] = num;
                break;
            }
        }
    }

    let value = array[0]*10 + array[1];
    values.push(value);
}

