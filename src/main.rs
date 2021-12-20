enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");

    let absent_number: Option<i32> = Option::None;
}
