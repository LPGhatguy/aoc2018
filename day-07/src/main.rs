use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let dependencies: Vec<_> = INPUT
        .lines()
        .map(|line| (line.as_bytes()[5] as char, line.as_bytes()[36] as char))
        .collect();

    let mut prereqs_for_each_step: HashMap<char, HashSet<char>> = HashMap::new();

    for (prereq, step) in &dependencies {
        prereqs_for_each_step.entry(*prereq).or_default();
        let mut for_step = prereqs_for_each_step.entry(*step).or_default();
        for_step.insert(*prereq);
    }

    let mut completed = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();

    for (letter, prereqs) in &prereqs_for_each_step {
        if prereqs.is_empty() {
            to_visit.push(*letter);
            visited.insert(*letter);
        }
    }

    to_visit.sort_by_key(|&letter| Reverse(letter));

    let mut output = Vec::new();

    while let Some(step) = to_visit.pop() {
        completed.insert(step);
        output.push(step);

        for (letter, prereqs) in &prereqs_for_each_step {
            if !visited.contains(letter) && prereqs.difference(&completed).next().is_none() {
                to_visit.push(*letter);
                visited.insert(*letter);
                to_visit.sort_by_key(|&letter| Reverse(letter));
            }
        }
    }

    let result: String = output.iter().collect();
    println!("{}", result);
}
