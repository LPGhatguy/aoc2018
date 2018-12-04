use std::cmp::Ordering;
use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RowKind {
    FallAsleep,
    WakeUp,
    Begin(u32),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Row {
    time: Time,
    kind: RowKind,
}

fn parse_row(input: &str) -> Row {
    let time = Time {
        month: input[6..8].parse().unwrap(),
        day: input[9..11].parse().unwrap(),
        hour: input[12..14].parse().unwrap(),
        minute: input[15..17].parse().unwrap(),
    };

    let kind = match input.bytes().nth(19).unwrap() {
        b'f' => RowKind::FallAsleep,
        b'w' => RowKind::WakeUp,
        b'G' => {
            let remaining = &input[26..];
            let end = remaining.find(" ").unwrap();

            RowKind::Begin(remaining[..end].parse().unwrap())
        },
        v => {
            panic!("ahh {}", v as char);
        },
    };

    Row {
        time,
        kind,
    }
}

struct SleepInfo {
    total: u16,
    per_minute: [u16; 60],
}

fn part_one() {
    let mut rows: Vec<_> = INPUT.lines().map(parse_row).collect();

    let mut sleep_time = HashMap::new();
    let mut guard = None;

    let mut iter = rows.iter();

    loop {
        let row = match iter.next() {
            Some(v) => v,
            None => break,
        };

        match row.kind {
            RowKind::Begin(id) => {
                guard = Some(id);
            },
            RowKind::FallAsleep => {
                let id = guard.unwrap();
                let asleep = row.time;
                let awake = iter.next().unwrap();

                println!("asleep: {:?}", row);
                println!("awake: {:?}", awake);

                let time = awake.time.minute - asleep.minute;

                let sleep_info = match sleep_time.get_mut(&id) {
                    Some(v) => v,
                    None => {
                        let info = SleepInfo {
                            total: 0,
                            per_minute: [0; 60],
                        };
                        sleep_time.insert(id, info);
                        sleep_time.get_mut(&id).unwrap()
                    },
                };

                sleep_info.total += time;
                for i in row.time.minute..awake.time.minute {
                    sleep_info.per_minute[i as usize] += 1;
                }
            },
            _ => unimplemented!()
        }
    }

    let mut times = sleep_time.iter().collect::<Vec<_>>();
    times.sort_by_key(|v| v.1.total);

    let mut highest_value = 0;
    let mut highest_key = 0;

    for (i, v) in (&times[times.len() - 1].1.per_minute[..]).iter().enumerate() {
        if *v > highest_value {
            highest_key = i;
            highest_value = *v;
        }
    }

    println!("minute {}", highest_key as u32 * *times[times.len() - 1].0);
    // println!("{}", times[times.len() - 1].1.total);
    // println!("{:?}", times);
}

fn main() {
    part_one();
}
