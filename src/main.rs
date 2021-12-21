mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn get_fruit(&self) -> &String {
            &self.seasonal_fruit
        }
    }
}


fn main() { 
    let breakfast = back_of_house::Breakfast::summer("rye");

    println!("Toast is: {}, fruit is: {}", breakfast.toast, breakfast.get_fruit());
}
