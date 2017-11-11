fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);

    println!("v: {:?}", v);


    let third = v[2];
    println!("third: {:?}", third);
    println!("v.get(2): {:?}", v.get(2));
    v.push(7);
    println!("v: {:?}", v);
}
