fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let none = plus_one(None);
    
    let six = plus_one(Some(5));
    println!("Six: {}", six.unwrap());
}
