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

fn main() {
    let claims: Vec<_> = INPUT.lines().map(parse_claim).collect();

    let mut occupancy: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    for Claim { position, size, id } in &claims {
        for x in position.0..(position.0 + size.0) {
            for y in position.1..(position.1 + size.1) {
                if let Some((existing_id, claims)) = occupancy.get(&(x, y)) {
                    occupancy.insert((x, y), (*existing_id, claims + 1));
                } else {
                    occupancy.insert((x, y), (*id, 1));
                }
            }
        }
    }

    'outer: for Claim { position, size, id } in &claims {
        for x in position.0..(position.0 + size.0) {
            for y in position.1..(position.1 + size.1) {
                if let Some((existing_id, claims)) = occupancy.get(&(x, y)) {
                    if *existing_id != *id || *claims != 1 {
                        continue 'outer;
                    }
                }
            }
        }

        println!("Part two: {}", id);
    }

    let count = occupancy.values().filter(|(_, v)| *v > 1).count();

    println!("Part one: {}", count);
}
