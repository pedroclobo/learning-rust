#![allow(unused)]

// as we write large programs, organizing the code becomes more important
// we can split it into multiple modules and then into multiple files
// each package can contain multiple binary crates and a single optional library crate
// the module system allows us to manage code
// it includes:
// - packages -> build, test and share creates
// - crates -> tree of modules that produces a library or executable
// - modules and use -> control the organization, scope and privacy of paths
// - paths -> a way of naming items

// creates can be binary crates or library crates
// the create root is the file that the compiler starts to read
// it makes up the root module of the crate
// for binaries it is src/main.rs
// for libraries it is src/lib.rs

// creating a package
// cargo new <package-name>

// modules
// they organize code within a create into groups for readability and easy reuse
// they are also used to control the privacy of code

// modules can also hold structs, enums, constants, functions, traits, etc.
// we use pub to make an item public
// by default items are private
// items in a parent directory can't access its children directories
// however a children directory can access its parent directory
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}
	}
}

// our current module tree
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// to find an item in the module tree, we use a path
// there are absolute paths and relative paths
pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
	fn fix_incorrect_order() {
		cook_order();
		super::deliver_order(); // we can access parent items with super
	}

	fn cook_order() {}

	// structs will have their fields private by default even if
	// the struct is public
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	// enums have their variants public if the enum is public
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

// use can bring paths into scope
// it is similar to creating a symbolic link in the file system
// note that use only works in the current scope
use crate::front_of_house::hosting; // idiomatic way for bringing function into scope

pub fn eat() {
	hosting::add_to_waitlist();
}

use std::collections::HashMap; // idiomatic way for bringing other items into scope

// bring items with the same name into scope
use std::fmt;
use std::io;

// we can also use the as keyword to rename types
use std::fmt::Result;
use std::io::Result as IoResult;

// we can re-export items from a module
// re-exporting is the process of bringing an item into scope
// and making it available to other
pub use crate::back_of_house::Breakfast;

// bring external traits into scope
// there are community creates available at crates.io
use rand::Rng;

// if were are importing multiple items from the same module or create,
// we can use nested paths
use std::{cmp::Ordering, io::Read};

// the glob operator brings all public items defined in a path into scope
use std::collections::*;

// we can also have modules separated into multiple files
// their file system layout must match the module tree
