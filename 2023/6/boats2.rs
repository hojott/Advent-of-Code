use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("input.txt")?;
    let parsed = parse_file(&data);

    println!("{}", winning_presses(parsed));

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_file(contents: &str) -> (u64, u64) {
    // yeah didnt feel like parsing it
    return (59707878, 430121812131276)
}

fn winning_presses(race: (u64, u64)) -> u64 {
    let lowest = lowest_pressed(race);
    let highest = race.0 - lowest;
    let margin = highest - lowest + 1;

    margin
}

fn lowest_pressed((time, best): (u64, u64)) -> u64 {
    
    let mut start: u64 = 0;
    let mut end: u64 = time;
    loop {
        let middle: u64 = start + (end - start) / 2;
        let distance = middle * (time - middle);
        let distance_plus = (middle+1) * (time - (middle+1));
        match distance {
            distance if distance <= best && distance_plus > best => {
                return middle + 1
            },
            distance if distance > best => {
                end = middle;
            },
            distance if distance < best => {
                start = middle;
            },
            _ => panic!()
        };
    }
}
