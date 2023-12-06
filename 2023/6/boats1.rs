use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = load_file("input.txt")?;
    let parsed = parse_file(&data);

    println!("{}", races_mul(parsed));

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_file(contents: &str) -> Vec<(u32, u32)> {
    
    return vec!((59, 430), (70, 1218), (78, 1213), (78, 1276))
}

fn races_mul(races: Vec<(u32, u32)>) -> u32 {
    let mut all_races = 1;
    for race in &races {
        let margin = highest_pressed(*race) - lowest_pressed(*race) + 1;
        all_races *= margin
    }

    all_races
}

fn lowest_pressed((time, best): (u32, u32)) -> u32 {
    
    for i in 0..time {
        let distance = i*(time-i);
        if distance > best {
            return i
        }
    }
    return 0
}

fn highest_pressed((time, best): (u32, u32)) -> u32 {
    
    for i in (0..time).rev() {
        let distance = i*(time-i);
        if distance > best {
            return i
        }
    }
    return 0
}
