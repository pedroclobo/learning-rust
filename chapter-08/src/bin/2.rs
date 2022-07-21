// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
fn pig_latin(s: &mut String) -> &String {
	let vowels = ['a', 'e', 'i', 'o', 'u'];

	// get first char
	let c = s.chars().next().unwrap();

	if vowels.contains(&c) {
		s.push_str("-hay");
		return s;
	} else {
		s.remove(0);
		*s = format!("{}-{}ay", s, c);
		return s;
	}
}

fn main() {
	let mut s = String::from("first");
	println!("{}", pig_latin(&mut s));

	let mut s = String::from("apple");
	println!("{}", pig_latin(&mut s));
}
