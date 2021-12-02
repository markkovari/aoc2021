use std::error::Error;
use std::fs;
use std::str::FromStr;

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
    fn move_with(self, direction: Direction) -> Self {
        match direction {
            Direction::Forward(x) => Self {
                depth: self.depth + self.aim * x,
                horizontal: self.horizontal + x,
                ..self
            },
            Direction::Up(x) => Self {
                aim: self.aim - x,
                ..self
            },
            Direction::Down(x) => Self {
                aim: self.aim + x,
                ..self
            },
        }
    }

    fn dist_from_origin(&self) -> u64 {
        self.horizontal * self.depth
    }
}

impl FromStr for Direction {
    type Err = Box<dyn std::error::Error>;
    fn from_str(from: &str) -> Result<Direction, Box<dyn Error>> {
        let parts: Vec<&str> = from.split(" ").collect::<Vec<_>>();
        if parts.len() == 2 {
            let amount = parts[1].trim().parse::<u64>()?;
            return match parts[0] {
                "forward" => Ok(Direction::Forward(amount)),
                "up" => Ok(Direction::Up(amount)),
                "down" => Ok(Direction::Down(amount)),
                another => panic!("direction does not exists in line:{}", another),
            };
        }
        panic!("Cannot read line, I DO NOT CARE, YOUR RESULTS ARE IN DANGER")
    }
}

fn main() {
    let file_name = "input.txt";
    let _file_name_test = "input.test";
    let content = fs::read_to_string(file_name).expect("something went wrong reading the file");
    let final_position = content
        .lines()
        .filter_map(|line| Direction::from_str(line).ok())
        .fold(Position::default(), |current, next| current.move_with(next));
    println!("{:?}", final_position.dist_from_origin());
}
