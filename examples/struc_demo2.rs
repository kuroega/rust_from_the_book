#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

//struct Nil;

//struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}
/*
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}*/

fn main() {
    let name = "Peter";
    let age = 18;
    let peter = Person { name, age };
    let george = Person { name: "George", age: 33 };
    println!("{:?}", peter);
    println!("{:?}", george);

    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("coordinate is defined by {} and {}", point.x, point.y);
}

