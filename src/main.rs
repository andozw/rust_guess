fn largest_i32(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &item in list {
        if item > max {
            max = item
        }
    }

    max
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &item in list {
        if item > max {
            max = item
        }
    }

    max
}

fn main() {
    let numbers = vec![7, 21, 9];
    println!("Largest number: {}", largest_i32(&numbers));
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['a', 'd', 'Z', 'q'];
    println!("Largest char: {}", largest(&chars));
}
