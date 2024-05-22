use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    println!("Starting solution");
    let now = Instant::now();

    solve();

    let elapsed_time = now.elapsed();
    println!("Execution took {:?} seconds.", elapsed_time);
}

fn solve() {
    let mut input: Vec<String> = vec![];
    let mut values: Vec<u32> = vec![];
    
    read_file(&mut input).unwrap();
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
                array[1] = character.to_digit(10).unwrap();
            }
            else {
                array[1] = character.to_digit(10).unwrap();
            }
        }
    }
    let value = array[0]*10 + array[1];
    values.push(value);
}


fn read_file(input: &mut Vec<String>) -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        input.push(line?);
    }
    Ok(())
}