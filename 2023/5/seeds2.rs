use std::fs::File;
use std::io::prelude::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = load_file("input.txt")?;
    let (seeds, categories) = parse_data(&file_contents);

    let locations = find_locations(seeds, categories);

    println!("{:?}", locations.iter().min_by_key(|i: &&(u64, u64)| i.0).expect("I checked").0);

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_data(data: &str) -> (Vec<(u64, u64)>, Vec<Vec<(u64, u64, u64)>>) {

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

fn parse_seeds(raw_seeds: &str) -> Vec<(u64, u64)> {

    let mut seed_values = Vec::new();
    let raw_seed_values = raw_seeds.split(": ").nth(1).expect("Data should be correct");
    for raw_seed in raw_seed_values.split(" ") {
        seed_values.push(raw_seed.parse::<u64>().unwrap());
    };

    let mut seeds = Vec::new();
    for (i, seed_value) in seed_values.iter().enumerate() {
        if i % 2 == 1 {
            seeds.push((seed_values[i-1], seed_values[i-1] + seed_value));
        }
    }

    seeds
}

fn parse_categories(raw_category: &str) -> Vec<(u64, u64, u64)> {

    let mut category = Vec::new();
    let mut raw_category_values = raw_category.split("\n");
    raw_category_values.next();

    for raw_line in raw_category_values {

        let mut line = Vec::new();
        for raw_value in raw_line.split(" ") {
            line.push(raw_value.parse::<u64>().unwrap());
        }
        category.push((line[0], line[1], line[2]));

    }

    category
}

fn find_locations(seeds: Vec<(u64, u64)>, categories: Vec<Vec<(u64, u64, u64)>>) -> Vec<(u64, u64)> {

    let mut locations = Vec::new();
    for seed in seeds.iter() {
        let mut source: Vec<(u64, u64)> = vec!(*seed);
        for category in &categories {
            let mut destination = Vec::new();
            for src in &source {
                destination.extend(find_destination(&src, category));
            };
            source = destination.clone();
        };
        locations.extend(source);
    };

    locations
}

fn find_destination(source: &(u64, u64), category: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    for map in category {
        let range = (map.1, map.1 + map.2);
        match source {
            source if range.0 > source.1 || range.1 <= source.0 => {
                continue
            },
            source if range.0 > source.0 && range.1 < source.1 => {
                continue
            },
            source if range.0 <= source.0 && range.1 >= source.1 => {
                return vec!((map.0 + (source.0 - range.0), map.0 + (source.1 - range.0)))
            },
            source if range.0 <= source.0 && range.1 < source.1 => {
                let mut ret = vec!((map.0 + (source.0 - range.0), map.0 + (range.1 - range.0)));
                let leftovers = find_destination(&(range.1, source.1), category);
                ret.extend(leftovers);
                return ret
            },
            source if range.0 > source.0 && range.1 >= source.1 => {
                let mut ret = vec!((map.0 + (range.0 - range.0), map.0 + (source.1 - range.0)));
                let leftovers = find_destination(&(source.0, range.0 - 1), category);
                ret.extend(leftovers);
                return ret
            },
            _ => panic!()
        }
    };
    return vec!(*source)
}