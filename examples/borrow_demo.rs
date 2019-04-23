fn main() {
    let s1 = String::from("hello");
//    let (s2, len) = calculate_length(s1);
//    println!("the length of '{}' is {}", s2, len);
    let len = calculate_length(&s1); // it's called borrowing
    println!("the length of '{}' is {}", s1, len);
}

// takes ownership
//fn calculate_length(s: String) -> (String, usize) {
//    let length = s.len();
//    (s, length)
//}

// takes no ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}
