use std::fs::File;
use std::io::{
    prelude::*,
    Result as ioResult,
    BufReader
};
use Direction::*;

fn main() {
    let reader: BufReader<File> = load_file("input.txt").unwrap();
    let (start, map) = parse_file(reader);

    let len = find_pipe_length(start, map).unwrap();
    println!("{}", len/2);
}

pub fn load_file(file_name: &str) -> ioResult<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Any,
    Empty
}

impl Direction {
    fn opposite(&self) -> Self {
        return match self {
            Left  => Right,
            Right => Left,
            Up    => Down,
            Down  => Up,
            Any   => Empty,
            Empty  => Any
        }
    }
}

#[derive(Debug)]
struct Pipe {
    directions: Vec<Direction>
}

impl Pipe {
    fn new(first_direction: Direction, second_direction: Direction) -> Self {
        Pipe {directions: [first_direction, second_direction].to_vec()}
    }
}

fn parse_file(reader: BufReader<File>) -> ((usize, usize), Vec<Vec<Pipe>>) {
    
    let mut start = (0, 0);
    let mut y: Vec<Vec<Pipe>> = Vec::new();
    for (i, result) in reader.lines().enumerate() {
        let line = result.unwrap();
        let mut x: Vec<Pipe> = Vec::new();
        for (j, char) in line.chars().enumerate() {
            x.push(match char {
                '-' => Pipe::new(Left, Right),
                '|' => Pipe::new(Up, Down),
                'J' => Pipe::new(Left, Up),
                '7' => Pipe::new(Left, Down),
                'L' => Pipe::new(Right, Up),
                'F' => Pipe::new(Right, Down),
                'S' => {start = (i, j); Pipe::new(Any, Any)},
                '.' => Pipe::new(Empty, Empty),
                 _  => panic!()

            });
        }

        y.push(x);
    }

    (start, y)
}

fn start_direction(start: (usize, usize), map: &Vec<Vec<Pipe>>) -> Direction {
    if map[start.0][start.1 - 1].directions.contains(&Right) {
        return Left
    };
    if map[start.0][start.1 + 1].directions.contains(&Left) {
        return Right
    };
    if map[start.0 - 1][start.1].directions.contains(&Down) {
        return Up
    };
    if map[start.0 + 1][start.1].directions.contains(&Up) {
        return Down
    };
    panic!()
}

fn find_pipe_length(start: (usize, usize), map: Vec<Vec<Pipe>>) -> Option<usize> {

    let mut direction = start_direction(start, &map);
    let mut position = start;
    for i in 0..map.len().pow(2) {
        println!("{}: {:?} -> {:?} ({:?})", i, position, direction, map[position.0][position.1]);
        position = match direction {
            Left  => (position.0, position.1 - 1),
            Right => (position.0, position.1 + 1),
            Up    => (position.0 - 1, position.1),
            Down  => (position.0 + 1, position.1),
            Any   => return Some(i + 1),
            Empty  => return None
        };

        let mut directions = map[position.0][position.1].directions.clone();
        let opposite = direction.opposite();
        for j in 0..2 {
            if directions[j] == opposite {
                directions.remove(j);
                break;
            };
        };

        direction = directions[0].clone();
    }

    return None
}