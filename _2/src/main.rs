use std::fs;

enum Direction {
    Forward(u64),
    Up(u64),
    Down(u64),
}
#[derive(Debug)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_to_dir(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(e) => self.x += e,
            Direction::Up(e) => self.y -= e,
            Direction::Down(e) => self.y += e,
        }
    }

    fn dist_from_origin(&self) -> u64 {
        self.x * self.y
    }
}

fn direction_from_str(from: &str) -> Option<Direction> {
    let parts: Vec<&str> = from.split(" ").collect::<Vec<_>>();
    if parts.len() != 2 {
        return None;
    }
    let amount = parts[1]
        .trim()
        .parse::<u64>()
        .expect("Cannot convert number");
    return match parts[0] {
        "forward" => Some(Direction::Forward(amount)),
        "up" => Some(Direction::Up(amount)),
        "down" => Some(Direction::Down(amount)),
        _ => None,
    };
}

fn main() {
    let file_name = "input.txt";
    let content = fs::read_to_string(file_name).expect("something went wrong reading the file");
    let directions: Vec<Direction> = content
        .lines()
        .map(|line| direction_from_str(line).unwrap())
        .collect();
    let mut position: Position = Position::new();
    for direction in directions {
        position.move_to_dir(direction);
    }
    println!("{:?}", position.dist_from_origin());
}
