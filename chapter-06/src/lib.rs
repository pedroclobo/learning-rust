#![allow(unused)]

// enums allow to define a type by enumerating its possible variants
// enums define data types in a different way than structs

enum IpAddrKind {
	V4,
	V6,
}

fn route(ip_kind: IpAddrKind) {}

fn ip_addr_kind() {
	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	route(IpAddrKind::V4);
	route(IpAddrKind::V6);
}

// we could represent an IP address with a structure like so
// however representing it with a enum is more concise
struct IpAddrStruct {
	kind: IpAddrKind,
	address: String,
}

// we can put data directly into the enum
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

// the name of each enum variant that we define
// also becomes a function that constructs an instance of the enum
fn ip_addr() {
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));
}

// variants can have different types
// similar to defining various structs
// however they are all grouped together under the same type
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

// we can also define methods on enums
impl Message {
	fn call(&self) {}
}

fn methods() {
	let m = Message::Write(String::from("hello"));
	m.call();
}

// the Option enum
// it is defined in the standard library
// it encodes the concept of a value being present or absent
// it replaces the null from other programming languages
// if we have a value that can be null, we must opt in and make the type Option<T>
// everything that isn't an Option<T> is garanteed to be present
fn option() {
	// Option<i32>
	let some_number = Some(5);

	// Option<&str>
	let some_string = Some("a string");

	// for None we need to annotate the type
	let absent_number: Option<i32> = None;
}

// match is a control flow construct that allows pattern matching
// unlike in if statements, in match the expression can return any type, not just booleans
// each arm of the match follows the scheme: pattern => some code
// each arm is an expression and the result of the expression is the value of the match
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		// for longer arms we can use {}
		Coin::Penny => {
			println!("Lucky penny!");
			1
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}

// we can also have patterns that bind to values
// matches in rust are exhaustive, meaning that we have to match on all cases
fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1), // note that i binds to the value in Some
	}
}

// we can take special actions for certain values and take a default action for all others
// other catches all the other values
// if we are not going to use the value we can specify _ so the compiler doesn't
// warn us about an unused variable
fn placeholders() {
	let dice_roll = 9;
	match dice_roll {
		3 => add_fancy_hat(),
		7 => remove_fancy_hat(),
		other => move_player(other),
	}

	fn add_fancy_hat() {}
	fn remove_fancy_hat() {}
	fn move_player(num_spaces: u8) {}
}

// if let
// it allows to write a more concise match to handle values that match a
// pattern and ignore the rest
// it also doesn't require us to use a match arm for every possible value
// it takes a pattern and an expressions separated by an equal sign
// we lose the enforced exhaustive nature of match, which can be bad sometimes
fn if_let() {
	let config_max = Some(3u8);
	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max),
		_ => (),
	}

	// instead of specifying _ and doing nothing, we can use if let
	let config_max = Some(3u8);
	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
}
