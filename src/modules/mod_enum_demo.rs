// make a public enum, all of its variants are public 
mod menu {
	pub enum Appetizer {
		Soup,
		Salad,
	}
}

fn main() {
	let order1 = menu::Appetizer::Soup;
	let order2 = menu::Appetizer::Salad;
}