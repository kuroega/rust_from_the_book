fn main() {
    let rect_tuple = (12, 4);
    println!(
        "Area: {}", area(rect_tuple)
    );

    let rect = Rectangle { width: 12, height: 4 };
    println!(
        "Area: {}", area_with_struct(&rect)
    );
    println!(
        "Area: {}", rect.area() 
    );

    println!("rect is {:#?}", rect);

    let rect2 = Rectangle { width: 10, height: 2 };
    let rect3 = Rectangle { width: 14, height: 3 };

    println!("rect can hold rect2? {}", rect.can_hold(&rect2));
    println!("rect can hold rect3? {}", rect.can_hold(&rect3));

    let sq = Rectangle::square(9);

    println!("area of sq: {}", sq.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
