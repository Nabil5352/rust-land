enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    println!("Hello, Enum!");

    enums();

    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter1 = value_in_cents(Coin::Quarter(UsState::Alabama));
    let quarter2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!(
        "Penny: {}\nNickel: {}\nDime: {},\nAlabama Quarter: {}\nAlaska Quarter: {}", 
        penny, nickel, dime, quarter1, quarter2);
}

fn enums() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddrKind::V4(127, 0, 0, 1);

    let _loopback = IpAddrKind::V6(String::from("::1"));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
