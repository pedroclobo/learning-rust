#![allow(unused)]

// generics deals with the duplication of concepts problem
// generics allow us to specify a type that is unknown at compile and run time
// Option<T>, Vec<T>, HashMap<K, V>, Result<T, E> use generics

// duplicating code is tedious and error prone
fn code_duplication() {
	// first list
	let number_list = vec![34, 50, 25, 100, 65];
	let mut largest = number_list[0];
	for number in number_list {
		if number > largest {
			largest = number;
		}
	}
	println!("The largest number is {}", largest);

	// second list
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let mut largest = number_list[0];
	for number in number_list {
		if number > largest {
			largest = number;
		}
	}
	println!("The largest number is {}", largest);
}

// functions eliminate duplication of code
// list is a slice of i32 values
fn largest(list: &[i32]) -> i32 {
	let mut largest = list[0];

	// pattern matching
	for &item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

fn without_code_duplication() {
	// first list
	let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("The largest number is {}", result);

	// second list
	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
	let result = largest(&number_list);
	println!("The largest number is {}", result);
}

// similar to what we did above, we can use generics to reduce code duplication
// the function could operate on an abstract list instead of specific values

// we need to name a parameter type: T, by convention (short for type)
// 'largest is generic over T
// it has a parameter list, which is a slice of values of type T
// the largest function returns a value of type T'
// it won't compile because rust doesn't know how to compare T's (>)
//fn largest<T>(list: &[T]) -> T {
//	let mut largest = list[0];
//
//	for &item in list {
//		if item > largest {
//			largest = item;
//		}
//	}
//
//	largest
//}

// we can also use generics in struct definitions
// because we only specified one type, T
// x and y must be the same type
struct PointSameType<T> {
	x: T,
	y: T,
}

// now we define two generic types
// x and y can have different types
struct PointDifferentType<T, U> {
	x: T,
	y: U,
}

// enums also benefit from generics
// as we have seen before, Option and Result take a generic type
enum Option<T> {
	Some(T),
	None,
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}

// we can also use generic on methods
impl<T> PointSameType<T> {
	// return a reference to the x field
	fn x(&self) -> &T {
		&self.x
	}
}

// we can also implement methods on concrete types
impl PointSameType<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

// generic type parameters in a struct definition are not always the same
// as those we use in that same struct’s method signatures
// the generic parameters X1 and Y1 are declared after impl because they go
// with the struct definition
// the generic parameters X2 and Y2 are declared after fn mixup(), because
// they’re only relevant to the method
struct Point<X1, Y1> {
	x: X1,
	y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
	fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}

fn points() {
	let p1 = Point { x: 5, y: 10.4 };
	let p2 = Point { x: "Hello", y: 'c' };

	let p3 = p1.mixup(p2);

	println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// there is not any slow down or runtime cost to using generics
// rust turns generic code into specific code by filling in the concrete types
// that are used when compiled
// this process is called monomorphization

// traits
// they define a functionality a type implements
// they define shared behavior in an abstract way
// trait bounds specify that a generic type can be any type that has certain behavior
// it is similar to the concept of interfaces, like in Java

// a type behavior consists of the methods we can call on that type
// trait definitions group method signatures together to define a set of behaviors
// Summary display summaries of data that might be stored in a NewsArticle or Tweet instance
// a trait can have multiple methods
pub trait Summary {
	// we don't provide an implementation of the method
	// each type implementing Summary will have to implement this method
	fn summarize(&self) -> String;
}

pub struct NewsArticle {
	pub headline: String,
	pub location: String,
	pub author: String,
	pub content: String,
}

// impl <trait> for <type>
impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({})", self.headline, self.author, self.location)
	}
}

pub struct Tweet {
	pub username: String,
	pub content: String,
	pub reply: bool,
	pub retweet: bool,
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

// we can only implement a trait on a type only if at least one of the trait or
// the type is local to our crate
// we can’t implement the Display trait on Vec<T> within our aggregator crate,
// because Display and Vec<T> are both defined in the standard library and
// are not local to our aggregator crate
// this restriction is part of a property called coherence, and more specifically the orphan rule
// named this way because the parent type is not present
// this rule ensures that other people’s code can’t break your code and vice versa
// without the rule, two crates could implement the same trait for the same type,
// and Rust wouldn’t know which implementation to use.

// we can have default behaviour in our trait methods
// when we implement the trait on a particular type, we can keep or override each
// method's default behavior
pub trait Summary2 {
	fn summarize(&self) -> String {
		String::from("(Read more...)")
	}
}

// to use a default implementation, we specify an empty impl block
impl Summary2 for NewsArticle {}

// traits can be passed as parameters
// calls the summarize method on its item parameter
// which is of some type that implements the Summary trait
pub fn notify(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}

// the 'impl trait' syntax is actually syntax sugar for a longer form known as a trait bound
pub fn notify2<T: Summary>(item: &T) {
	println!("Breaking news! {}", item.summarize());
}

// we could use this to allow for the two types to have different types
//pub fn notify(item1: &impl Summary, item2: &impl Summary)

// in order to force the two parameters to have the same type we must use a trait bound
//pub fn notify<T: Summary>(item1: &T, item2: &T)

// we can also specify multiple trait bounds
//pub fn notify(item: &(impl Summary + Display))
//pub fn notify<T: Summary + Display>(item: &T)

// where clauses can make trait bounds clearer
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
//fn some_function<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug

// we can also return types that implement traits
// the return type implements Summary
fn returns_summarizable() -> impl Summary {
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("of course, as you probably already know, people"),
		reply: false,
		retweet: false,
	}
}

// however, we can only return one type
// if we would to return either a NewsArticle or a Tweet, this wouldn't work

// fixing the largest function
// T must implement PartialOrd in order to work on slices of any type that we can compare
// T must implement Copy in order to work on the types we wanted it to work: i32 and char
// if we don’t want to restrict the largest function to the types that implement the Copy trait,
// we could specify that T has the trait bound Clone instead of Copy
// we could also return a &T instead of a T, not requiring nor Copy nor Clone
fn largest_fixed<T: PartialOrd + Copy>(list: &[T]) -> T {
	let mut largest = list[0];

	for &item in list {
		if item > largest {
			largest = item;
		}
	}

	largest
}

// we can use trait bounds to conditionally implement methods
use std::fmt::Display;

struct Pair<T> {
	x: T,
	y: T,
}

// Pair<T> always implements new
impl<T> Pair<T> {
	fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

// Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
			println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}

// we can also implement a trait for any type that implements another particular trait
// implementations of a trait on types that satisfy the trait bounds are called blanket implementations
// we can call the to_string method on any type that implements the Display trait
//impl<T: Display> ToString for T {
//    --snip--
//}

// lifetime are another kind of generic
// rather than ensuring that a type has the behavior we want,
// lifetimes ensure that references are valid as long as we need them to be
// every reference has a lifetime that is the scope in which they are valid
// most of the time lifetimes are implicit
// the main aim of lifetimes is to prevent dangling references
// r has a lifetime of 'a and x has a lifetime of 'b
// the program won't compile because 'b is shorter than 'a
fn dangling_reference() {
	{
		//let r;

		{
			let x = 5;
			//r = &x;
		}

		//println!("r: {}", r);
	}
}

// here x has a lifetime of 'a and r has a lifetime of 'b
// and 'b is longer than 'a
fn no_dangling_reference() {
	{
		let x = 5;

		let r = &x;

		println!("r: {}", r);
	}
}

// how to return the longer of two strings?
// we pass and return &str because we don't want the function to take ownership
// the return type needs a generic lifetime parameter on it because
// rust can't tell whether the reference being returned refers to x or y
// we also don't know the concrete lifetimes of the references that will be passed in,
// so the compiler can't look at the scopes and determine if the references are valid
// we need to add a lifetime annotation

// the lifetime annotation must specify the following constraint:
// the returned reference will be valid as long as both the parameters are valid
// the signature tells us that:
// for some lifetime 'a, the function takes two parameters, both string slices
// that live at least as long as lifetime 'a
// the returned string slice will live as least as long as lifetime 'a
// the longest function should reject values that don't adhere to these constraints

// when we pass concrete references to longest, the concrete lifetime that is
// substituted for 'a is the part of the scope of x that overlaps with the scope of y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

// lifetime annotations don't change how long any of the references live
// they describe the relationships of the lifetimes of multiple references
// names of lifetimes must start with '
// &i32          // a reference
// &'a i32       // a reference with an explicit lifetime
// &'a mut i32   // a mutable reference with an explicit lifetime
// if two references have the same lifetime 'a, they both live as long as the generic lifetime

// in this example, string1 is valid until the end of the outer scope
// string2 is valid inside the inner scope
// result references something that is valid until the end of the inner scope
fn example() {
	let string1 = String::from("long string is long");

	{
		let string2 = String::from("xyz");
		let result = longest(string1.as_str(), string2.as_str());
		println!("The longest string is {}", result);
	}
}

// in this example, string1 is valid until the end of the outer scope
// string2 is valid inside the inner scope
// result references something that is valid only inside the inner scope...
// in order for result to be valid, string2 must also be valid until the end of the outer scope
fn bad_example() {
	let string1 = String::from("long string is long");
	let result;
	{
		let string2 = String::from("xyz");
		result = longest(string1.as_str(), string2.as_str());
	}
	//println!("The longest string is {}", result);
}

// if we changed the implementation of longest to always return the first parameter,
// we wouldn't need to specify a lifetime on the y parameter
// the lifetime of the y parameter doesn't have any relationship to the other lifetimes
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
	x
}

// the lifetime of the return parameter needs to match the lifetime for one of the parameters
// won't work
//fn longest2<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}

// we can define structs that don't have owned types
// structs can instead hold references, but they need a lifetime annotation
// this means an instance of ImportantExcerpt can't outlive the reference
// it holds in its part field
struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn structs() {
	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.').next().expect("Could not find a '.'");
	let i = ImportantExcerpt {
		part: first_sentence,
	};
}

// lifetime elisions rules are patterns programmed into Rust's analysis of references
// they are a set of cases that the compiler will consider and if the code fits these
// cases, we don't need to write the lifetimes explicitly
// when the compiler can't infer the lifetimes of the references, it will ask us
// to annotate them
// input lifetimes refer to lifetimes on function or method parameters
// output lifetimes refer to lifetimes on return values

// there are 3 rules that the compiler checks
// if after aplying the rules, the compiler can't infer the lifetimes of the references,
// it will ask us to annotate them

// 1. the compiler assigns a lifetime parameter to each parameter that’s a reference
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned
//    to all output lifetime parameters
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self
//    because this is a method, the lifetime of self is assigned to all output lifetime parameter

// lifetime annotations in method definitions
// lifetime names for struct fields always need to be declared after impl
// references might be tied to the lifetime of references in the struct's fields
// or they might be independent
impl<'a> ImportantExcerpt<'a> {
	// we are not required to annotate the lifetime of the reference to self because
	// of the first elision rule
	fn level(&self) -> i32 {
		3
	}

	// first and third rules apply and all references have a lifetime
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		println!("Attention please: {}", announcement);
		self.part
	}
}

// the static lifetime
// it denotes that the affected reference can live for the entire duration of the program
fn static_lifetimes() {
	let s: &'static str = "I have a static lifetime.";
}

// generic type parameters, trait bounds, and lifetimes together
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
	T: Display,
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
