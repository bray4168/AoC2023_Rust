use crate::input;
use crate::solve::Solve;
use std::collections::HashMap;

pub struct Solution2;

impl Solve for Solution2 {
    fn solve(&self) -> u32{
        let mut input: Vec<String> = vec![];
        let mut values: Vec<u32> = vec![];
        
        input::read_file(&"src/day1/input.txt", &mut input).unwrap();
        for line in input {
            get_values(&line, &mut values);
        }

        let sum: u32 = values.iter().sum();
        println!("The solution for solution 2 is: {}", sum);
        sum
    }
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
    for character in line.chars() {
        if character.is_numeric() && character.is_ascii() {
            if array[0] == 0 {
                array[0] = character.to_digit(10).unwrap();
            }
            array[1] = character.to_digit(10).unwrap();

            // Clean the string we are tracking
            string = Default::default();
        }
        else {
            string += &character.to_string();
            let num = check_strings_for_int(&string);
            if num != 0 {
                if array[0] == 0 {
                    array[0] = num;
                }
                array[1] = num;

                // Clean the string we are tracking
                string = Default::default();
            }
        }
    }
    let value = array[0]*10 + array[1];
    values.push(value);
}

#[cfg(test)]
mod tests {
    use crate::solve::Solve;
    use crate::day1::Solution2;

    #[test]
    fn solution_2_test() {
        let solution = Solution2{};
        assert_eq!(solution.solve(), 55477);
    }
}