use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // generate_panic();
    // recoverable_error();
    // unwrap_error();
    // expect_error();
    // propagating_errors();
    propagating_errors_refactored();
}

fn generate_panic() {
    let v = vec![1,2,3];
    v[99];
    panic!("Crash and Burn!!");
}

fn recoverable_error() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

fn unwrap_error() {
    let f = File::open("yellow.txt").unwrap();
}

fn expect_error() {
    let f = File::open("yellow.txt").expect("Failed to open file");
}

fn propagating_errors() -> Result<String, io::Error> {
    let f = File::open("yellow.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagating_errors_refactored() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("yellow.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}
