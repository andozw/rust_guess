use rust_guess;

mod common;

#[test]
fn it_adds_twoooo() {
    common::setup();
    assert_eq!(4, rust_guess::adder::add_two(2));
}
