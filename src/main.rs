// When returning a reference from a function, the lifetime parameter for the 
// return type needs to match the lifetime parameter for one of the parameters.
// If the reference returned does not refer to one of the parameters, it must refer
// to a value created within the function, which would be a dangling reference.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let string1 = String::from("longest");
    let string2 = String::from("short");

    let result = longest(string1.as_str(), string2.as_str());

    println!("Longest string is {}", result);
}
