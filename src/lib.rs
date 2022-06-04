// shut the fuck up
#![allow(dead_code)]

// shut the fuck up 2: electric boogaloo
#![allow(unused_imports)]

pub mod graph;
pub mod operators;
pub mod synthesis;
pub mod maths;

#[cfg(test)]
fn it_doesnt_work() {
	// im not even going to bother with an actual unit test i just want to see if the garbage my code generates even makes any sense

	// create sine bank to work with
	let sine_bank = synthesis::SineBank::new(2048, 80.0, 44100.0);
}