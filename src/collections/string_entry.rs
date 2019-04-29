fn main() {
    let data = "initial contents";
    // to_string() method is available on any type 
    // that implements the Display trait
    let s = data.to_string();
    println!("{}", s);

    // the method also works on a literal directly
    let s = "initial contents".to_string();
    println!("{}", s);


    // is equivalent to the following code 
    let s = String::from("initial contents");
    println!("{}", s);

    // append operation
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str does not take ownership 
    println!("s2 is {}", s2); // otherwise, println!() won't work

    // or use push() to add a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);


    // string concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);
    // + operator uses the add method
    // fn add(self, s: &str) -> String {
    // }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    let s = format!("{}-{}-{}", s1, s2, s3);
}

