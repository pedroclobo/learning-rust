#![allow(unused)]

// "Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence." - Dijkstra

// correctness is the extent to which our code does what we expect it to do
// because of that, rust includes support for writing automated software tests

// tests are rust functions that verify that the non-test code is functioning in the
// excepted way
// they:
// - set up any needed data or state
// - run the code we want to test
// - assert the results are what we expect

// a test in rust is a function that is annotated with the test attribute
// to change a function into a test function, we add #[test] on top of it
// cargo test will run the tests
// we can also pass arguments to cargo test to only run tests that match the name
// Doc-tests is for the results of any documentation tests
#[cfg(test)]
mod tests {
	#[test] // tell the test runner this is a test
	fn exploration() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}

	// tests fail when something panics
	#[test]
	fn another() {
		panic!("Make this test fail");
	}
}

// the assert! macro is useful when we want to ensure some condition is true
// the argument evaluates to a boolean
// if true, nothing happens
// if false, calls panic! and the test fails

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

#[cfg(test)]
mod tests_rectangle {
	// we need to bring the code under test in the outer module to the scope
	// of the inner module
	use super::*;

	#[test]
	fn larger_can_hold_smaller() {
		let larger = Rectangle {
			width: 8,
			height: 7,
		};
		let smaller = Rectangle {
			width: 5,
			height: 1,
		};

		assert!(larger.can_hold(&smaller));
	}

	#[test]
	fn smaller_cannot_hold_larger() {
		let larger = Rectangle {
			width: 8,
			height: 7,
		};
		let smaller = Rectangle {
			width: 5,
			height: 1,
		};

		assert!(!smaller.can_hold(&larger));
	}
}

// the macros assert_eq! and assert_ne! test two arguments for equality or inequality
// they also print the two values if the assertion fails, which is great
// this macros print their arguments in debug format, if the assertion fails
// the values being compared must implement the PartialEq and Debug traits
// this traits are usually derivable with #[derive(PartialEq, Debug)]
pub fn add_two(a: i32) -> i32 {
	a + 3
}

#[cfg(test)]
mod tests_new {
	use super::*;

	#[test]
	fn it_adds_two() {
		assert_eq!(4, add_two(2));
	}
}

pub fn greeting(name: &str) -> String {
	String::from("Hello!")
}

// we can add custom failure messages
#[test]
fn greeting_contains_name() {
	let result = greeting("Carol");
	// extra arguments are passed to the format! macro
	assert!(
		result.contains("Carol"),
		"Greeting did not contain name, value was `{}`",
		result
	);
}

// we can also check if our code handles error conditions as expected
pub struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}

		Guess { value }
	}
}

// should_panic tests are unreliable
// a test can pass if the code panics for another reason
// an expect parameter will contain a string that the failure message must contain
#[cfg(test)]
mod tests_guess {
	use super::*;

	#[test]
	// the should_panic attribute makes the test pass if the function panics
	// the test fails if the function doesn't panic
	// now the test will only pass if the failure message contains the specified string
	#[should_panic(expected = "Guess value must be less than or equal to 100")]
	fn greater_than_100() {
		Guess::new(200);
	}
}

// we can use Result<T, E> in tests too
// this tests will return an Err instead of panicking
// we can use the ? operator
// we can't use the #[should_panic] attribute
// instead, to assert we use assert!(value.is_err())
#[cfg(test)]
mod tests_result {
	#[test]
	fn it_works() -> Result<(), String> {
		if 2 + 2 == 4 {
			Ok(())
		} else {
			Err(String::from("two plus two does not equal four"))
		}
	}
}

// the default behaviour of cargo test in to produce a binary that runs all tests in parallel
// it also captures the output generated and prevents it from being displayed
// however, we can change this behaviour
// some of the command line options go to cargo and others go to the test binary
// the first arguments go to cargo and the arguments after -- go to the binary

// the following will make cargo not use any parallelism
// cargo test -- --test-threads=1

// be default, only failed tests print the output
// the following will show print values for passing tests as well
// cargo test -- --show-output

// we may want to run tests related to a certain code
// the following will only run the specified tests
// cargo test <name-of-test>

// the following will only run test that contain substring in their names
// cargo test <substring>

// the following will run the tests contained in the module
// cargo test <module>

#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}

#[test]
// ignore will tell cargo test to ignore this test
// we can run only the ignored test with cargo test -- --ignored
// to run all tests: cargo test -- --include-ignored
#[ignore]
fn expensive_test() {
	// code that takes an hour to run
}

// tests have 2 main categories:
// - unit tests: small and focused test that test a single part of the code
// - integration tests: tests that are external to our library and potentially test the whole codebase

// unit tests are placed in the src directory in each file that contains the code to be tested
// the convention is to create a module named tests and annotate it with cfg(test)
// the cfg(test) annotation tells rust to compile and run the test code only when we run cargo test

// rust allows us to test private functions
pub fn add_two_1(a: i32) -> i32 {
	internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
	a + b
}

#[cfg(test)]
mod tests_1 {
    // bring parent items into scope
    // we can now call internal_adder
	use super::*;

	#[test]
	fn internal() {
		assert_eq!(4, internal_adder(2, 2));
	}
}

// integration tests are placed inside the tests directory
// cargo will compile each of the files as an individual create
// we need to bring our library into scope
// we don't need the #[cfg(test)] annotation because cargo treats the tests directory specialy
// to run a particular test file, we use cargo test --test <filename>
// inside tests/integration_test.rs:
/*
user adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
*/

// in integration tests, we can create a tests/common.rs and place a function named setup in it
// this function can me balled from multiple test functions in multiple test files
// for the common test to not show up in cargo test, we can move it to tests/common/mod.rs
// we then call it inside other integration tests like so:
/*
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
*/

// if our project is a binary and doesn't contain a src/lib.rs, we cannot place integration
// inside the tests directory
