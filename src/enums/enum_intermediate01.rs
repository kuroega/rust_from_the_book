enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),	
}

// defining the enum Message is similar to defining different kinds of struct 
// defintions like below

struct QuitMessage; // unit struct
struct MoveMessage {
	x: i32,
	y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// define methods on enum
impl Message {
	fn call(&self) {
		// some code
	}
}

fn fn main() {
	let m = Message::Write(String::from("hello"));
	m.call();
}