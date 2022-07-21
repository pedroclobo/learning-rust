use std::io;

fn main() {
	// read input
	let mut num = String::new();
	io::stdin()
		.read_line(&mut num)
		.expect("error reading input");

	let num: i32 = match num.trim().parse() {
		Ok(num) => num,
		Err(_) => 0,
	};

	println!("Temperature: {}ºC", fahrenheit_to_celsius(num));
}

fn fahrenheit_to_celsius(f: i32) -> i32 {
	((f - 32) * 5) / 9
}
