// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here)
// of the list.
use std::collections::HashMap;
use std::io;

fn median(v: &mut Vec<i32>) -> i32 {
	v.sort();
	v[v.len() / 2]
}

fn mode(v: &Vec<i32>) -> i32 {
	let mut occ = HashMap::new();

	for &i in v {
		let count = occ.entry(i).or_insert(0);
		*count += 1;
	}

	let mut i_max = v[0];
	let mut count_max = 0;
	for (i, count) in occ {
		if count > count_max {
			count_max = count;
			i_max = i;
		}
	}

	i_max
}

fn main() {
	let mut v: Vec<i32> = Vec::new();

	// parse input
	let mut s = String::new();
	print!("Enter a list of numbers: ");
	io::stdin().read_line(&mut s).expect("error reading input");

	for num in s.split_whitespace() {
		v.push(num.parse().expect("list can only hold numbers"));
	}

	println!(
		"The median is {} and the mode is {}.",
		median(&mut v),
		mode(&v)
	);
}
