fn main() {
    println!("Hello, data structures!");

    strings();

    assignment();

    let s = String::from("world"); // s comes into scope
    
    // s's value move into the function. and so is no longer valid here
    takes_ownership(s);

    let x = 5; // x comes into scope

    // but i32 is Copy, so it's okat to still use x afterward
    makes_copy(x);

    //println!("{}", s); // s no longer valid
    // println!("{}", x); // x is valid
} // here, x goes out of scope, then s.
// But because s's value was moved, nothing special happens

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

fn takes_ownership(param: String) {
    println!("{}", param);
} // Here, param goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(param: i32) {
    println!("{}", param);
} // Here, param goes out of scope. Nothing special happens.