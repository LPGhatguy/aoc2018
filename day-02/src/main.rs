static INPUT: &str = include_str!("../input.txt");

fn count_duplicate_leters(value: &str) -> [u32; 26] {
    value
        .bytes()
        .fold([0; 26], |mut counts, byte| {
            counts[(byte - b'a') as usize] += 1;
            counts
        })
}

fn part_one() {
    let result = INPUT
        .lines()
        .map(count_duplicate_leters)
        .fold((0, 0), |(num_twos, num_threes), counts| {
            let mut has_two = 0;
            let mut has_three = 0;

            for &count in &counts {
                if count == 2 {
                    has_two = 1;
                } else if count == 3 {
                    has_three = 1;
                }
            }

            (num_twos + has_two, num_threes + has_three)
        });

    println!("Part one: {}", result.0 * result.1);
}

fn part_two() {
    let ids: Vec<_> = INPUT.lines().collect();

    for (index_a, a) in ids.iter().enumerate() {
        'inner: for (index_b, b) in ids.iter().enumerate() {
            if index_a == index_b {
                continue;
            }

            let mut differing_index = None;

            for (index, (byte_a, byte_b)) in a.bytes().zip(b.bytes()).enumerate() {
                if byte_a != byte_b {
                    if differing_index.is_some() {
                        continue 'inner;
                    }

                    differing_index = Some(index);
                }
            }

            let differing_index = differing_index.unwrap();

            println!("Part two: {}{}", &a[0..differing_index], &a[differing_index + 1..]);
            return;
        }
    }
}

fn main() {
    part_one();
    part_two();
}
