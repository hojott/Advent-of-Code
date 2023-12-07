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
    let mut numbers = [0; 14];
    for (i, char) in raw_hand.chars().enumerate() {
        match char {
            char if char.is_digit(10) => {
                let num: usize = char.to_digit(10).unwrap()
                    .try_into().unwrap();
                hand[i] = num.try_into().unwrap();
                numbers[num-1] += 1;
            },
            'T' => {
                let num: usize = 10;
                hand[i] = num.try_into().unwrap();
                numbers[num-1] += 1;
            },
            'J' => {
                let num: usize = 1;
                hand[i] = num.try_into().unwrap();
                let vec = numbers.iter().map(|x| x + 1).collect::<Vec<u32>>();
                numbers = vec.try_into().unwrap();
            },
            'Q' => {
                let num: usize = 12;
                hand[i] = num.try_into().unwrap();
                numbers[num-1] += 1;
            },
            'K' => {
                let num: usize = 13;
                hand[i] = num.try_into().unwrap();
                numbers[num-1] += 1;
            },
            'A' => {
                let num: usize = 14;
                hand[i] = num.try_into().unwrap();
                numbers[num-1] += 1;
            },
            _ => panic!()
        };
    };

    let jokers = numbers[0];
    let mut joked = false;
    numbers.sort_by(|a, b| b.cmp(&a));
    let mut kind = HandKind::HighCard;
    for amount in numbers {
        let new_amount = if joked {
            amount - jokers
        } else {
            amount
        };
        match new_amount {
            5 => {
                kind = HandKind::FiveKind;
                break;
            },
            4 => {
                kind = HandKind::FourKind;
                break;
            },
            3 => {
                kind = HandKind::ThreeKind;
                joked = true;
            },
            2 => {
                match kind {
                    HandKind::ThreeKind => {
                        kind = HandKind::FullHouse;
                        break;
                    },
                    HandKind::OnePair => {
                        kind = HandKind::TwoPair;
                        break;
                    },
                    _ => kind = HandKind::OnePair
                };
                joked = true;
            },
            _ => break
        }
    };

    return Hand {kind: kind, hand: hand }
}