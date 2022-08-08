#![allow(unused)]

// smart pointers are data structures that act like a pointer
// and hold additional metadata and capabilities
// references are pointers that only borrow data
// smart pointers own the data they point too
// String and Vec<T> are examples of smart pointers

// smart pointers are usually implemented with structs
// however, smart pointers implement the Deref and Drop traits
// - Deref allows a smart pointer to behave like a reference
// - Drop allows us to customization the code to run when a smart pointer goes out of scope

// the most straightforward smart pointer is Box<T>
// is allows us to store data in the heap rather than the stack
// useful when:
// - we don't know the size of a type at compile time
// - we have a large amount of data and we want to transfer ownership
// - we want to own a generic value that implements some trait

// the deallocation happens for the i32 on the heap
// and for the box stored on the stack
fn store_i32_on_heap() {
	let b = Box::new(5);
	println!("b = {}", b);
} // b is deallocated here

// rust must know how much space a type takes at compile time
// recursive types can be part of themselves
// because of the infinite nesting, rust can't know the size of recursive types

// a cons list is made up of pairs: a single value and another pair
/*
enum List {
	Cons(i32, List),
	Nil,
}
*/

// to deal with the infinite recursion, we store the List indirectly
// cons will have the size of an i32 plus the Box's pointer data
enum List {
	Cons(i32, Box<List>),
	Nil,
}

fn cons() {
	use crate::List::{Cons, Nil};

	// store 1, 2 and 3 on the list
	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// implementing the Defer trait allows us to customize the behaviour of the * operator
fn dereferences() {
	let x = 5;

	// the only difference between the following is that, in the second,
	// y is a box pointing to the value of x instead of a reference to x
	// if we change x to be 10, y will still be 5 in the second version
	let y = &x; // y is a reference to x
	let y = Box::new(x); // will work as well

	assert_eq!(x, 5);
	assert_eq!(*y, 5); // we follow the reference (dereference)
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
	type Target = T; // associated type

	// without Deref, the compiler could only deference references
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn my_box_dereferences() {
	let x = 5;
	let y = MyBox::new(x);

	assert_eq!(x, 5);
	assert_eq!(*y, 5); // this won't work unless MyBox implements Deref

	// *y turns into *(y.deref())
	// as deref returns a reference
	// if it didn't, the value would be moved out of self because of the ownership system
}

// deref coercion happens in function and method parameters
// it does a sequence of calls to deref until the type matches the parameter's type
fn hello(name: &str) {
	println!("Hello, {}!", name);
}

// &MyBox<String> -> &String
// &String -> &str
// if there was no coercion we would have to write hello(&(*m)[..]);
fn coercion() {
	let m = MyBox::new(String::from("Rust"));
	hello(&m); // coercion makes it possible to pass a &MyBox<String>
}

// similar to how we use Deref to override * on immutable references,
// we can use DerefMut to override * on mutable references

// &T -> &U when T: Deref<Target=U>
// &mut T -> &mut U when T: DerefMut<Target=U>
// &mut T -> &U when T: Deref<Target=U>

// the Drop trait allows us to specify code to run on clean-up
// it is called automatically when a value goes out of scope

// sometimes we want to drop a value earlier
// we can do it with drop(<variable>)

// in rust, it is possible to enforce multiple ownership
// in graphs, multiple edges may point to the same node
// and that node is owned by all those edges
// Rc<T> enables us to do this
// it keeps of the number of references to a value
// if the number of references is 0, the value is dropped
// Rc<T> is only for single threaded scenarios
use std::rc::{Rc, Weak};

enum SharedList {
	Cons(i32, Rc<SharedList>),
	Nil,
}

// Rc only allows us to share immutable references
// a.clone() creates a deep copy
// Rc::clone(&a) only increments the reference count, not making a deep copy
fn shared_list() {
	use crate::SharedList::{Cons, Nil};

	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	assert_eq!(Rc::strong_count(&a), 1);

	// b and c have different first elements
	// but share a as its tail
	let b = Cons(3, Rc::clone(&a)); // we could have used a.clone()
	assert_eq!(Rc::strong_count(&a), 2);

	{
		let c = Cons(4, Rc::clone(&a));
		assert_eq!(Rc::strong_count(&a), 3);
	}

	assert_eq!(Rc::strong_count(&a), 2);
}

// interior mutability allows us to mutate data even if we only have a immutable reference
// it uses unsafe code to bend rust's rules about mutation and borrowing
// it is used when we can ensure that the borrowing rules will be followed at runtime,
// even though the compiler can't ensure it
// with RefCell<T> the borrowing rules are enforced at runtime
// if something bad happens, the program will panic
use std::cell::RefCell;

struct Person {
	name: RefCell<String>,
	age: u32,
}

impl Person {
	// even though push_str requires mutable access to self.name
	// using a RefCell we can mutate it with only a &self
	fn append_to_name(&self, name: String) {
		self.name.borrow_mut().push_str(&name[..]);
	}
}

fn will_panic() {
	let p = Person {
		name: RefCell::new(String::from("Pedro")),
		age: 20,
	};

	// we have two mutable references to the same data
	// the program will panic at runtime
	let mut one_borrow = p.name.borrow_mut();
	let mut two_borrow = p.name.borrow_mut();

	one_borrow.push('A');
	two_borrow.push('A');
}

// we combine RefCell<T> with Rc<T> for a value to have multiple owners
// and that we can mutate
enum SharedMutableList {
	Cons(Rc<RefCell<i32>>, Rc<SharedMutableList>),
	Nil,
}

fn multiple_mutable_references() {
	use crate::SharedMutableList::{Cons, Nil};

	let value = Rc::new(RefCell::new(5));

	let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
	let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
	let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

	// all lists will have 15
	*value.borrow_mut() += 10;
}

// rust makes it very difficult, but it is possible to have memory leaks
// it is possible to create reference cycles with Rc<T> and RefCell<T>
enum CycleList {
	Cons(i32, RefCell<Rc<CycleList>>),
	Nil,
}

impl CycleList {
	fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
		use crate::CycleList::{Cons, Nil};
		match self {
			Cons(_, item) => Some(item),
			Nil => None,
		}
	}
}

// the Rc<T> reference count will never be 0
// the values will never be dropped
fn reference_cycle() {
	use crate::CycleList::{Cons, Nil};

	let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
	let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

	// a points to b
	// b points to a
	if let Some(link) = a.tail() {
		*link.borrow_mut() = Rc::clone(&b);
	}
}

// calling Rc::clone will increase the strong_count of an Rc<T>
// calling Rc::downgrade will increase the weak reference count
// and return a Weak<T>
// the weak_count doesn't need to be 0 for the value to get dropped
// strong references express ownership, weak ones don't
// this will avoid reference cycles!
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>, // we use this so Nodes can refer to its parent
	children: RefCell<Vec<Rc<Node>>>,
}

fn tree() {
	let leaf = Rc::new(Node {
		value: 3,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![]),
	});

	let branch = Rc::new(Node {
		value: 5,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![Rc::clone(&leaf)]),
	});

	// add branch as a parent of leaf
	*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	// upgrade return an Option<Rc<T>> because Rc<T> might no longer exist
	leaf.parent.borrow().upgrade();
}
