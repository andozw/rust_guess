use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();

    // expect is like unwrap but you can set custom message.
    let f = File::open("hello.txt").expect("Custom panic message");
}
