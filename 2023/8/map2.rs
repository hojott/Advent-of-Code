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

fn parse_file(reader: &mut BufReader<File>) -> Result<(String, Vec<(String, String, String)>), ParseError> {

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

        locations.push((loc_name, dest_1, dest_2));
    }

    locations.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((directions, locations))

}

#[derive(Debug)]
struct Location {
    steps: usize,
    steps_to_loop: usize,
    direction_pos: usize,
    z_at: Vec<usize>
}

fn solve_maze(directions: String, locations: &Vec<(String, String, String)>) -> Result<u64, ParseError> {
    
    let mut start_locations: Vec<&String> = Vec::new();
    for location in locations {
        if location.0.chars().nth(2).unwrap() == 'A' {
            start_locations.push(&location.0)
        }
    };
    let mut loc_data: Vec<Location> = Vec::new();

    for start_location in start_locations {
        
        let mut location: &String = &start_location.clone();
        let mut direction_pos: usize = 0;
        let mut past_locations: Vec<(usize, &String)> = Vec::new();
        let mut steps: usize = 0;
        let mut z_at: Vec<usize> = Vec::new();
        let mut steps_to_loop: usize = 0;
        loop {
            direction_pos = steps % directions.len();

            if past_locations.contains(&(direction_pos, location)) {
                let past_i = past_locations.iter().position(|x| x == &(direction_pos, location)).unwrap();
                steps_to_loop = past_locations[past_i].0;
                break;
            }

            if location.chars().nth(2).unwrap() == 'Z' {
                z_at.push(steps);
            };

            past_locations.push((direction_pos, location));

            let direction = directions.chars()
                .nth(direction_pos).unwrap();

            let index = locations.binary_search_by(| x | x.0.cmp(&location)).unwrap();
            location = match direction {
                'L' => &locations[index].1,
                'R' => &locations[index].2,
                 _  => return Err(ParseError::InvalidDirectionError)
            };

            steps += 1;
        };

        loc_data.push(Location {
            steps: steps,
            steps_to_loop: steps_to_loop,
            direction_pos: direction_pos,
            z_at: z_at
        });
    }

    // Works only because of the input
    // doesn't work "always"

    let mut value = 1;
    for loc in loc_data {
        for zpoint in loc.z_at {
            value = lcm(value, zpoint.try_into().unwrap());
        }
    }

    Ok(value)
}

fn lcm(a: u64, b: u64) -> u64 {
    a*b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}
