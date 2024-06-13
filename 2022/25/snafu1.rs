use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct SNAFU(Vec<SnafuDigit>);

impl SNAFU {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn from_str(string: &str) -> Self {
        let vec = Vec::new();
        for letter in string.chars() {
            vec.insert(0, match letter {
                '2' => SnafuDigit::Two,
                '1' => SnafuDigit::One,
                '0' => SnafuDigit::Zero,
                '-' => SnafuDigit::Minus,
                '=' => SnafuDigit::DoubleMinus,
                 _  => panic!()
            })
        }
        Self(vec)
    }

    fn to_string(&self) -> String {
        let chars = Chars();


    }
}

impl Add for SNAFU {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let SNAFU(A) = self;
        let SNAFU(B) = other;

        let r = SnafuDigit::Zero;
        for (i, (a, b)) in A.zip(B).enumerate().rev() {
            a + b + r
        }
    }

}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn parse_file(reader: <BufReader<File>>) -> Vec<SNAFU> {}