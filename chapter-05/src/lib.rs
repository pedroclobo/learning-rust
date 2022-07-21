#![allow(unused)]

// a struct is a custom data type that packages together a number of related
// values that form a meaningful group
// each piece of data (field) has a name
// the declaration order doesn't matter
// note the String instead of &str: we want each User to own its data
// we could use &str but we need to use lifetimes
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn structs() {
	// the entire instance must be mutable
	// we can't mark certain fields as mutable
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	// dot notation to get a specific field from the struct
	user1.email = String::from("anotheremail@example.com");

	// struct update syntax
	// its lets us include the non-specified fields from other instances of the struct
	// assignment moves the data
	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};
}

fn build_user(email: String, username: String) -> User {
	// field init shorthand
	// it lets us assign parameters to the fields with the same name
	User {
		email,
		username,
		active: true,
		sign_in_count: 1,
	}
}

// tuple structs
// they don't associate names with their fields
// useful for creating named tuples with a type
// we can destructure them and use . to index them
// Color and Point are different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
}

// unit-like structs
// they don't have any fields
// they are called this way because they behave like ()
// useful for implement a trait on some type that doesn't have any data
struct AlwaysEqual;

fn unit_like() {
	let subject = AlwaysEqual;
}

