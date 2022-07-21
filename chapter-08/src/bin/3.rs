// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”.
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn main() {
	let mut employees = HashMap::new();

	loop {
		// parse input
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("error reading input");

		// split words
		let mut words: Vec<&str> = Vec::new();
		for word in input.split_whitespace() {
			words.push(word);
		}

		let command = words[0];
		match command {
			// create string from &str
			// input and words will be dropped at the end of the loop
			"Add" => {
				employees.insert(String::from(words[1]), String::from(words[3]));
			}
			"List" => {
				let mut v = Vec::new();

				for (employee, department) in employees.iter() {
					if department == words[1] {
						v.push(employee);
					}
				}
				v.sort();
				for employee in v {
					println!("{}", employee);
				}
			}
			"Quit" => {
				break;
			}
			_ => continue,
		}
	}
}
