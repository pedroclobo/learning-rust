// release profiles allows us to have more control over options for compiling code
// the two main profiles of cargo are:
// - dev: used when running cargo build
// - release: used when running cargo build --release

// cargo has default settings for each of the profiles
// if there aren't any [profile,*] sections in Cargo.toml

// the opt-level setting controls the level of optimizations (0 to 3)
/*
* [profile.dev]
* opt-level = 0
*
* [profile.release]
* opt-level = 3
*/

// we can override any of the defaults
/*
* [profile.dev]
* opt-level = 1
*/

// configuration options are available at
// https://doc.rust-lang.org/cargo/

// we can publish our crates to https://crates.io
// documenting our packages will help other understand our code
// the documentation comment will generate HTML documentation
// documentation comments start with /// and support markdown syntax

//! # Chapter 14
//!
//! `chapter_14` is a collection of utilities to make performing certain
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter_14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}

// to generate the HTML we can run cargo doc
// cargo doc --open will build the HTML for our crate and open it in a browser

// typical used sections are:
// - examples
// - panics: where the function could panic
// - errors: describes the kinds of errors that might occur
// - safety: describes why the function is unsafe (if it is)

// example code blocks in the documentation will be tested
// when running cargo test

// we can add documentation to the item that contain the comments
// rather than adding documentation to the items following the comments
// with //!
// they are usually used to describe the purpose of the container

// we can organize our project with a certain structure and publish it to others
// with another structure
// instead of use my_crate::some_module::another_module::UsefulType;
// they could just use my_crate::UsefulType;.

// we can do this by using pub use
// we take a public item in one location and make it public in another location

// when using re-exports, cargo will list them in the HTML

// we need an account on crates.io so we can publish crates
// before publishing a create we need to add metadata to the crate by editing Cargo.toml
// name, description and license are required
// - we can use a dual license or specify a custom license

// after all this is setup, we can publish our create with cargo publish
// note that publishing a create is permanent
// to publish a new version, we edit the version in Cargo.toml and run cargo publish

// although we can't remove published versions of a create,
// we can prevent future projects from using it as a new dependency
// yanking a version will allow project that already use it to continue using it
// and prevent new ones from adding it as a dependency
// cargo yank --vers 1.0.1
// cargo yank --vers 1.0.1 --undo

// cargo offers workspaces so we can have multiple libraries in one create
// a workspace is a set of packages that share the same Cargo.toml and output directory
// in the top level Cargo.toml we specify the following
/* [workspace]
   members = [
	   "adder",
	   "add-one",
   ]
*/
// next we create a new project inside that same folder with
// cargo new adder
// cargo new --lib add-one
/*
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
*/
// all creates share the target directory to avoid unnecessary rebuilding

// to make the binary create adder depend on the library create add-one,
// we add the following to adder/Cargo.toml:
/*
[dependencies]

add-one = { path = "../add-one" }
*/

// to run the binary create from the add directory, we do it with
// cargo run -p adder

// the workspace has only one Cargo.lock at the top level directory,
// so all creates share the same dependency versions

// we can add dependencies in each crate's Cargo.toml
// we can't use a dependency we add in some crate's Cargo.toml in another crate
// unless we also specify that dependency in the other crate's Cargo.toml

// running cargo test will run the tests of all the workspace crates
// we can also run tests for one particular crate with
// cargo test -p <crate-name>

// to publish the crates in a workspace we need to publish them one by one

// we can use cargo install to install crates locally
// we can only install packages that have binary targets
