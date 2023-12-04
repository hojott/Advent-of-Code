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
    Gear,
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
                '*'                       => Symbol::Gear,
                _                         => Symbol::Special, // "Shiny Null"
            };

            y.push(symbol);
        };

        x.push(y);

    };

    x
}

fn find_numbers(engine_data: Vec<Vec<Symbol>>) -> u32 {
    // Inefficient way to find all numbers
    let mut gears_sum = 0;

    for (i, row) in engine_data.iter().enumerate() {
        for (j, symbol) in row.iter().enumerate() {
            match symbol {
                Symbol::Gear =>  {
                    // horizontal numbers are found with diagonal numbers
                    let part_num: u32 = 0;

                    let (part_mul0, part_num) = horizontal_numbers(&row, j, part_num);
                    let (part_mul1, part_num) = diagonal_numbers(&engine_data, i, j, part_num);
                    
                    let part_mul = part_mul0 * part_mul1;

                    println!("{}", part_num);
                    println!("{}", part_mul);
                    if part_num == 2 {
                        gears_sum += part_mul;
                    };
                },
                _ => continue,
            }
        }
    }

    gears_sum
}

fn horizontal_numbers(row: &Vec<Symbol>, i: usize, mut part_num: u32) -> (u32, u32) {

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

    if &num0 != "" {
        part_num += 1;
    };
    if &num1 != "" {
        part_num += 1;
    };

    (num0.parse::<u32>().unwrap_or(1) * num1.parse::<u32>().unwrap_or(1), part_num)
}

fn diagonal_numbers(table: &Vec<Vec<Symbol>>, i: usize, j: usize, mut part_num: u32) -> (u32, u32) {
    // A bit of repeating code...

    let num0: u32;
    match table[i-1][j] {
        Symbol::Number(n) => (num0, part_num) = vertical_numbers(&table[i-1], j, n, part_num),
        _                 => (num0, part_num) = horizontal_numbers(&table[i-1], j, part_num)
    }

    let num1: u32;
    match table[i+1][j] {
        Symbol::Number(n) => (num1, part_num) = vertical_numbers(&table[i+1], j, n, part_num),
        _                 => (num1, part_num) = horizontal_numbers(&table[i+1], j, part_num)
    }

    (num0 * num1, part_num)
}

fn vertical_numbers(row: &Vec<Symbol>, i: usize, num_at: u32, part_num: u32) -> (u32, u32) {

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

    (num.parse::<u32>().unwrap(), part_num+1)
}