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

        let num0 = find_digits(&line, false);
        let num1 = find_digits(&reversed(&line), true);
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

fn find_digits(str: &str, reverse: bool) -> i32 {
    for (id, char) in str.chars().enumerate() {

        if char.is_digit(10) {
            return char.to_digit(10).unwrap()
                .try_into().unwrap()
        };

        // could be sped up if the checks are made only if the char is valid
        if !reverse {
            if let Ok(t) = calculate_chars(str, id.try_into().unwrap(), false) {
                return t
            };
        } else {
            if let Ok(t) = calculate_chars(str, id.try_into().unwrap(), true) {
                return t
            }
        }
    }
    0
}

fn calculate_chars(str: &str, id: usize, reverse: bool) -> Result<i32, ()> {
    // its okay..

    let mut start2 = 0;
    let mut start3 = 0;
    let mut start4 = 0;
    if id > 2 {
        start2 = id-2
    };
    if id > 3 {
        start3 = id-3
    };
    if id > 4 {
        start4 = id-4
    };

    if !reverse {
        if &str[start3..id+1] == "zero" { return Ok(0) }
        else if &str[start2..id+1] == "one" { return Ok(1) }
        else if &str[start2..id+1] == "two" { return Ok(2) }
        else if &str[start4..id+1] == "three" { return Ok(3) }
        else if &str[start3..id+1] == "four" { return Ok(4) }
        else if &str[start3..id+1] == "five" { return Ok(5) }
        else if &str[start2..id+1] == "six" { return Ok(6) }
        else if &str[start4..id+1] == "seven" { return Ok(7) }
        else if &str[start4..id+1] == "eight" { return Ok(8) }
        else if &str[start3..id+1] == "nine" { return Ok(9) }
        else { Err(()) }
    } else {
        if &str[start3..id+1] == "orez" { return Ok(0) }
        else if &str[start2..id+1] == "eno" { return Ok(1) }
        else if &str[start2..id+1] == "owt" { return Ok(2) }
        else if &str[start4..id+1] == "eerht" { return Ok(3) }
        else if &str[start3..id+1] == "ruof" { return Ok(4) }
        else if &str[start3..id+1] == "evif" { return Ok(5) }
        else if &str[start2..id+1] == "xis" { return Ok(6) }
        else if &str[start4..id+1] == "neves" { return Ok(7) }
        else if &str[start4..id+1] == "thgie" { return Ok(8) }
        else if &str[start3..id+1] == "enin" { return Ok(9) }
        else { Err(()) }
    }
}
