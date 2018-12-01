use std::collections::HashSet;

static INPUT: &str = include_str!("../input.txt");

fn part_one() {
    let result = INPUT
        .split("\n")
        .map(str::trim_right)
        .map(|v| v.parse::<i32>().unwrap())
        .fold(0i32, |a, b| a + b);

    println!("part one: {:?}", result);
}

fn part_two() {
    let changes = INPUT
        .split("\n")
        .map(str::trim_right)
        .map(|v| v.parse::<i32>().unwrap())
        .cycle();

    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();

    seen_frequencies.insert(frequency);

    for change in changes {
        frequency += change;

        if !seen_frequencies.insert(frequency) {
            println!("part two: {:?}", frequency);
            break;
        }
    }
}

fn main() {
    part_one();
    part_two();
}
