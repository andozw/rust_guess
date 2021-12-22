fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // Does not works since add signature is: add(self, s: &str) -> String
    // let s3 = s1 + s2;
    let s3 = s1 + &s2;

    println!("s3: {}", s3);

    // Can no longer use s1 since it was moved by call to `+`
    // println!("s1 cannot be accessed: {}", s1);
    
    
    // Use format for easier concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // Rust strings don't support indexing
    // Wont work:
    // let first = s[0];
    
    // Why? String is a wrapper over a Vec<u8>
    //
    // Indexing into a string is often a bad idea since it is not clear what the type of 
    // indexing operation should be: byte, character, grapheme cluster, string slice? 
    // So you must be explicit. 
    // To ask for string slice:
    let first = &s[0..1];
    println!("first: {}", first);

    // Better way to iterate over Strings
   for c in "नमस्ते".chars() {
       println!("{}", c);
   }

   for b in "नमस्ते".bytes() {
       println!("{}", b);
   }
}
