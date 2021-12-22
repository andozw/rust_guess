fn main() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    s1.push('X');
    println!("s1: {}, s2:{}", s1, s2);
}
