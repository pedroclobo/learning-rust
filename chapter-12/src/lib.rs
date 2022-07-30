use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub ignore_case: bool,
}

// this will make a full copy of the data for the Config instance to own,
// which takes more time and memory than storing a reference to the string data
// however, cloning the data also makes our code very straightforward because
// we don’t have to manage the lifetimes of the references
// in this circumstance, giving up a little performance to gain simplicity
// is a worthwhile trade-off
impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		// is_ok returns false if the environment variable isn't set
		let ignore_case = env::var("IGNORE_CASE").is_ok();

		Ok(Config {
			query,
			filename,
			ignore_case,
		})
	}
}

// Box<dyn Error> returns a type that implements Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search(&config.query, &contents)
	};

	for line in results {
		println!("{}", line);
	}

	Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
}

// Test Driven Development, TDD:
// 1. write a test that fails and run it to make sure it fails for the reason you expect
// 2. write or modify just enough code to make the new test pass
// 3. refactor the code you just added or changed and make sure the tests continue to pass
// 4. repeat from step 1
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
