struct Point<T, U> {
    x: T, 
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }

    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Zane", y: 'x' };
    let p3 = p1.mixup(p2);
    println!("p3: [{}, {}]", p3.x(), p3.y());

    // can just access directly
    println!("p3: [{}, {}]", p3.x, p3.y);
}
