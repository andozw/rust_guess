use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
    
fn main() {
    let username = read_username_from_file().unwrap();

    println!("Username: {}", username);
}
