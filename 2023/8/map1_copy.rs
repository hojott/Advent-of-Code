use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

use std::rc::{
    Rc,
    Weak
};

use std::convert::TryInto;

fn main() {
    let mut reader = load_file("input.txt").unwrap();
    let (directions, raw_locations) = parse_file(&mut reader).unwrap();

    let locations: Vec<Rc<Location>> = Vec::new();
    for raw_location in raw_locations {
        Location::to_vec(
            raw_location.0,
            &raw_locations,
            &mut locations
        );
    }

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

fn parse_line(line: String) -> Result<(String, String, String), ParseError> {
    let mut split = line.split(" = (");

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

    Ok((loc_name, dest_1, dest_2))
}

fn parse_file(reader: &mut BufReader<File>) -> Result<(String, Vec<(String, String, String)>), ParseError> {

    let mut directions = String::new();
    let _ = reader.read_line(&mut directions);
    directions = directions.trim().to_string();

    let _ = reader.read_line(&mut String::new()); // skip line

    let mut raw_destinations = Vec::new();
    for (i, result) in reader.lines().enumerate() {
        let raw_loc = result.unwrap();
        let (loc_name, dest_1, dest_2) = parse_line(raw_loc)?;
        raw_destinations.push((loc_name, dest_1, dest_2));
    }

    raw_destinations.sort_by(|a, b| a.0.cmp(&b.0));

    Ok((directions, raw_destinations))
}

#[derive(Clone, Debug)]
struct Location {
    id: String,
    left: Weak<Location>,
    right: Weak<Location>,
}

impl Location {

    fn to_vec(loc_name: String, raw_locs: &Vec<(String, String, String)>, locs: &mut Vec<Rc<Location>>) {

        let mut weak_locs: Vec<&Weak<Location>> = Vec::new();
        let mut loc_names: Vec<String> = Vec::new();

        let i = raw_locs.as_slice()
            .binary_search_by(|x| x.0.cmp(&loc_name.to_string()))
            .unwrap();

        let location = Rc::new_cyclic( | me |
            Location {
                id: loc_name,
                left: Location::__to_vec_rec(
                    raw_locs[i].1,
                    raw_locs,
                    &mut weak_locs,
                    &mut loc_names,
                    &mut locs,
                    me.clone(),
                    loc_name
                ),
                right: Location::__to_vec_rec(
                    raw_locs[i].2,
                    raw_locs,
                    &mut weak_locs,
                    &mut loc_names,
                    &mut locs,
                    me.clone(),
                    loc_name
                )
            }
        );
    }

    fn __to_vec_rec(
        loc_name: String,
        raw_locs: &Vec<(String, String, String)>,
        weak_locs: &mut Vec<&Weak<Location>>,
        loc_names: &mut Vec<String>,
        locs: &mut Vec<Rc<Location>>,
        prev: Weak<Location>,
        prev_name: String
    ) -> Weak<Self> {

        let prev_i = loc_names.partition_point(|&x| x < prev_name);
        loc_names.insert(prev_i, prev_name);
        weak_locs.insert(prev_i, &prev);

        match loc_names.as_slice()
            .binary_search(|x: &String| x.cmp(&loc_name.to_string()))
            {
            Ok(i) => return *weak_locs[i],
            Err(_) => { /* ignore */ }   
        };

        let i = raw_locs.as_slice()
            .binary_search_by(|x| x.0.cmp(&loc_name.to_string()))
            .unwrap();

        let location = Rc::new_cyclic( | me |
            Location {
                id: loc_name,
                left: Location::__to_vec_rec(
                    raw_locs[i].1,
                    raw_locs,
                    &mut weak_locs,
                    &mut loc_names,
                    &mut locs,
                    me.clone(),
                    loc_name
                ),
                right: Location::__to_vec_rec(
                    raw_locs[i].2,
                    raw_locs,
                    &mut weak_locs,
                    &mut loc_names,
                    &mut locs,
                    me.clone(),
                    loc_name
                )
            }
        );

        let locs_i = locs.partition_point(|&x| x.id < location.id);
        locs.insert(locs_i, location);

        Rc::downgrade(&location)
    }
}

fn solve_maze(directions: String, locations: &Vec<Rc<Location>>) -> Result<u64, ParseError> {
    
    let mut steps: usize = 0;
    let loc_i = locations.as_slice()
        .binary_search_by(|x| x.id.cmp(&"AAA".to_string()))
        .unwrap();
    let mut location: Rc<Location> = locations[loc_i];
    loop {
        let direction = directions.chars()
            .nth(steps % directions.len())
            .unwrap();
        steps += 1;

        location = match direction {
            'L' => location.left.upgrade().unwrap(),
            'R' => location.right.upgrade().unwrap(),
            _ => return Err(ParseError::InvalidDirectionError)
        };

        if location.id == "ZZZ".to_string() {
            return Ok(steps.try_into().unwrap())
        };
    }
}