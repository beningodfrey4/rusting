// hello function programming
#[derive(Debug)]

enum IpAddrKind { // enums are not just fancy identifiers, but are variant types, just like Haskell :)
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type)
}

fn main() {
    let _four = IpAddrKind::V4(127, 0, 0, 1); // Variants have to always be resolved to only one "kind"
    let _six = IpAddrKind::V6(String::from("::1"));
    route(_four);
    let money = Coin::Dime;
    let five = Some(5);
    let six = plus_one(five); // Option type, great for exception handling, similar to expect() in std.
    let none = plus_one(None);
    print_pennies(&money);
    print_pennies(&Coin::Penny);

    if let Coin::Penny = money {
        println!("Penny");
    } else {
        println!("Not a penny")
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25, // matches must be exhaustive by design and must cover all variants with a valid statement (in this case, one that returns u32)
    }
}

fn print_pennies(coin: &Coin) {
    match coin {
        Coin::Penny => println!("Penny"),
        _ => () // '_' is a placeholder for all unmentioned variants and '()' is the unit type from functional programming
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}