use std::fs;

#[derive(Debug, Clone)]
enum Direction {
    Forward(u64),
    Up(u64),
    Down(u64),
}

#[derive(Debug)]
struct Position {
    horizontal: u64,
    depth: u64,
    aim: u64,
}
impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Position {
    fn move_to_dir(&mut self, direction: Direction) {
        match direction {
            Direction::Forward(x) => {
                self.depth += self.aim * x;
                self.horizontal += x;
            }
            Direction::Up(x) => self.aim -= x,
            Direction::Down(x) => self.aim += x,
        }
    }

    fn dist_from_origin(&self) -> u64 {
        self.horizontal * self.depth
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
    let _file_name_test = "input.test";
    let content = fs::read_to_string(file_name).expect("something went wrong reading the file");
    let mut position: Position = Position::default();
    content
        .lines()
        .map(|line| direction_from_str(line).unwrap())
        .for_each(|direction| position.move_to_dir(direction));
    println!("{:?}", position.dist_from_origin());
}
