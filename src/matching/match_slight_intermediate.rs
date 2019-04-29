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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Dime;
    let c3 = Coin::Dime;
    let c4 = Coin::Quarter(UsState::Alabama);
    let c5 = Coin::Nickel;
    let c6 = Coin::Quarter(UsState::Alaska);

    value_in_cents(c1);
    value_in_cents(c2);
    value_in_cents(c3);
    value_in_cents(c4);
    value_in_cents(c5);
    value_in_cents(c6);
}