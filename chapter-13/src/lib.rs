#![allow(unused)]

// rust’s closures are anonymous functions we can save in a variable or pass as arguments to other functions
// we can create the closure in one place and then call the closure to evaluate it in a different context
// unlike functions, closures can capture values from the scope in which they’re defined

// this function takes a long time to execute
use std::thread;
use std::time::Duration;
fn simulated_expensive_calculation(intensity: u32) -> u32 {
	println!("calculating slowly...");
	thread::sleep(Duration::from_secs(2));
	intensity
}

// we are calling the costly function repeated times
fn generate_workout_1(intensity: u32, random_number: u32) {
	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			simulated_expensive_calculation(intensity)
		);
		println!(
			"Next, do {} situps!",
			simulated_expensive_calculation(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a brake today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				simulated_expensive_calculation(intensity)
			);
		}
	}
}

// we only call the costly function once
// however we call it even when it is not needed
fn generate_workout_2(intensity: u32, random_number: u32) {
	let expensive_result = simulated_expensive_calculation(intensity);

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result);
		println!("Next, do {} situps!", expensive_result);
	} else {
		if random_number == 3 {
			println!("Take a brake today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_result);
		}
	}
}

// if we want to define code in one place in our program
// but only execute that code where we need the result,
// we can use closures
// closures contain an anonymous function, not its result
fn generate_workout_3(intensity: u32, random_number: u32) {
	// define the closure and store it in a variable
	// |<param1>, <param2>, ...|
	let expensive_closure = |num| {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	};

	// we're still calling the closure twice inside this block
	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_closure(intensity));
		println!("Next, do {} situps!", expensive_closure(intensity));
	} else {
		if random_number == 3 {
			println!("Take a brake today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_closure(intensity));
		}
	}
}

// closures don't require type annotation because, unlike functions,
// they are not exposed to the users
// however, we can add type annotation if we want
fn add_one_v1(x: u32) -> u32 {
	x + 1
}

fn closures() {
	let add_one_v2 = |x: u32| -> u32 { x + 1 };
}

// we can't call the closure twice passing it a String first and then a u32
// the compiler inferred the type of x and the return type to be Strings
fn closures_types() {
	let example_closure = |x| x;
	let s = example_closure(String::from("hello"));
	//let n = example_closure(5);
}

// to solve the workout generation app problem of calling the closure twice in the if statement,
// we can create a struct that holds the closure and its result
// it will execute the closure only if the resulting value needs to be computed
// we need to specify the type of the closure
// all closures implement at least one of the traits: Fn, FnMut, FnOnce
struct Cacher<T>
where
	T: Fn(u32) -> u32,
{
	calculation: T,
	value: Option<u32>,
}

// before we execute the closure, the value will be None
// when we execute the closure, we store its return value in value
// only Cacher manages its value field, hence all the fields being private
impl<T> Cacher<T>
where
	T: Fn(u32) -> u32,
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: None,
		}
	}

	fn value(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
				v
			}
		}
	}
}

fn generate_workout_4(intensity: u32, random_number: u32) {
	let mut expensive_result = Cacher::new(|num| {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	});

	if intensity < 25 {
		println!("Today, do {} pushups!", expensive_result.value(intensity));
		println!("Next, do {} situps!", expensive_result.value(intensity));
	} else {
		if random_number == 3 {
			println!("Take a brake today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				expensive_result.value(intensity)
			);
		}
	}
}

// closures can access variables from their scope
// when a closure captures variables from its environment,
// it uses extra memory to store the values for use
// closures can capture values via
// - taking ownership (FnOnce)
// - borrowing mutably (FnMut)
// - borrowing immutably (Fn)
fn scope() {
	let x = 4;
	let equal_to_x = |z| z == x;
	let y = 4;
	assert!(equal_to_x(y));
}

// to force a closure to take ownership of the value we can use 'move'
fn moving() {
	let x = vec![1, 2, 3];

	// x has been moved
	// we can't use it after this line
	let equal_to_x = move |z: Vec<u32>| z == x;
}

// iterators allow us to perform some task on a sequence of items
// it iterates over the elements and determines when the sequence has finished
// iterators are lazy: they wait for us to call methods on them
fn iterators() {
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();
	for val in v1_iter {
		println!("Got: {}", val);
	}
}

// all iterators implement a trait named Iterator
// the associated type represents the type returned from the iterator
pub trait Iterator {
	type Item; // associated type
	fn next(&mut self) -> Option<Self::Item>;
}

// we call the iterator mutably because each time we call next,
// we change its internal state
// the values we get from the calls are immutable references to the values stored in the vector
// if we want to own the values we call 'iter_mut' instead of 'iter'
#[test]
fn iterator_demonstration() {
	let v1 = vec![1, 2, 3];
	let mut v1_iter = v1.iter();
	assert_eq!(v1_iter.next(), Some(&1));
	assert_eq!(v1_iter.next(), Some(&2));
	assert_eq!(v1_iter.next(), Some(&3));
	assert_eq!(v1_iter.next(), None);
}

// methods that call the 'next' method are called consuming adaptors
// sum iterates over all the values and computes their sum
#[test]
fn iterator_sum() {
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();

	// we aren't allowed to call v1 after this line
	let total: i32 = v1_iter.sum();
	assert_eq!(total, 6);
}

// methods that change iterators into different kinds are called iterator adaptors
// we can chain multiple calls to iterator adaptors but,
// because they are lazy, we need to can one of the consuming adpators
fn iterator_adaptors() {
	let v1: Vec<i32> = vec![1, 2, 3];

	// this line does nothing
	// the specified closure never gets called
	v1.iter().map(|x| x + 1);

	let v1: Vec<i32> = vec![1, 2, 3];
	// collect consumes the iterator
	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
	assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

// the filter iterator adaptor takes a closure that takes each item from the iterator
// and returns a boolean
// if the closure returns true, the value will be included in the iterator
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
	let shoes = vec![
		Shoe {
			size: 10,
			style: String::from("sneaker"),
		},
		Shoe {
			size: 13,
			style: String::from("sandal"),
		},
		Shoe {
			size: 10,
			style: String::from("boot"),
		},
	];
	let in_my_size = shoes_in_my_size(shoes, 10);
	assert_eq!(
		in_my_size,
		vec![
			Shoe {
				size: 10,
				style: String::from("sneaker")
			},
			Shoe {
				size: 10,
				style: String::from("boot")
			},
		]
	);
}

// we can create iterators for our own types with the iterator trait
struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl std::iter::Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}

#[test]
fn calling_next_directly() {
	let mut counter = Counter::new();
	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}

// we can also use other iterator trait methods
// - pair the values of Counter with another Counter skipping the first value
// - take the product of the pair
// - take those that are divisible by 3
// - add all the resulting values together
#[test]
fn using_other_iterator_trait_methods() {
	let sum: u32 = Counter::new()
		.zip(Counter::new().skip(1))
		.map(|(a, b)| a * b)
		.filter(|x| x % 3 == 0)
		.sum();

	assert_eq!(18, sum);
}
