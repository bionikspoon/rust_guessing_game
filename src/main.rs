#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            &Coin::Penny => 1,
            &Coin::Nickel => 5,
            &Coin::Dime => 10,
            &Coin::Quarter => 25,
        }
    }
}

fn main() {
    println!("Penny value: {:?}", Coin::Penny.value());
    println!("Nickel value: {:?}", Coin::Nickel.value());
    println!("Dime value: {:?}", Coin::Dime.value());
    println!("Quarter value: {:?}", Coin::Quarter.value());
}
