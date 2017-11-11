fn main() {
    let some_five = Some(5);
    let some_six = plus_one(some_five);
    let none = plus_one(None);

    println!("{:?}", some_five);
    println!("{:?}", some_six);
    println!("{:?}", none);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
