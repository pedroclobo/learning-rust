#![allow(unused)]

// take the width and height of a rectangle specified in pixels
// and calculate the area of the rectangle

// the function is supposed to calculate the area on one rectangle
// but takes 2 parameters
fn naive(width: u32, height: u32) -> u32 {
	width * height
}

// tuples don't name their elements
// it is unclear what each element is supposed to be
fn tuples(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}

// opt in to print out debugging information
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// its much clearer than the alternatives before
fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

fn debug(rectangle: &Rectangle) {
	// we can't do this because rectangle doesn't implement std::fmt::Display
	//println!("{}", rectangle);

	// we want the Debug output format
	println!("{:?}", rectangle);

	// prettier format
	println!("{:#?}", rectangle);

	// another way: use the dbg! macro
	// takes ownership of the value
	// returns ownership of the value
	dbg!(rectangle);

	// we would rather use the following so dbg! doesn't take ownership
	dbg!(&rectangle);
}

// how to tie the area computation more closely to our Rectangle struct
// as it won't work with other types?
// we can use methods
// they are functions associated with a specific struct
// their first parameter is self

// everything within the impl block is associated with the Rectangle struct
// the functions inside are called associated functions
// there can be multiple impl blocks for a single struct
impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}

	// self in an alias for the type of the struct
	// methods can take ownership of self
	// and borrow it immutably or mutably
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// we can give a method the sames name as one of the struct's fields
	// this is typically used in defining getters and setters
	fn width(&self) -> bool {
		self.width > 0
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

// rust does automatic dereferencing on struct methods
// like -> and . in C

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

	// :: to access the associated function that doesn't take self as a parameter
	let sq = Rectangle::square(3);
}
