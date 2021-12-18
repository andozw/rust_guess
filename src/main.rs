fn main() {
    println!("{}", first_word(&String::from("Hello sir")));
    println!("{}", first_word(&String::from("")));
    println!("{}", first_word(&String::from("mutable")));
    println!("{}", first_word(&String::from(" bread")));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
