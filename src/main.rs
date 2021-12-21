mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// re-export with `pub use` so callers also have access.
use crate::front_of_house::hosting;

// can also use relative paths with `self`
// and bring in conflicting names by aliasing with `as`
use self::front_of_house::hosting as ghosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    ghosting::add_to_waitlist();
}
