fn main() {
    println!("Hello, collections!");

    vectors();
}

fn vectors() {
    // Create
    let v: Vec<i32> = Vec::new();

    let x = vec![1, 2, 3];

    // Update
    let mut y = Vec::new();
    y.push(3);
    y.push(5);
    y.push(7);
    y.push(9);

    // Reading
    let third: &i32 = &x[2];
    let third_alt: Option<&i32> = x.get(2);

    let does_not_exists = &x[100];

} // v, x, y goes out of scope and is freed here
