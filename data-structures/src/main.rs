fn main() {
    println!("Hello, data structures!");

    strings();
}

fn strings() {
    let mut s = String::from("hello");
    // s is not valid here; it's not yet declared
    // s is valid from this point forward

    // do staff with s
    s.push_str(", world!"); // push_str() appends a leteral to a String
    println!("{}", s);

    // this scope is over, and s is no longer valid
}