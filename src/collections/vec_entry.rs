fn main() {
    let v: Vec<i32> = Vec::new();
    // or 
    {
        let v2 = vec![1, 2, 3];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    let mut v_mutable = Vec::new();
    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    v_mutable.push(8);

    let third: &i32 = &v_mutable[2];
    println!("The third element is {}", third);

    match v_mutable.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v2 = vec![100, 32, 57];
    for i in &v2 {
        println!("{}", i);
    }

    let mut v3 = vec![100, 32, 57];
    for i in &mut v3 {
        // to change the value that the reference refers to 
        // we have to use dereference operator (*)
        *i += 50;
        println!("{}", i);
    }
}
