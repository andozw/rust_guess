use std::collections::HashMap;

fn print_map(map: &HashMap<String, i32>) {
    println!("----------");
    for (key, value) in map {
        println!("{}: {}", key, value);
    }
    println!("----------");
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 25);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![10, 25];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    scores.insert(String::from("Green"), 33);

    println!("Red Score: {}", scores.get(&String::from("Red")).unwrap());

    print_map(&scores);

    // Replace entry
    scores.insert(String::from("Red"), 62);
    
    // Only add entry if it doesn't already exist.
    println!("foo: {}", scores.entry(String::from("Green")).or_insert(77));
    println!("bar: {}", scores.entry(String::from("Purple")).or_insert(77));

    print_map(&scores);

    // increment value in map.
    let text = "hello world wonderful hello world yahtzee";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
