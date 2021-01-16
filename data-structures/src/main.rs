fn main() {
    println!("Hello, data structures!");

    strings();

    assignment();
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

fn assignment() {
    let x = 5;
    let y = x; // both x and y are valid
    println!("X and Y is: {} {}", x, y);

    let s1 = String::from("hello");
    //let s2 = s1; // s1 no longer valid
    let s2 = s1.clone();
    println!("S1 and S2 is: {} {}", s1, s2);
}
