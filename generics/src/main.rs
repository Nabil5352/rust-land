struct Point<T> {
    x: T,
    y: T
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn main() {
    // Functions
    let numbers = vec![34, 23, 101, 100];
    let result = findLargest(&numbers); 
    println!("The largest number is {}", result);

    let chars = vec!['y', 'c', 'p', 'a'];
    let result = findLargest(&chars); 
    println!("The largest character is {}", result);

    // Struct
    let integer = Point {x: 2, y: 10};
    let float = Point {x: 2.2, y: 10.1};
}

fn findLargest<T>(list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

   largest
}
