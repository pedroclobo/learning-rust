// we want to run the program with
// cargo run <search-string> <file-name>

// separation of concerns for binary projects:
// - split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs
// - as long as your command line parsing logic is small, it can remain in main.rs
// - when the command line parsing logic starts getting complicated, extract it from main.rs
// and move it to lib.rs

// main.rs is responsible for:
// - calling the command line parsing logic with the argument values
// - setting up any other configuration
// - calling a run function in lib.rs
// - handling the error if run returns an error

use std::env;
use std::process;

use chapter_12::Config;

fn main() {
	// return an iterator over the command line arguments
	let args: Vec<String> = env::args().collect();

	// return inner value of Ok
	// else call the code in the closure
	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});

	if let Err(e) = chapter_12::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(1);
	}
}
