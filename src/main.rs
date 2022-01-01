struct MyPointer {
    data: String,
}

impl Drop for MyPointer {
    fn drop(&mut self) {
        println!("Droppping MyPoint: {}", self.data)
    }
}

fn main() {
    let a = MyPointer {
        data: String::from("first"),
    };

    let b = MyPointer {
        data: String::from("second"),
    };

    {
        let c = MyPointer {
            data: String::from("third"),
        };
    }

    println!("Finished!");
}
