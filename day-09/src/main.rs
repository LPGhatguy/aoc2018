static INPUT: &str = include_str!("../input.txt");

fn modulo(a: isize, b: isize) -> usize {
	((a % b + b) % b) as usize
}

#[derive(Debug)]
struct Node {
	prev: usize,
	next: usize,
	value: usize,
}

fn main() {
	let player_count = 411;
	let last_marble_value = 72059 * 100;

	let mut player_scores = vec![0; player_count];
	let mut active_player = 0;
	let mut marbles = vec![Node {
		prev: 0,
		next: 0,
		value: 0,
	}];
	let mut current_marble = 0;

	for marble in 1..=last_marble_value {
		if marble % 23 == 0 {
			player_scores[active_player] += marble;

			let mut remove_index = current_marble;
			for _ in 0..7 {
				remove_index = marbles[remove_index].prev;
			}

			let Node { value, prev, next } = marbles[remove_index as usize];

			marbles[prev].next = next;
			marbles[next].prev = prev;

			player_scores[active_player] += value;
			current_marble = next;

			// println!("Player {} got {} and {}", active_player, marble, value);
		} else {
			let insert_after = marbles[current_marble].next;
			let insert_before = marbles[insert_after].next;
			let new_node = Node {
				prev: insert_after,
				next: insert_before,
				value: marble,
			};
			marbles.push(new_node);
			marbles[insert_after].next = marbles.len() - 1;
			marbles[insert_before].prev = marbles.len() - 1;

			current_marble = marbles.len() - 1;
		}

		active_player = (active_player + 1) % player_count;
		// println!("{}: {:?}", current_marble, marbles);
	}

	// println!("{:?}", player_scores);
	player_scores.sort();
	println!("{}", player_scores[player_scores.len() - 1]);
}
