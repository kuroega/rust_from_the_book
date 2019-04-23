struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // use initialization shorthand
        username, // use initialization shorthand
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut u1 = User {
        email: String::from("xxx@outlook.com"),
        username: String::from("xxx"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = User {
        email: String::from("a@yahoo.com"),
        username: String::from("a"),
        ..u1 // remaining fields have the same value as those in u1
    };

    u1.email = String::from("yy@zhihu.com");
    let addr = u1.email;
    println!("{}", addr);

    println!("{}", u2.sign_in_count);

    let u3 = build_user(String::from("s@zhihu.com"), String::from("s"));

    println!("{}", u3.email);

}

