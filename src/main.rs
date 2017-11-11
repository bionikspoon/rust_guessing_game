#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {

    let rect = Rectangle {
        length: 50,
        width: 30,
    };

    println!("rect is {:#?}", rect);

    println!(
        "The area of the reactangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
