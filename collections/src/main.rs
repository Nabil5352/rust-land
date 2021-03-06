use std::collections::HashMap;

fn main() {
    println!("Hello, collections!");

    vectors();

    reading_vector();

    iterate_vector();

    enum_collection();

    strings();

    hash_map();
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

fn strings() {
    // Approach 1
    let s1 = String::from("Initial contents 1");

    // Approach 2
    let s2 = "Initial contents 2".to_string();

    // Approach 3
    let data = "Initial contents 3";
    let s3 = data.to_string();

    println!("{}\n{}\n{}", s1, s2, s3);

    // Append String : Approach 1
    let mut s4 = String::from("foo");
    let s5 = "bar";
    s4.push_str(s5);
    println!("s4 is {}, s5 is {}", s4, s5);

    // Append String : Approach 2
    let s6 = String::from("Hello, ");
    let s7 = String::from("world!");
    let s8 = s6 + &s7;
    println!("s8 is {}", s8);

    // format! macro
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");
    let macro_str = format!("{}-{}-{}", s10, s11, s12);
    println!("macro_str is {}", macro_str);

    // String index
    let s9 = String::from("hello");
    let s_index = &s9[0..1];
    println!("String index {}", s_index);


    // String iteration
    let s13 = "Deadlock";

    println!("Char output");
    for c in s13.chars() {
        println!("{}", c);
    }

    println!("Byte output");
    for b in s13.bytes() {
        println!("{}", b);
    }
}

fn hash_map() {
    // Approach 1
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // Approach 2
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let Initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(Initial_scores.iter()).collect();

    // access
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // access through loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(11);
    scores.entry(String::from("Red")).or_insert(100);
    println!("{:?}", scores);

    // iterating
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
