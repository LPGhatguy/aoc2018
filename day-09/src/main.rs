#[derive(Clone, Copy)]
struct Node {
	prev: usize,
	next: usize,
	value: usize,
}

fn play_game(player_count: usize, last_marble_value: usize) -> usize {
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

			let removed_marble = marbles[remove_index as usize];
			marbles[removed_marble.prev].next = removed_marble.next;
			marbles[removed_marble.next].prev = removed_marble.prev;

			player_scores[active_player] += removed_marble.value;
			current_marble = removed_marble.next;

			// For diagnostics:
			// println!("Player {} got {} and {}", active_player, marble, value);
		} else {
			let insert_after = marbles[current_marble].next;
			let insert_before = marbles[insert_after].next;

			let new_marble_index = marbles.len();
			marbles.push(Node {
				prev: insert_after,
				next: insert_before,
				value: marble,
			});

			marbles[insert_after].next = new_marble_index;
			marbles[insert_before].prev = new_marble_index;

			current_marble = new_marble_index;
		}

		active_player = (active_player + 1) % player_count;
	}

	player_scores.sort();
	player_scores[player_scores.len() - 1]
}

fn main() {
	println!("Part one: {}", play_game(411, 72059));
	println!("Part two: {}", play_game(411, 72059 * 100));
}
