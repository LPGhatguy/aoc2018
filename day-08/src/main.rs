static INPUT: &str = include_str!("../input.txt");

struct Node {
	children: Vec<Node>,
	metadata: Vec<u32>,
}

fn parse_node<T: Iterator<Item = u32>>(input: &mut T, sum: &mut u32) -> Node {
	let num_children = input.next().unwrap();
	let num_metadata = input.next().unwrap();

	let mut children = Vec::new();
	let mut metadata = Vec::new();

	for _ in 0..num_children {
		children.push(parse_node(input, sum));
	}

	for _ in 0..num_metadata {
		let meta = input.next().unwrap();
		*sum += meta;
		metadata.push(meta);
	}

	Node { children, metadata }
}

fn main() {
	let numbers: Vec<u32> = INPUT
		.split(" ")
		.map(str::parse)
		.map(Result::unwrap)
		.collect();

	let mut sum = 0;
	parse_node(&mut numbers.iter().cloned(), &mut sum);
    println!("Hello, world! {:?}", sum);
}
