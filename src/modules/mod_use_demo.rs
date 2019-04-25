mod sound {
	pub mod instrument {
		pub fn clarinet() {
			// func body
		}
	}
}

// absolute path 
use crate::sound::instrument;
// private path

fn main() {
	instrument::clarinet();
	instrument::clarinet();
	instrument::clarinet();
}