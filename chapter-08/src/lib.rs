#![allow(unused)]

// the stardard library includes a number of useful data structures called collections
// collections can contain multiple values
// they are stored on the heap, which means the amount of data doesn't need to
// be known at compile time

// vectors
// they store more than one value in a single data structure
// all values must be the same type
fn vector() {
	// create a new vector
	// we need a type annotation if we don't insert any values
	let v: Vec<i32> = Vec::new();

	// the vec! macro will create a vector with the given values
	// because we create it with an initial set of values,
	// rust can now infer the type of the vector
	let mut v = vec![1, 2, 3, 4];

	// adding elements to a vector
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	// when a vector gets dropped, all its elements are dropped as well
	{
		let v = vec![1, 2, 3, 4];
	} // <- v goes out of scope and is freed here

	// reading elements from a vector
	// indexing
	// makes the program crash with out-of-bounds error
	let third: &i32 = &v[2];

	// get method
	// returns None with out-of-bounds error
	// gives an Option<&T>
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	// when we hold an immutable reference to the first element in a vector
	// and try to add an element to the end,
	// we break the borrow checker rule that says we can't have multiple mutable references to the same data
	// adding a new element might trigger a copy on the heap and the first element
	// reference will no longer be valid
	let mut v = vec![1, 2, 3, 4, 5];
	let first = &v[0];
	//v.push(6);
	//println!("The first element is: {}", first);

	// iterating over a vector
	let v = vec![100, 32, 57];
	for i in &v {
		println!("{}", i);
	}

	// we can also iterate over mutable references
	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

	// despite vector only being able to hold one data type,
	// we can use enums so vectors can hold different data types
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	// see https://doc.rust-lang.org/std/vec/struct.Vec.html
	// for more information about vectors
}

// &str is a core type which is a reference to some UTF-8 encoded string data
// Strings are a growable, mutable and owned collection of bytes UTF-8 encoded string type
fn string() {
	// creating a String
	let s = String::new();

	// create a String from a literal
	// to_string is available on any type that implements Display
	let s = "initial contents".to_string();
	let s = String::from("initial contents");

	// Strings are UTF-8 encoded
	let hello = String::from("السلام عليكم");
	let hello = String::from("Dobrý den");
	let hello = String::from("Hello");
	let hello = String::from("שָׁלוֹם");
	let hello = String::from("नमस्ते");
	let hello = String::from("こんにちは");
	let hello = String::from("안녕하세요");
	let hello = String::from("你好");
	let hello = String::from("Olá");
	let hello = String::from("Здравствуйте");
	let hello = String::from("Hola");

	// appending to a String
	let mut s = String::from("foo");
	s.push_str("bar"); // append a string slice,
	s.push('l'); // append a single char

	// concatenating Strings
	// + has the signature: fn add(self, s: &str) -> String
	// the compiler can coerce &String into &str
	// &s2 becomes &s2[..]
	// self isn't a reference, so ownership is taken
	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	// the format! macro returns a String
	// it doesn't take ownership
	let s = format!("{}-{}-{}", s1, s2, s3);

	// indexing
	// rust doesn't support string indexing
	let hello = String::from("Hola"); // 4 bytes
	let hello = String::from("Здравствуйте"); // 24 bytes, because each char is 2 bytes

	// we can index string slices
	// they can very easily crash the program
	let hello = "Здравствуйте";
	let s = &hello[0..4];

	// iterating over a string
	// we need to specify if we want to iterate over chars or bytes
	for c in "नमस्ते".chars() {
		println!("{}", c);
	}

	for b in "नमस्ते".bytes() {
		println!("{}", b);
	}
}

// HashMap<K, V> stores a pair <key, value>, using a hash function
// all keys must have the same type and all values must have the same type
fn hash_map() {
	use std::collections::HashMap;

	// creating a hash map
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	// another way of creating a hash map
	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];
	let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

	// values that implement the Copy trait are copied into the hash map
	// for owned values, they will be moved to the hash map
	// we can move references but must guarantee that those remain valid, with lifetimes
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");
	let mut map = HashMap::new();
	map.insert(field_name, field_value);

	// accessing a value in a hash map
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	let team_name = String::from("Blue");

	// returns an Option<&V>
	let score = scores.get(&team_name); // using the get method

	// iterate over a hash map
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	// overwriting a value in a hash map
	// Blue will be replaced
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);

	// only inserting a new value if the key has no value
	// entry takes the key we want to check for
	// and returns an enum that indicates that represents a value that may be present or not
	// or_insert returns a mutable reference to the value, if it exists
	// otherwise it inserts a new value
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	// updating a hash map based on old value
	// split_whitespace iterates over sub-slices separated by whitespace
	let text = "hello world wonderful world";
	let mut map = HashMap::new();
	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1; // dereference
	}
}
