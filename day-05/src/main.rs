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
            input.replace_range(index..=(index + 1), "");

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

    let mut lowest_length = INPUT.len();

    for first_byte in b'A'..b'Z' {
        let second_byte = first_byte + 32;

        let mut test_input = INPUT.replace(first_byte as char, "").replace(second_byte as char, "");
        react(&mut test_input);

        if test_input.len() < lowest_length {
            lowest_length = test_input.len();
        }
    }

    println!("Part two: {}", lowest_length);
}

#[test]
fn reacting() {
    assert!(should_react(b'A', b'a'));
    assert!(should_react(b'a', b'A'));
    assert!(!should_react(b'A', b'A'));
    assert!(!should_react(b'a', b'a'));
}