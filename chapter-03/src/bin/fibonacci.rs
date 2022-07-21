use std::collections::HashMap;
use std::io;

fn main() {
	// read input
	let mut num = String::new();
	io::stdin()
		.read_line(&mut num)
		.expect("error reading input");

	let num: u128 = match num.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	println!("Number: {}.", fibonnaci(num));
}

// O(2^n)
//fn fibonnaci(n: u128) -> u128 {
//    if n == 0 {
//		0
//	} else if n == 1 {
//		1
//	} else {
//		fibonnaci(n - 1) + fibonnaci(n - 2)
//	}
//}

// O(n)
fn fibonnaci(n: u128) -> u128 {
	let mut mem: HashMap<u128, u128> = HashMap::new();
	mem.insert(0, 0);
	mem.insert(1, 1);
	fibonnaci_aux(n, &mut mem)
}

fn fibonnaci_aux(n: u128, mem: &mut HashMap<u128, u128>) -> u128 {
	let res = match mem.get(&n) {
		Some(res) => *res,
		None => fibonnaci_aux(n - 1, mem) + fibonnaci_aux(n - 2, mem),
	};
	mem.insert(n, res);
	res
}
