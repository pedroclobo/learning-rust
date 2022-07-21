#![allow(unused)]

// ownership
// its a set or rules to manage memory, as opposed to the garbage collector (Java)
// and letting the user mange the memory (C)
// if any of the rules in not met, the program will not compile
// allows for memory safety
// none of the ownership features slow down the program

// rules:
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// scopes define a range of the program where a variable is valid
fn scopes() {
	{
		// s is not valid here, it’s not yet declared
		let s = "hello"; // s is valid from this point forward
	} // this scope is now over, and s is no longer valid
}

// to illustrate the rules of ownership, we consider the String type
// it is mutable and stored on the heap, as opposed to the s variable in the previous example
// string literals, like s, are immutable
// variables on the stack get trivially copied, unlike variables on the heap

// with the String type:
// memory must be requested from the memory allocator at runtime
// we need a way of returning this memory to the allocator when we’re done with the String

// the memory is automatically return when the variable goes out of scope
// when freeing the memory, rust calls drop()
fn dropping() {
	let mut s = String::from("hello"); // allocates memory on heap
	s.push_str(", world!");
} // drop s here, freeing the memory on the heap

// multiple variables can interact with the same data
// rust will never create deep copies of data, automatically
fn moving() {
	// variables on the stack get their value copied
	let x = 5;
	let y = x;

	// heap variables get their value moved
	// String is made up of
	// ptr - pointer to the memory that holds the contents of the String
	// length - memory, in bytes, the contents are using
	// capacity - memory, in bytes, the String has received from the allocator
	// the assignment copies the pointer
	let s1 = String::from("hello");
	let s2 = s1; // s1 has been moved to s2 and s1 is no longer valid
} // drop s2 here

// Copy trait can be place on types stored on the stack
// when implementing this trait, variables do not move and are trivially copied
// a type that implements Copy cannot implment Drop
// the following types implement Copy:
// integer types, such as u32
// boolean type
// floating point types, such as f64
// character type
// tuples, if they only contain types that also implement Copy
// (i32, i32) implements Copy, but (i32, String) does not
fn copying() {
	// variables on the stack get their value copied
	let x = 5;
	let y = x;

	// create deep copy of s1
	let s1 = String::from("hello");
	let s2 = s1.clone(); // expensive operation
}

// works like variable assignment
// passing a variable to a function is a move or copy, just like assignments
fn functions() {
	let s = String::from("hello");

	// s is moved into takes_ownership
	takes_ownership(s);

	let x = 5;

	// x is copied into makes_copy
	makes_copy(x);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}

// returning values can also transfer ownership
fn return_values() {
	// gives_ownership moves its return value into s1
	let s1 = gives_ownership();

	let s2 = String::from("hello");

	// s2 is moved into takes_and_gives_back
	// takes_and_gives_back moves its return value into s3
	let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
	let some_string = String::from("yours");

	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	// a_string is returned and moves out to the calling function
	a_string
}

// what if we want to let a function use a value but not take ownership?
// references address that problem
// they point to an address in memory and we can follow it, even if we don't own the data
// unlike pointers, rust ensures references are always valid
fn borrowing() {
	let s1 = String::from("hello");
	let len = calculate_length(&s1);

	println!("The length of '{}' is {}.", s1, len);
}

// we can access string length using a reference
// however, we can't modify the String
// when functions have references as parameters instead of the actual values,
// we won’t need to return the values in order to give back ownership,
// because we never had ownership
// the act of creating a reference is called borrowing
fn calculate_length(s: &String) -> usize {
	s.len()
	// s will not be dropped because calculate_length doesn't own it
}

// if we want to modify a borrowed value, we need to use a mutable reference
// if there is one mutable reference to a piece of data, we can't have any other references,
// even if they are immutable
// this prevents data races
fn mutable_references() {
	let mut s = String::from("hello"); // s has to be mutable for us to have a mutable reference

	// pass a mutable reference
	change(&mut s);
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}

// rust allows multiple mutable references, just not simultaneous ones
// the scopes in which the references are used can't overlap
fn expection() {
	let mut s = String::from("hello");

	let r1 = &s; // no problem
	let r2 = &s; // no problem
	println!("{} and {}", r1, r2);
	// variables r1 and r2 will not be used after this point

	let r3 = &mut s; // no problem
	println!("{}", r3);
}

// slices are a reference to a contiguous block of an array
// because they are a reference, then don't have ownership

// rather than referencing the whole String,
// we reference just a portion
// String are UTF-8 encoded, so an error occurs if we slice in the middle of a multi-byte character
fn slices() {
	let s = String::from("hello world");
	let hello = &s[..5];
	let world = &s[6..];
	let hello_world = &s[..];
}

// how to write a function that takes a string of words separated by spaces
// and returns the first word it finds in that string?
fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}

// string slices prevent the following error
// recall if we have an immutable reference we can't also have a mutable reference
fn error() {
	let mut s = String::from("hello world");

	// immutable borrow
	let word = first_word(&s);

	// mutable borrow
	//s.clear(); // error!

	println!("the first word is: {}", word);
}

// string literals are string slices
// it's a slice pointing to a point of the binary
fn string_literals() {
	let s = "Hello, world!"; // s is &str
}

// first_word could be written as fn first_word(s: &str) -> &str
// as it would allow for both String and string slices

// there also other types of slices
// we can use slices for all sorts of collections
fn other_slices() {
	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];
	assert_eq!(slice, &[2, 3]);
}
