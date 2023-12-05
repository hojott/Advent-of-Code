use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = load_file("input.txt")?;
    let (seeds, categories) = parse_data(&file_contents);

    let locations = find_locations(seeds, categories);

    println!("{}", locations.iter().min().expect("I checked"));

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_data(data: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {

    let (mut seeds, mut categories) = (Vec::new(), Vec::new());
    for (i, category) in data.split("\n\n").enumerate() {
        if i == 0 {
            seeds = parse_seeds(&category);
        } else {
            categories.push(parse_categories(&category));
        };
    };

    (seeds, categories)
}

fn parse_seeds(raw_seeds: &str) -> Vec<u64> {

    let mut seeds = Vec::new();
    let raw_seed_values = raw_seeds.split(": ").nth(1).expect("Data should be correct");
    for raw_seed in raw_seed_values.split(" ") {
        seeds.push(raw_seed.parse::<u64>().unwrap());
    };

    seeds
}

fn parse_categories(raw_category: &str) -> Vec<Vec<u64>> {

    let mut category = Vec::new();
    let mut raw_category_values = raw_category.split("\n");
    raw_category_values.next();

    for raw_line in raw_category_values {

        let mut line = Vec::new();
        for raw_value in raw_line.split(" ") {
            line.push(raw_value.parse::<u64>().unwrap());
        }
        category.push(line);

    }

    category
}

fn find_locations(seeds: Vec<u64>, categories: Vec<Vec<Vec<u64>>>) -> Vec<u64> {

    let mut locations = Vec::new();
    for seed in &seeds {
        let mut source = *seed;
        for category in &categories {
            source = find_destination(source, category);
        };

        locations.push(source);
    };

    locations
}

fn find_destination(source: u64, category: &Vec<Vec<u64>>) -> u64 {
    println!("{}", source);
    for map in category {
        let map_range = map[1]..map[1]+map[2];
        if map_range.contains(&source) {
            for i in map_range {
                if source == i {
                    return map[0]+(i-map[1])
                }
            }
        }
    }

    return source
}