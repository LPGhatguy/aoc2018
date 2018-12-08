static INPUT: &str = include_str!("../input.txt");

struct Node {
	children: Vec<Node>,
	metadata: Vec<u32>,
	value: u32,
}

fn parse_node<T: Iterator<Item = u32>>(input: &mut T, sum: &mut u32) -> Node {
	let num_children = input.next().unwrap();
	let num_metadata = input.next().unwrap();

	let mut children = Vec::new();
	let mut metadata = Vec::new();

	for _ in 0..num_children {
		let child = parse_node(input, sum);
		children.push(child);
	}

	for _ in 0..num_metadata {
		let meta = input.next().unwrap();
		*sum += meta;
		metadata.push(meta);
	}

	let value = if children.is_empty() {
		metadata.iter().sum()
	} else {
		let mut value = 0;
		for meta in &metadata {
			if let Some(child) = children.get(*meta as usize - 1) {
				value += child.value;
			}
		}
		value
	};

	Node { children, metadata, value }
}

fn main() {
	let numbers: Vec<u32> = INPUT
		.split(" ")
		.map(str::parse)
		.map(Result::unwrap)
		.collect();

	let mut sum = 0;
	let root = parse_node(&mut numbers.iter().cloned(), &mut sum);

    println!("Part one: {:?}", sum);
    println!("Part two: {}", root.value);
}
