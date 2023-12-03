use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

fn main() -> std::io::Result<()> {
    let reader = load_file("input.txt")?;
    let parsed = parse_reader(reader);

    println!("{}", find_numbers(parsed));

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

enum Symbol {
    Number(u32),
    Special,
    Null
}

fn parse_reader(reader: BufReader<File>) -> Vec<Vec<Symbol>> {

    let mut x = Vec::new();
    for line in reader.lines() {

        let mut y = Vec::new();
        for char in line.expect("In loop").chars() {

            let symbol = match char {
                '.'                       => Symbol::Null,
                char if char.is_digit(10) => Symbol::Number(char.to_digit(10).unwrap()),
                _                         => Symbol::Special,
            };

            y.push(symbol);
        };

        x.push(y);

    };

    x
}

fn find_numbers(engine_data: Vec<Vec<Symbol>>) -> u32 {
    // Inefficient way to find all numbers
    let mut engine_sum = 0;

    for (i, row) in engine_data.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            match symbol {
                Symbol::Special =>  {
                    // horizontal numbers are found with diagonal numbers
                    let part_sum = horizontal_numbers(&row, j) + diagonal_numbers(&engine_data, i, j);

                    engine_sum += part_sum;
                },
                _ => continue,
            }
        }
    }

    engine_sum
}

fn horizontal_numbers(row: &Vec<Symbol>, i: usize) -> u32 {

    let mut num0 = "".to_string();
    for symbol in row[i+1..].iter() {
        match symbol {
            Symbol::Number(n) => num0 = num0 + &n.to_string(),
            _                 => break
        }
    };

    let mut num1 = "".to_string();
    for symbol in row[..i].iter().rev() {
        match symbol {
            Symbol::Number(n) => num1 = n.to_string() + &num1,
            _                 => break
        }
    };

    num0.parse::<u32>().unwrap_or(0) + num1.parse::<u32>().unwrap_or(0)
}

fn diagonal_numbers(table: &Vec<Vec<Symbol>>, i: usize, j: usize) -> u32 {
    // A bit of repeating code...

    let num0: u32;
    match table[i-1][j] {
        Symbol::Number(n) => num0 = vertical_numbers(&table[i-1], j, n),
        _                 => num0 = horizontal_numbers(&table[i-1], j)
    }

    let num1: u32;
    match table[i+1][j] {
        Symbol::Number(n) => num1 = vertical_numbers(&table[i+1], j, n),
        _                 => num1 = horizontal_numbers(&table[i+1], j)
    }

    num0 + num1
}

fn vertical_numbers(row: &Vec<Symbol>, i: usize, num_at: u32) -> u32 {

    let mut num = num_at.to_string();

    for symbol in row[i+1..].iter() {
        match symbol {
            Symbol::Number(n) => num = num.to_string() + &n.to_string(),
            _                 => break,
        }
    };
    for symbol in row[..i].iter().rev() {
        match symbol {
            Symbol::Number(n) => num = n.to_string() + &num,
            _                 => break,
        }
    };

    num.parse::<u32>().unwrap_or(0)
}