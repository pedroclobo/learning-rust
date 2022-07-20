// bring libraries into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	// println! is a macro
	// hence the !
	println!("Guess the number!");

	// generate random number
	let secret_number = rand::thread_rng().gen_range(1..=100);

	// loop until break
	loop {
		println!("Please input your guess.");

		// let assigns a value to a variable
		// variables are immutable by default
		// String::new() returns a new empty String
		// String is growable and UTF-8 encoded
		// :: indicates that new is associated with the type String
		let mut guess = String::new();

		// std::io::stdin()
		// read_line appends to the String
		// & denotes a reference, to access data without copying it
		// read_line returns a Result, an enum with variants Ok and Err
		// Result has an expect method
		// expect crashes the program if an Err is received
		// otherwise it will return the value of Ok
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		// shadow the previous value of guess with a new value
		// parse converts the String to a number
		// : to annotate a variable's type
		// _ is a catch-all value
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		// placeholders
		println!("You guessed: {guess}");

		// Ordering is an enum with variants: Less, Equal, Greater
		// a match expression is made of arms
		// break brakes the loop
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
