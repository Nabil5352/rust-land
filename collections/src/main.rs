fn main() {
    println!("Hello, collections!");

    vectors();

    reading_vector();

    iterate_vector();

    enum_collection();
}

fn vectors() {
    // Create
    let _v: Vec<i32> = Vec::new();

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

    // let does_not_exists = &x[100];
    println!("Third: {}\nThird alt: {:?}", third, third_alt);

} // v, x, y goes out of scope and is freed here

fn reading_vector() {
    let mut v = vec![1, 2, 3, 4, 5];

    let _third: &i32 = &v[2];
    let _third: Option<&i32> = v.get(2);

    let _first = &v[0];
    v.push(6);

    for i in &mut v {
        *i += 50;
    }
    
    for i in &v {
        println!("{}", i);
    }
}

fn iterate_vector() {
    let v = vec![100, 32, 57];

    println!("Immutable for loop");
    for i in &v {
        println!("{}", i);
    }

    let mut x = vec![100, 32, 57];
    println!("Mutable for loop");
    for i in &mut x {
        println!("{}", *i + 50);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_collection() {
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}