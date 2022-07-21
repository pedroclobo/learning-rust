#![allow(unused)]

fn variables_and_mutability() {
	// x has to be a mutable variable,
	// otherwise we can't change its value later
	let mut x = 5;
	println!("The value of x is: {x}");
	x = 6;
	println!("The value of x is: {x}");
}

fn constants() {
	// constants are always immutable and can't be mutable
	// and their type has to be annotated
	// the naming convention is all uppercase with underscores
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

	println!(
		"The value of THREE_HOURS_IN_SECONDS is: {}",
		THREE_HOURS_IN_SECONDS
	);
}
// there can also be global constants
const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;

// the second variable is what the compiler will see when we use the name of the variable
fn shadowing() {
	let x = 5;

	// we can shadow the value of x with a new value
	let x = x + 1;

	// the new x in only valid inside this scope (inner shadowing)
	{
		let x = x * 2;
		println!("The value of x in the inner scope is: {x}");
	}

	// prints old value of x (6)
	println!("The value of x is: {x}");

	// when shadowing we can reuse the same variable name with a different type
	let spaces = "   ";
	let spaces = spaces.len();

	// that can't happen with mutable variables
	// they must be the same type
	//let mut spaces = "   ";
	//spaces = spaces.len();
}

// rust is statically typed, the compiler must know each variable type at compile time
// scalar types: (integers, floating-point numbers, booleans, characters)
// compound types: (tuples, arrays)
fn data_types() {
	// integer types
	// is a number without a fractional component
	// i means signed integer, u means unsigned integer
	// default type is i32
	// use isize or usize to index collections

	// integer  types
	// 8-bit    i8    u8
	// 16-bit   i16   u16
	// 32-bit   i32   u32
	// 64-bit   i64   u64
	// 128-bit  i128  u128
	// arch     isize usize     (architecture-size)

	// integer         literals
	// Decimal         98_222    (_ is a visual separator)
	// Hex             0xff
	// Octal           0o77
	// Binary          0b1111_0000
	// Byte (u8 only)  b'A'

	// floating-point types
	let x = 2.0; // f64
	let y: f32 = 3.0; // f32

	// numeric operations
	// addition
	let sum = 5 + 10;

	// subtraction
	let difference = 95.5 - 4.3;

	// multiplication
	let product = 4 * 30;

	// division
	let quotient = 56.7 / 32.2;
	let floored = 2 / 3; // Results in 0

	// remainder
	let remainder = 43 % 5;

	// boolean type
	let t = true;
	let f: bool = false; // with explicit type annotation

	// character type
	// 4 bytes in size
	// unicode scalar value
	let c = 'z';
	let z: char = 'ℤ'; // with explicit type annotation
	let heart_eyed_cat = '😻';

	// tuples
	// fixed size
	// can hold multiple data types
	// stored on the stack
	let tup: (i32, f64, u8) = (500, 6.4, 1);

	// pattern matching to destruct the tuple
	let (x, y, z) = tup;

	// indexing the tuple
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;

	// unit tuple
	// represent empty value and empty return value
	let unit_tuple = ();

	// arrays
	// fixed size
	// single data type
	// stored on the stack
	let a = [1, 2, 3, 4, 5];
	let a: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation

	// [3, 3, 3, 3, 3]
	let a = [3; 5];

	// indexing the array
	// out of bounds result in a runtime error
	let first = a[0];
	let second = a[1];
}

// are defined with the fn keyword
// the function definition order doesn't matter
// parameters are part of the function signature
// concrete values are arguments
// we must declare the type of the parameters
fn functions(value: i32, unit_label: char) {
	println!("The measurement is: {value}{unit_label}");
}

// rust is an expression-based language
// function bodies are made up of a series of statements,
// optionally ending in an expression
// statements are instructions that perform some action and do not return a value
// expressions evaluate to a resulting value.

// we must annotate the return type of the function
// function definitions are also statements
fn statements() -> i32 {
	// statement
	let y = 6;

	// expression inside the {}
	// y maps to the expression
	// expressions don't have ; at the end, or they would be statements
	let y = {
		let x = 3;
		x + 1
	};

	// not valid
	// won't assign a to 7 because "b = 7" is a statement
	//let mut a = 1;
	//let mut b = 2;
	//a = b = 7;

	// expression
	// we can also return earlier with the return keyword
	y
}

// the most common constructs in rust are if expressions and loops
fn control_flow() {
	// if expressions
	// blocks associated with an if statement are arms
	// use a match if there are many else if statements
	let number = 6;
	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 4, 3, or 2");
	}

	// conditions must be a boolean
	// not valid
	// if number {}

	// if in a let statement
	let condition = true;
	let number = if condition { 5 } else { 6 };

	// not valid
	// rust must know the variable type at compile time
	//let number = if condition { 5 } else { "six" };

	// loops
	// there are 3 types of loops: loop, while and for
	// loop loops forever or until we tell it to stop
	// continue can be used to skip the current iteration of the loop
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if counter == 10 {
			// return value from loop
			break counter * 2;
		}
	};

	// we can label loops
	let mut count = 0;
	'counting_up: loop {
		let mut remaining = 10;

		loop {
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}

	// conditional loops
	let mut number = 3;
	while number != 0 {
		number -= 1;
	}

	// looping through a collection
	let a = [10, 20, 30, 40, 50];
	for element in a {
		println!("the value is: {element}");
	}

	// rev reverses the order
	for number in (1..4).rev() {
		println!("{number}!");
	}
}
