static INPUT: &str = include_str!("../input.txt");

fn should_react(a: u8, b: u8) -> bool {
    a - 32 == b || a + 32 == b
}

fn react(input: &mut String) {
    let mut index = 0;

    loop {
        let a = input.as_bytes()[index];
        let b = input.as_bytes()[index + 1];

        if should_react(a, b) {
            let mut remove_start = index;
            let mut remove_end = index + 1;

            input.replace_range(remove_start..=remove_end, "");

            if index > 0 {
                index -= 1;
            }
        } else {
            index += 1;
        }

        if index >= input.len() - 2 {
            break;
        }
    }
}

fn main() {
    let mut input = INPUT.to_string();
    react(&mut input);

    println!("Part one: {} units", input.len());
}

#[test]
fn reacting() {
    assert!(should_react(b'A', b'a'));
    assert!(should_react(b'a', b'A'));
    assert!(!should_react(b'A', b'A'));
    assert!(!should_react(b'a', b'a'));
}