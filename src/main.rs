#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

fn main() {
    let maybe_number = Maybe::Just(5);
    let maybe_string = Maybe::Just("string");

    let missing_number: Maybe<i32> = Maybe::Nothing;

    println!("{:?}", maybe_number);
    println!("{:?}", maybe_string);
    println!("{:?}", missing_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        None => unimplemented!(),
        Some(y) => x + y,
    };
}
