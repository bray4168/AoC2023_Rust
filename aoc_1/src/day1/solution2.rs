use crate::input;
use crate::solve::Solve;

pub struct Solution2;

impl Solve for Solution2 {
    fn solve(&self) -> u32{
        let mut input: Vec<String> = vec![];
        let mut values: Vec<u32> = vec![];
        
        input::read_file(&mut input).unwrap();
        for line in input {
            get_values(&line, &mut values);
        }

        let sum: u32 = values.iter().sum();
        println!("The solution for solution 2 is: {}", sum);
        sum
    }
}

fn check_strings_for_int(string: &str) -> u32{
    let mut ret = 0;
    let int_strings: [&str; 9] = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    for each in int_strings {
        if string.contains(each) {
            //println!("Matched with {}", each);
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
            // Do something to see if we spell a number
            string += &character.to_string();
            let num = check_strings_for_int(&string);
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