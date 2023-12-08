use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

use std::convert::TryInto;

fn main() {
    let mut reader = load_file("input.txt").unwrap();
    let (directions, locations) = parse_file(&mut reader).unwrap();

    let steps = solve_maze(directions, &locations).unwrap();
    println!("{}", steps);
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

#[derive(Debug)]
enum ParseError {
    BadFormattingError,
    InvalidDirectionError,
}

fn parse_file(reader: &mut BufReader<File>) -> Result<(String, Vec<(String, (String, String))>), ParseError> {

    let mut directions = String::new();
    let _ = reader.read_line(&mut directions);
    directions = directions.trim().to_string();

    let _ = reader.read_line(&mut String::new()); // skip line

    let mut locations = Vec::new();
    for result in reader.lines() {
        let raw_loc = result.unwrap();
        let mut split = raw_loc.split(" = (");

        let loc_name = split.nth(0).unwrap()
            .to_string();

        let raw_dest = match split.next() {
            Some(t) => t,
            None => return Err(ParseError::BadFormattingError)
        };
        split = raw_dest.split(", ");

        let dest_1 = split.nth(0).unwrap()
            .to_string();

        let raw_dest_2 = match split.next() {
                Some(t) => t,
                None => return Err(ParseError::BadFormattingError)
            };
        split = raw_dest_2.split(")");

        let dest_2 = split.nth(0).unwrap()
            .to_string();

        locations.push((loc_name, (dest_1, dest_2)));
    }

    locations.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((directions, locations))

}

fn solve_maze(directions: String, locations: &Vec<(String, (String, String))>) -> Result<u64, ParseError> {
    
    let mut steps: usize = 0;
    let mut location = "AAA";
    loop {
        let direction = directions.chars().nth(steps % directions.len())
            .unwrap();
        steps += 1;

        let index = locations.iter().position(|x| &x.0 == location).unwrap();
        location = match direction {
            'L' => &locations[index].1.0,
            'R' => &locations[index].1.1,
            _   => return Err(ParseError::InvalidDirectionError)
        };

        if location == "ZZZ" {
            return Ok(steps.try_into().unwrap())
        };
    }
}