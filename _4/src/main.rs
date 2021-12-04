use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct ChoosenNumbers(Vec<u8>);

impl FromStr for ChoosenNumbers {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ChoosenNumbers(
            s.split(",").filter_map(|e| e.parse::<u8>().ok()).collect(),
        ))
    }
}

#[derive(Debug)]
struct Elem(u8, bool);

impl FromStr for Elem {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Elem(s.parse::<u8>()?, false))
    }
}

impl Elem {
    fn found(&mut self) {
        self.1 = true
    }
}

#[derive(Debug)]
struct Row(Vec<Elem>, bool);

impl FromStr for Row {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Row(
            s.split(" ")
                .filter_map(|elem| Elem::from_str(elem).ok())
                .collect::<Vec<Elem>>(),
            false,
        ))
    }
}

impl Row {
    fn found_at(&mut self, at: u8) {
        for elem in self.0.iter_mut() {
            if elem.0 == at {
                elem.found()
            }
        }
    }

    fn full(&self) -> bool {
        self.0.iter().all(|elem| elem.1)
    }
}

#[derive(Debug)]
struct Table(Vec<Row>, bool);

impl FromStr for Table {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Table(
            s.split("\n")
                .filter_map(|row| Row::from_str(row).ok())
                .collect::<Vec<Row>>(),
            false,
        ))
    }
}

impl Table {
    fn any_row(&self) -> bool {
        self.0.iter().any(|e| e.1)
    }
    fn toggle(&mut self, at: u8) {
        for row in self.0.iter_mut() {
            row.found_at(at)
        }
    }
    fn any_colum(&self) -> bool {
        (0..self.0.len()).any(|i| self.every_of_column(i))
    }

    fn every_of_column(&self, column: usize) -> bool {
        self.0.iter().all(|row| (row.0.get(column)).unwrap().1)
    }
}

fn main() {
    let content = include_str!("../input.test")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = ChoosenNumbers::from_str(content[0]);

    let tables: Vec<Table> = (&content[1..])
        .into_iter()
        .filter_map(|&table| Table::from_str(table).ok())
        .collect::<Vec<Table>>();

    println!("{:?}", tables);
    println!("{:?}", choosen_numbers);
}
