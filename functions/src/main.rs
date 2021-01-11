fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    return_value();

    let f = five();
    println!("The value of f is: {}", f);

    let p = plus_one(9);
    println!("The value of p is: {}", p);

    expressions(4);

    looping();
}

fn return_value() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn expressions(x: i32) {
    if x < 5 {
        println!("conditions was true");
    } else if x == 5 {
        println!("Equals to 5");
    } else {
        println!("conditions was false");
    }

    let condition = true;
    let y = if condition {
        5
    } else {
        6
    };

    println!("Value of condition x is: {}", y);
}

fn looping() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    while index < 5 {
        println!("index is: {}", a[index]);
        index = index + 1;
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}