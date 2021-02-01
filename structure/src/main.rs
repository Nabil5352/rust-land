struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    println!("Hello, Struct!");

    let email = String::from("nabil.ibn.mahmud@pm.me");
    let username = String::from("nabilahmad");
    let user = build_user(email, username);
    println!("User email: {} \nUsername: {}\nActive user? {}\nSigned in: {} times",
    user.email, user.username, user.active, user.sign_in_count);

    rectangle();
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}

fn rectangle() {
    // way 1 : traditional parameter
    let width = 30;
    let height = 50;

    println!("Way 1: The area of the rectangle is {} square pixels", area1(width, height));

    // way 2 : tuples
    let rect = (30, 50);

    println!("Way 2: The area of the rectangle is {} square pixels", area2(rect));

    // way 3: struct
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Way 3: The area of the rectangle is {} square pixels", area3(&rect1));

    // debug print
    println!("rect1 is {:#?}", rect1);

    // methon on Rectangle struct
    println!("Area method on Rectangle struct. Area: {}", rect1.area());

    let rect2 = Rectangle { width: 30, height: 50 };
    let rect3 = Rectangle { width: 10, height: 40 };
    let rect4 = Rectangle { width: 60, height: 45 };

    println!("Can hold: {}", rect2.can_hold(&rect3));
    println!("Can hold: {}", rect3.can_hold(&rect4));

    println!("Associated function. Square: {:#?}", Rectangle::square(3));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

