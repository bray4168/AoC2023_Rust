use crate::utility;

pub fn solve() {
    let mut input: Vec<String> = vec![];
    let mut values: Vec<u32> = vec![];
    
    utility::read_file(&mut input).unwrap();
    for line in input {
        get_values(&line, &mut values);
    }

    let sum: u32 = values.iter().sum();
    println!("The answer is: {}", sum);
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