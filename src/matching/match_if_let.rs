#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// if let: The Concise Control Flow  
fn fn main() {
    let some_u8_value = Some(0u8);

    // In some case, we only want the program to match 
    // a certain value, in the case below we only want
    // to execute the code when the target value is 3.

    // Using match (exhaustive by nature) we have to check 
    // 'None' case which makes a simple operation wordy.
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // Let's rewrite the above code in 'if let' style.
    // 'if let' will only target to one pattern and ignore
    // the result, which reduces boilerplate
    if let Some(3) = some_u8_value {
        println!("three");
    }


    let mut count = 0;
    // Generally, 'if let' is a syntax sugar of 'match'
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // is equivalent to
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}