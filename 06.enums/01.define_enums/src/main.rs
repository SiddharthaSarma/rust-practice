enum IpAddr {
    v4,
    v6,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum IpAddrNew {
    v4(u32, u32, u32, u32),
    v6(String),
}
fn main() {
    let four = IpAddr::v4;
    let six = IpAddr::v6;

    let four_new = IpAddrNew::v4(127, 0, 0, 1);
    let six_new = IpAddrNew::v6(String::from("::1"));

    let val = Some("5");

    let some_max = Some(3u8);
    match some_max {
        Some(max) => println!("the max value is {}", max),
        _ => (),
    }
    if let Some(max) = some_max {
        println!("the max value using if let is {}", max);
    }

    let mut count = 1;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
