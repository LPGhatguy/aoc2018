extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

use regex::Regex;

static INPUT: &str = include_str!("../input.txt");

lazy_static! {
    static ref BOX: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}

#[derive(Debug)]
struct Claim {
    id: u32,
    position: (u32, u32),
    size: (u32, u32),
}

fn parse_claim(input: &str) -> Claim {
    let captures = BOX.captures(input).unwrap();

    Claim {
        id: captures[1].parse().unwrap(),
        position: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
        size: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
    }
}

fn part_one() {
    let claims: Vec<_> = INPUT.lines().map(parse_claim).collect();

    let mut occupancy: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    for claim in &claims {
        for x in claim.position.0..(claim.position.0 + claim.size.0) {
            for y in claim.position.1..(claim.position.1 + claim.size.1) {
                if let Some((id, claims)) = occupancy.get(&(x, y)) {
                    occupancy.insert((x, y), (*id, claims + 1));
                } else {
                    occupancy.insert((x, y), (claim.id, 1));
                }
            }
        }
    }

    'outer: for claim in &claims {
        for x in claim.position.0..(claim.position.0 + claim.size.0) {
            for y in claim.position.1..(claim.position.1 + claim.size.1) {
                if let Some((id, claims)) = occupancy.get(&(x, y)) {
                    if *id != claim.id || *claims != 1 {
                        continue 'outer;
                    }
                }
            }
        }

        println!("Part two: {}", claim.id);
    }

    let mut count = 0;
    for (_, claims) in occupancy.values() {
        if *claims >= 2 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn part_two() {
}

fn main() {
    part_one();
    part_two();
}
