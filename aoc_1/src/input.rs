use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn read_file(file_name: &str, input: &mut Vec<String>) -> io::Result<()> {
    let f = File::open(file_name)?;
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        input.push(line?);
    }
    Ok(())
}