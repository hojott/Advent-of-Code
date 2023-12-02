use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};
use std::cmp::max;

fn main() -> std::io::Result<()> {
    let mut reader = load_file("input.txt")?;
 
    let mut line = String::new();
    let mut sum = 0;
    while let Ok(_) = reader.read_line(&mut line) {
        if line.len() == 0 {
            break;
        };

        let game = find_maxes(parse_game(&line));

        if !( (game.blues > 14) | (game.greens > 13) | (game.reds > 12) ) {
            sum += game.id;
            println!("{}", game.id);
        }

        line = String::new();
    }

    println!("{sum}");

    Ok(())
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

struct Game {
    id: i32,
    blues: i32,
    reds: i32,
    greens: i32,
}

struct Cubes {
    // enums would be better
    color: String,
    amount: i32,
}

fn parse_game(line: &str) -> (i32, Vec<Cubes>) {
    let mut game_data = line.split(": ");

    let id = game_data.nth(0).expect("Unless input is bad")
        .split(" ").nth(1).expect("^^^")
        .parse::<i32>().unwrap();

    let round_data = game_data.next().expect("See above")
        .split("; ");

    let mut game_data_parsed = Vec::new();
    for round in round_data {

        let cubes_data = round.split(", ");
        for cubes in cubes_data {

            let mut cubes_values = cubes.split(" ");
            game_data_parsed.push(Cubes {
                amount: cubes_values.nth(0).expect("")
                    .parse::<i32>().unwrap(),
                color: cubes_values.next().expect("")
                    .to_string().replace("\n", "")
            })

        }
    }

    (id, game_data_parsed)
}

fn find_maxes((id, all_cubes): (i32, Vec<Cubes>)) -> Game {
    let mut green_max = 0;
    let mut red_max = 0;
    let mut blue_max = 0;
    
    for cubes in all_cubes {
        match cubes.color.as_str() {
            "green" => { green_max = max(green_max, cubes.amount) },
            "blue" => { blue_max = max(blue_max, cubes.amount) },
            "red" => { red_max = max(red_max, cubes.amount) },
            &_ => todo!()
        }
    }

    Game { id: id, blues: blue_max, greens: green_max, reds: red_max }
}