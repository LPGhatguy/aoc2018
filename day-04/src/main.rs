use std::collections::HashMap;

static INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
struct Time {
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

#[derive(Debug)]
enum RowKind {
    FallAsleep,
    WakeUp,
    Begin(u32),
}

#[derive(Debug)]
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
            panic!("this isn't happening, right? {}", v as char);
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

fn main() {
    // assumes rows are already sorted; used "sort lines" (f9) in Sublime Text
    let rows: Vec<_> = INPUT.lines().map(parse_row).collect();

    let mut sleep_info_per_guard = HashMap::new();
    let mut mutating_guard = None;

    let mut iter = rows.iter();

    loop {
        let row = match iter.next() {
            Some(v) => v,
            None => break,
        };

        match row.kind {
            RowKind::Begin(id) => {
                mutating_guard = Some(id);
            },
            RowKind::FallAsleep => {
                let id = mutating_guard.unwrap();
                let asleep = row.time;
                let awake = iter.next().unwrap();

                let time = awake.time.minute - asleep.minute;

                let sleep_info = match sleep_info_per_guard.get_mut(&id) {
                    Some(v) => v,
                    None => {
                        let info = SleepInfo {
                            total: 0,
                            per_minute: [0; 60],
                        };
                        sleep_info_per_guard.insert(id, info);
                        sleep_info_per_guard.get_mut(&id).unwrap()
                    },
                };

                sleep_info.total += time;
                for i in row.time.minute..awake.time.minute {
                    sleep_info.per_minute[i as usize] += 1;
                }
            },
            _ => panic!("that row ain't right"),
        }
    }

    let mut times = sleep_info_per_guard.iter().collect::<Vec<_>>();
    times.sort_by_key(|v| v.1.total);

    let mut highest_value = 0;
    let mut highest_key = 0;

    let most_sleepy_guard = times[times.len() - 1];
    for (minute, asleep_time) in (&most_sleepy_guard.1.per_minute[..]).iter().enumerate() {
        if *asleep_time > highest_value {
            highest_key = minute;
            highest_value = *asleep_time;
        }
    }

    println!("part one: {}", highest_key as u32 * most_sleepy_guard.0);

    let mut highest_guard = 0;
    let mut highest_value = 0;
    let mut highest_key = 0;

    for (id, sleep_info) in &times {
        for (i, v) in (&sleep_info.per_minute[..]).iter().enumerate() {
            if *v > highest_value {
                highest_guard = **id;
                highest_key = i;
                highest_value = *v;
            }
        }
    }

    println!("part two: {}", highest_guard as usize * highest_key);
}
