use crate::input;
use crate::solve::Solve;

pub struct Solution1;

impl Solve for Solution1 {
    fn solve(&self) -> u32{
        let mut input: Vec<String> = vec![];
        let mut values: Vec<u32> = vec![];
        
        input::read_file(&"src/day1/input.txt", &mut input).unwrap();
        for line in input {
            get_values(&line, &mut values);
        }
    
        let sum: u32 = values.iter().sum();
        println!("The solution for solution 1 is: {}", sum);
        sum
    }
}

fn get_values(line: &String, values: &mut Vec<u32>) {
    let mut array: [u32; 2] = [0, 0];
    for character in line.chars() {
        if character.is_numeric() && character.is_ascii() {
            if array[0] == 0 {
                array[0] = character.to_digit(10).unwrap();
            }
            array[1] = character.to_digit(10).unwrap();
        }
    }
    let value = array[0]*10 + array[1];
    values.push(value);
}

#[cfg(test)]
mod tests {
    use crate::solve::Solve;
    use crate::day1::Solution1;

    #[test]
    fn solution_1_test() {
        let solution = Solution1{};
        assert_eq!(solution.solve(), 55477);
    }
}