use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    announcement: T
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", announcement);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("longest");
    let string2 = String::from("short");

    let result = longest_with_announcement(string1.as_str(), 
        string2.as_str(),
        "ANNOUNCEMENT");

    println!("Longest string is {}", result);
}
