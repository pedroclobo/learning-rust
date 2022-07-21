#![allow(unused)]

// errors will always occur
// in many cases, rust enforces us to take an action and fix an error at compile time
// errors can be recoverable or not recoverable
// in recoverable errors like 'file not found' we would report the problem to the user
// and retry the operation
// in non-recoverable errors like 'divide by zero' we would stop the program
// rust doesn't have exceptions
// it deals with recoverable errors with Result<T, E>
// and panic! macro for non-recoverable errors

// the panic! macro will print a failure message, clean up the stack and quit
// we could also skip the clean up with 'panic = 'abort' in the Cargo.toml
fn non_recoverable_error() {
	panic!("crash and burn");
}

// we can run the program with 'RUST_BACKTRACE=1' to backtrace the error

// Result<T, E>
// T and E are generic parameters
// T represents the success type
// E represents the error type
fn recoverable_error() {
	use std::fs::File;
	use std::io::ErrorKind;

	// Result<std::fs::File, std::io::Error>
	let f = File::open("hello.txt");

	// we can match on different errors
	let f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => match File::create("hello.txt") {
				Ok(fc) => fc,
				Err(e) => panic!("Problem creating the file: {:?}", e),
			},
			other_error => {
				panic!("Problem opening the file: {:?}", other_error)
			}
		},
	};

	// other alternative to match
	let f = File::open("hello.txt").unwrap_or_else(|error| {
		if error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|error| {
				panic!("Problem creating the file: {:?}", error);
			})
		} else {
			panic!("Problem opening the file: {:?}", error);
		}
	});

	// match can sometimes be really verbose
	// unwrap is a shortcut just like the match expression
	// if the result is Ok, it will return the value inside the Ok
	// if the result is Err, it will panic
	let f = File::open("hello.txt").unwrap();

	// we could also use expect to write our error message
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// we can also propagate errors
// instead of handling the error within the function itself,
// we can return the error to the calling code
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
	use std::fs::File;

	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e), // we return the error
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

// error propagation is so common that rust provides the ? operator
// the ? will work like the match we defined above
// however error values with the ? operator called on them go through
// the 'from' function, which will convert the error to the type specified in
// the return type of the function
// there must be a impl From<OtherError> for ReturnedError
// we can only use ? when the return type is appropriate
fn read_username_from_file_with_operator() -> Result<String, io::Error> {
	use std::fs::File;
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

// we can even chain the ? operator
fn read_username_from_file_chained() -> Result<String, io::Error> {
	use std::fs::File;
	let mut s = String::new();

	File::open("hello.txt")?.read_to_string(&mut s)?;

	Ok(s)
}

// we could also use the standard library's convenient function
fn read_username_from_file_std() -> Result<String, io::Error> {
	use std::fs;
	fs::read_to_string("hello.txt")
}

// ? can also be used with Option<T> in functions that return Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
	text.lines().next()?.chars().last()
}

// fn main() can also return a Result<(), E>
// Box<dyn Error> denotes any type of error
// the executable will exit with error code 0 if main returns Ok(())
// and will exit with error code 1 if main returns Err(())
// fn main() can return any types that implement std::process::Termination
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
	use std::fs::File;
	let f = File::open("hello.txt")?;

	Ok(())
}

// when to return a Result and when to panic?
// returning Result is a good default choice when we’re defining a function that might fail
// in prototype code or tests it's more appropriate to panic

// it is appropriate to use unwrap when we know the function will not fail
fn will_not_fail() {
	use std::net::IpAddr;

	// will certainly not fail
	let home: IpAddr = "127.0.0.1".parse().unwrap();
}

// guidelines
// we should panic if it is possible for the function to fail
// when a failure is expected, we should return a Result

// how to ensure the guess number is between 1 and 100 in the guessing game?
// if statements all over the code would be tedious
fn guessing_game() {
	let guess = String::new();
	let secret_number = 42;

	loop {
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		if guess < 1 || guess > 100 {
			println!("The secret number will be between 1 and 100.");
			continue;
		}

		match guess.cmp(&secret_number) {
			std::cmp::Ordering::Less => todo!(),
			std::cmp::Ordering::Equal => todo!(),
			std::cmp::Ordering::Greater => todo!(),
		}
	}
}

// we could create a new type and put the validations in it
pub struct Guess {
	value: i32, // value must be private to prevent outside code from mutating it
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}

		Guess { value }
	}

	// getter
	pub fn value(&self) -> i32 {
		self.value
	}
}
