use std::{i32, u32};
use std::collections::HashSet;

static INPUT: &str = include_str!("../input.txt");

fn manhatten(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn get_bounding_box(mines: &[(i32, i32)]) -> ((i32, i32), (i32, i32)) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for mine in mines {
        min_x = min_x.min(mine.0);
        min_y = min_y.min(mine.1);
        max_x = max_x.max(mine.0);
        max_y = max_y.max(mine.1);
    }

    ((min_x, min_y), (max_x, max_y))
}

fn closest_mine(point: (i32, i32), mines: &[(i32, i32)]) -> Option<usize> {
    let mut min_index = None;
    let mut min_distance = u32::MAX;

    for (index, mine) in mines.iter().enumerate() {
        let distance = manhatten(point, *mine);

        if distance == min_distance {
            min_index = None;
        } else if distance < min_distance {
            min_index = Some(index);
            min_distance = distance;
        }
    }

    min_index
}

fn get_area(mines: &[(i32, i32)], index: usize) -> Option<u32> {
    let (start_x, start_y) = mines[index];
    let mut to_visit = vec![
        (start_x + 1, start_y),
        (start_x - 1, start_y),
        (start_x, start_y + 1),
        (start_x, start_y - 1),
    ];

    let mut visited = HashSet::new();
    for coord in &to_visit {
        visited.insert(*coord);
    }

    let mut area = 0;

    println!("finding {}", index);
    while let Some((x, y)) = to_visit.pop() {
        if area >= 160_000 {
            println!("OUT OF BOUNDS");
            return None;
        }

        match closest_mine((x, y), mines) {
            Some(closest_index) => {
                if closest_index == index {
                    area += 1;

                    for next in &[
                        (x + 1, y),
                        (x - 1, y),
                        (x, y + 1),
                        (x, y - 1),
                    ] {
                        if !visited.contains(next) {
                            visited.insert(*next);
                            to_visit.push(*next);
                        }
                    }
                }
            },
            _ => {},
        }
    }

    Some(area)
}

fn main() {
    let mines: Vec<(i32, i32)> = INPUT
        .lines()
        .map(|v| {
            let mut iter = v.split(", ");
            (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
        })
        .collect();

    let mut max_area = 0;

    for (index, _) in mines.iter().enumerate() {
        match get_area(&mines, index) {
            Some(area) => {
                max_area = max_area.max(area);
            },
            None => {},
        }
    }

    println!("part one: {}", max_area);

    // println!("Area of 0 is {}", get_area(&mines, 0));

    // println!("{:?}", closest_mine((202, 254), &mines));
    // println!("{} are eligable, out of {}", eligable.len(), mines.len());
}
