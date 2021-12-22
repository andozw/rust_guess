enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("The third element is: {}", third);

    match v2.get(2) {
        Some(third) => println!("The matched element is: {}", third),
        None => println!("No third element"),
    }

    let mut v3 = Vec::new();

    v3.push(9);
    v3.push(8);
    v3.push(7);
    v3.push(6);

    for i in &v3 {
        println!("[{}]", i);
    }
    
    for i in &mut v3 {
        *i *= 11;
        println!("[{}]", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // get returns None when element does not exist.
    let will_not_panic = v3.get(100);
    let will_panic = &v3[100];
}
