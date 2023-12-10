use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

fn main() {
    let reader = load_file("input.txt").unwrap();
    let data = parse_file(reader);

    let mut sum: i32 = 0;
    for v in data.into_iter() {
        let next = find_prev(v);
        println!("{}", next);
        sum += next;
    }

    println!("{}", sum);
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn parse_file(reader: BufReader<File>) -> Vec<Vec<i32>> {

    let mut data: Vec<Vec<i32>> = Vec::new();
    for result in reader.lines() {
        let line = result.unwrap();
        let line_split = line.split(" ");

        let mut values: Vec<i32> = Vec::new();
        for raw_value in line_split {
            let value = raw_value.trim()
                .parse::<i32>().unwrap();
            values.push(value);
        }

        data.push(values);
    }

    data
}

fn derivate(values: &Vec<i32>) -> Vec<i32> {
    let mut derived: Vec<i32> = Vec::new();
    for i in 0..values.len() {
        if i == 0 { continue };

        derived.push(values[i] - values[i-1])
    };

    derived
}

fn find_prev(values: Vec<i32>) -> i32 {

    let mut derivates: Vec<Vec<i32>> = Vec::new();
    derivates.push(values);
    let mut i = 0;
    loop {
        if derivates[i] == vec!(0; derivates[i].len()) { break };
        derivates.push(derivate(&derivates[i]));
        i += 1;
    }

    let len = derivates.len();
    for i in (0..len).rev() {
        if i == 0 { break };
        let prev = derivates[i-1][0] - derivates[i][0];
        derivates[i-1].insert(0, prev);
    };

    derivates[0][0]
}
