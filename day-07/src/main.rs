use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

static INPUT: &str = include_str!("../input.txt");

fn part_one() {
    let dependencies: Vec<_> = INPUT
        .lines()
        .map(|line| (line.as_bytes()[5] as char, line.as_bytes()[36] as char))
        .collect();

    let mut prereqs_for_each_step: HashMap<char, HashSet<char>> = HashMap::new();

    for (prereq, step) in &dependencies {
        prereqs_for_each_step.entry(*prereq).or_default();
        let for_step = prereqs_for_each_step.entry(*step).or_default();
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
    println!("part one: {}", result);
}

fn part_two() {
    let dependencies: Vec<_> = INPUT
        .lines()
        .map(|line| (line.as_bytes()[5] as char, line.as_bytes()[36] as char))
        .collect();

    let mut prereqs_for_each_step: HashMap<char, HashSet<char>> = HashMap::new();

    for (prereq, step) in &dependencies {
        prereqs_for_each_step.entry(*prereq).or_default();
        let for_step = prereqs_for_each_step.entry(*step).or_default();
        for_step.insert(*prereq);
    }

    let mut completed: HashSet<char> = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();

    for (letter, prereqs) in &prereqs_for_each_step {
        if prereqs.is_empty() {
            to_visit.push(*letter);
            visited.insert(*letter);
        }
    }

    to_visit.sort_by_key(|&letter| Reverse(letter));

    let mut workers = vec![None; 5];
    let mut output: Vec<char> = Vec::new();
    let mut current_time: u32 = 0;

    loop {
        for (index, worker) in workers.iter_mut().enumerate() {
            if let Some((completion_time, step)) = worker {
                if current_time >= *completion_time {
                    println!("{}: Worker {} finished {}", current_time, index, step);
                    completed.insert(*step);
                    output.push(*step);

                    for (letter, prereqs) in &prereqs_for_each_step {
                        if !visited.contains(letter) && prereqs.difference(&completed).next().is_none() {
                            to_visit.push(*letter);
                            visited.insert(*letter);
                            to_visit.sort_by_key(|&letter| Reverse(letter));
                        }
                    }

                    *worker = None;
                }
            }
        }

        for (index, worker) in workers.iter_mut().enumerate() {
            if worker.is_none() {
                if let Some(next_step) = to_visit.pop() {
                    let cost = (next_step as u32) - 64 + 60;

                    let completion = current_time + cost;
                    println!("{}: Worker {} taking {}, done at {}", current_time, index, next_step, completion);
                    *worker = Some((completion, next_step));
                }
            }
        }

        let idle_count = workers.iter().filter(|v| v.is_none()).count();

        if idle_count == workers.len() {
            break;
        }

        current_time += 1;
    }

    let result: String = output.iter().collect();
    println!("result: {}", result);
    println!("total time: {}", current_time);
}

fn main() {
    part_one();
    part_two();
}