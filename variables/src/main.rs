fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x+2; //shadowing
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_00;
    println!("Max point is: {}", MAX_POINTS);
}
