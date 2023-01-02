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
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
