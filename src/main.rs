enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// Borrowing rules: 
//      * you can have either (but not both of) one mutable reference or any number
//        of immutable references.
//      * references must always be valid.
//
// With Box<T> the borrowing rules' invariants are enforced at compile time
// With RefCell<T> the invariants are enforced at runtime.
// Both are only allowed in single thread programs.
// Because RefCell<T> allows mutable borrows checked at runtime, 
// you can mutate the value inside the RefCell<T> even when the
// RefCell<T> is immutable.
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));

    // Rc::clone for shallow copies and reference count updates.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }

    println!("count after c dies: {}", Rc::strong_count(&a));
}
