use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};
use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let file_name = "input.txt";

    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut sum = 0;

    while let Ok(_) = reader.read_line(&mut line) {
        if line.len() == 0 {
            break;
        }

        let num0 = find_digits(&line);
        let num1 = find_digits(&reversed(&line));
        let num = (num0.to_string() + &num1.to_string())
            .parse::<i32>().unwrap();

        sum += num;

        println!("{}, +{}", sum, num);
        
        line = String::new();
    }

    Ok(())
}

fn reversed(str: &str) -> String {
    str.chars().rev().collect()
}

fn find_digits(str: &str) -> i32 {
    for char in str.chars() {
        if char.is_digit(10) {

            return char.to_digit(10).unwrap()
                .try_into().unwrap()
        }
    }
    0
}