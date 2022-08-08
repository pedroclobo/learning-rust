#![allow(unused)]

// one of rust's main goals is to handle concurrence safely
// rust's 'fearless concurrency' refers to the fact that we can catch
// many concurrency bugs at compile time instead of runtime

// splitting the computation between multiple threads can lead to several problems:
// - race conditions
// - deadlocks
fn thread() {
	use std::thread;
	use std::time::Duration;

	// this thread will be stopped when the main thread ends
	let handle = thread::spawn(|| {
		for i in 1..10 {
			println!("hi number {} from the spawned thread!", i);
			thread::sleep(Duration::from_millis(1));
		}
	});

	// if we joined the threads here,
	// the main thread would wait for the handle thread to terminate
	//handle.join().unwrap();

	for i in 1..5 {
		println!("hi number {} from the main thread!", i);
		thread::sleep(Duration::from_millis(1));
	}

	// join waits for the thread to finish
	// it blocks the main thread until the thread terminates
	handle.join().unwrap();
}

// the move closure allows us to share data between threads
fn moving() {
	use std::thread;

	let v = vec![1, 2, 3];

	let handle = thread::spawn(move || {
		println!("Here's a vector: {:?}", v);
	});
}

// message passing is a popular approach to ensure safe concurrency
// "Do not communicate by sharing memory; instead share memory by communicating"
// rust has channels to accomplish message-sending concurrency
// the channel has two halves: a transmitter and a receiver
// a channel is said to be closed if either of the halves are dropped
fn channel() {
	use std::sync::mpsc;
	use std::thread;

	// mpsc stand for multiple producer, single consumer
	// tx is the sending end (transmitter)
	// rx is the receiving end (receiver)
	let (tx, rx) = mpsc::channel();

	// move sending end to thread and send a message
	thread::spawn(move || {
		let val = String::from("hi");
		tx.send(val).unwrap();
	});

	// recv will block the main thread and wait for a value to be sent
	// try_recv will not block the main thread
	let received = rx.recv().unwrap();
	println!("Got: {}", received);
}

fn multiple_messages() {
	use std::sync::mpsc;
	use std::thread;
	use std::time::Duration;

	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	// we can treat rx as an iterator
	// it will iterate over all received messages
	// it will stop when the channel is closed
	for received in rx {
		println!("Got: {}", received);
	}
}

// we can have multiple producers by cloning the transmitter half of the channel
fn multiple_producers() {
	use std::sync::mpsc;
	use std::thread;
	use std::time::Duration;

	let (tx, rx) = mpsc::channel();
	let tx1 = mpsc::Sender::clone(&tx); // clone the transmitter half

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for val in vals {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	thread::spawn(move || {
		let vals = vec![
			String::from("hi"),
			String::from("from"),
			String::from("the"),
			String::from("thread"),
		];

		for val in vals {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("Got: {}", received);
	}
}

// it is also possible to communicate by sharing memory
// mutex are a common concurrency primitive
// mutex is an abbreviation for mutual exclusion
// - we must acquire the lock before using the data
// - we must unlock the mutex when we're done with the data
// Mutex is a smart pointer, that offers interior mutability
// it implements Deref
// it also implements Drop
// - the lock is released as the MutexGuard return by lock goes out of scope
fn mutex() {
	use std::sync::Mutex;

	let m = Mutex::new(5);

	{
		// a call to lock fails if another thread holding the lock panics
		let mut num = m.lock().unwrap(); // acquire the lock to access the data
		*num = 6; // num is a mutable reference to the data inside
	}
}

// it is possible for multiple threads to own the same value
// our first approach would be to use a Rc<T>
// however, Rc<T> is not thread safe, as it does not implement Send
// Arc<T> is a type like Rc<T> that is thread safe (atomically reference counted type)
fn multiple_threads_share() {
	use std::sync::{Arc, Mutex};
	use std::thread;

	let counter = Arc::new(Mutex::new(0));
	let mut handles = vec![];

	for _ in 1..10 {
		let counter = Arc::clone(&counter);
		let handle = thread::spawn(move || {
			let mut num = counter.lock().unwrap();
			*num += 1;
		});
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}
}

// the Send trait indicates that ownership of the type can be transferred safely across threads
// the Sync trait indicates that it is safe for the type to be references by multiple threads
