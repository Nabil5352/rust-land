fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = x+2; //shadowing
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_00;
    println!("Max point is: {}", MAX_POINTS);

    // Scaler type: floating-point
    let y = 2.0; // f64
    let z: f32 = 3.0; // f32
    println!("Float {} {}", y, z);

    // Boolean type
    let f: bool = false; // with explicit type annonation
    println!("Boolean {}", f);
    
    // Character type
    let c = 'z';
    println!("Character {}", c);

    // Compound type: tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is: {} {} {}", x, y, z);
    println!("The value of x,y,z is: {} {} {}", tup.0, tup.1, tup.2);

    // Compound type: array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first and second is: {} {}", first, second);
}
