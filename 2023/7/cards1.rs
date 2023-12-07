use std::fs::File;
use std::io::{
    prelude::*,
    BufReader
};

use std::cmp::Ordering;

use std::convert::TryInto;

fn main() -> () {
    let reader = load_file("input.txt").unwrap();
    let data = parse_file(reader);

    let mut hands = Vec::new();
    for raw_hand in data {
        let hand = check_hand(&raw_hand.0);
        hands.push((hand, raw_hand.1));
    };

    hands.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut sum: u32 = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        let multiplier: u32 = (i+1).try_into().unwrap();
        sum += multiplier * hand.1
    };

    println!("{}", sum);
}

fn load_file(file_name: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

fn parse_file(reader: BufReader<File>) -> Vec<(String, u32)> {
    
    let mut data = Vec::new();
    for raw_hand_bid in reader.lines() {
        let unwrapped = raw_hand_bid.unwrap();
        let mut split = unwrapped.split(" ");

        let hand = split.nth(0).unwrap()
            .to_string();
        let bid = split.next().unwrap()
            .parse::<u32>().unwrap();

        data.push((hand, bid));
    }

    data
}

#[derive(Debug)]
enum HandKind {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    kind: HandKind,
    hand: [u32; 5],
}

impl Hand {
    fn to_u32(&self) -> u32 {
        match self.kind {
            HandKind::FiveKind => 6,
            HandKind::FourKind => 5,
            HandKind::FullHouse => 4,
            HandKind::ThreeKind => 3,
            HandKind::TwoPair => 2,
            HandKind::OnePair => 1,
            HandKind::HighCard => 0
        }
    }

    fn cmp(&self, hand2: &Hand) -> Ordering {
        match (self, hand2) {
            _ if self.to_u32() == hand2.to_u32() => {
                for (a, b) in self.hand.iter().zip(hand2.hand) {
                    if *a == b {
                        continue
                    } else {
                        return (*a).cmp(&b)
                    }

                };
                return Ordering::Equal
            },
            _ if self.to_u32() > hand2.to_u32() => return Ordering::Greater,
            _ if self.to_u32() < hand2.to_u32() => return Ordering::Less,
            _ => panic!()
        }
    }    
}

fn check_hand(raw_hand: &str) -> Hand {

    let mut hand = [0; 5];
    let mut numbers = [0; 13];
    for (i, char) in raw_hand.chars().enumerate() {
        match char {
            char if char.is_digit(10) => {
                let num: usize = char.to_digit(10).unwrap()
                    .try_into().unwrap();
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            'T' => {
                let num: usize = 10;
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            'J' => {
                let num: usize = 11;
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            'Q' => {
                let num: usize = 12;
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            'K' => {
                let num: usize = 13;
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            'A' => {
                let num: usize = 14;
                hand[i] = num.try_into().unwrap();
                numbers[num-2] += 1;
            },
            _ => panic!()
        };
    };

    let mut kind = HandKind::HighCard;
    let mut two_count = 0;
    let mut three_count = 0;
    for amount in numbers {
        match amount {
            5 => kind = HandKind::FiveKind,
            4 => kind = HandKind::FourKind,
            3 => three_count += 1,
            2 => two_count += 1,
            _ => continue
        }
    };

    let mut hand_struct = Hand {kind: kind, hand: hand };
    match (hand_struct.to_u32(), two_count, three_count) {
        (4..=5, _, _) => { /* ignore */ },
        (_, 2, _) => hand_struct.kind = HandKind::TwoPair,
        (_, 1, 1) => hand_struct.kind = HandKind::FullHouse,
        (_, _, 1) => hand_struct.kind = HandKind::ThreeKind,
        (_, 1, _) => hand_struct.kind = HandKind::OnePair,
        (_, _, _) => { /* ignore */ }
    };

    return hand_struct
}