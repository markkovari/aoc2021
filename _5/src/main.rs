struct Point {
    horizontal: u8,
    vertical: u8,
}

struct Line {
    start: Point,
    end: Point,
}

struct Table {
    points: Vec<Vec<Point>>,
}

fn main() {
    let content = include_str!("../input.test").lines().collect::<Vec<&str>>();
    println!("Content: {:?}", content);
}
