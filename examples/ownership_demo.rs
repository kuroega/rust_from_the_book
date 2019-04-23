fn main() {
    let x = 5;
//    let y = x;
//    println!("{} : {}", x, y);

    let s1 = String::from("Hello"); // first variable is disabled, a.k.a. s1 is moved to s2
//    let s2 = s1;
//    println!("{} : {}", s1, s2); // error, will only work if use "let s2 = s1.clone();"

    takes_ownership(s1);
//    println!("{}", s1); // error

    makes_copy(x);
    println!("{}", x); // works

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3  = takes_and_gives_back(s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}