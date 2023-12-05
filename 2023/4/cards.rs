use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};
use std::error::Error;
use std::fmt::Error as fmtError;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = load_file("input.txt")?;
    let cards = parse_cards(reader)?;
    let sum = find_winning(cards);

    println!("{}", sum);
    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn parse_cards(reader: BufReader<File>) -> Result<Vec<(Vec<u32>, Vec<u32>)>, Box<dyn Error>> {
    
    let mut all_numbers = Vec::new();
    for card in reader.lines() {
        let card = card.expect("In loop");
        let mut numbers = card
            .split(": ").nth(1).ok_or(fmtError)?
            .split(" | ");

        let winning = numbers.nth(0).ok_or(fmtError)?.split(" ");
        let mut w_vec = Vec::new();
        for w in winning {
            if w != "" {
                w_vec.push(w.parse::<u32>()?);
            };
        };

        let owned = numbers.next().ok_or(fmtError)?.split(" ");
        let mut o_vec = Vec::new();
        for o in owned {
            if o != "" {
                o_vec.push(o.parse::<u32>()?);
            };
        };

        all_numbers.push((w_vec, o_vec));
    };

    Ok(all_numbers)
}

fn find_winning(cards: Vec<(Vec<u32>, Vec<u32>)>) -> u32 {

    let mut sum = 0;
    for card in &cards {
        let mut points = 0;
        for num in &card.1 {
            if card.0.contains(&num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
                println!("{}", num)
            }
        }

        sum += points;
    }

    sum
}