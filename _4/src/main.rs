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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        if self.full() {
            self.1 = true
        }
    }

    fn full(&self) -> bool {
        self.0.iter().all(|elem| elem.1)
    }

    fn get_unused(self) -> Vec<u128> {
        self.0
            .into_iter()
            .filter(|elem| !elem.1)
            .map(|elem| elem.0 as u128)
            .collect()
    }
}

#[derive(Debug, Clone)]
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
    fn toggle(&mut self, at: u8) -> () {
        for row in self.0.iter_mut() {
            row.found_at(at)
        }
    }
    fn any_colum(&self) -> bool {
        self.0
            .iter()
            .enumerate()
            .any(|(i, _)| self.every_of_column(i))
    }
    fn every_of_column(&self, column: usize) -> bool {
        self.0.iter().all(|row| row.0.get(column).unwrap().1)
    }
    fn found_full(&self) -> bool {
        self.any_colum() || self.any_row()
    }
    fn get_unused_numbers(self) -> Vec<u128> {
        self.0
            .into_iter()
            .map(|row| row.get_unused())
            .flatten()
            .collect()
    }
}

fn main() {
    let content = include_str!("../input.data")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = ChoosenNumbers::from_str(content[0]).unwrap();

    let mut tables: Vec<Table> = (&content[1..])
        .into_iter()
        .filter_map(|&table| Table::from_str(table.trim()).ok())
        .collect::<Vec<Table>>();

    for number in choosen_numbers.0 {
        for table in tables.iter_mut() {
            table.toggle(number);
            if table.found_full() {
                let missings = table
                    .clone()
                    .get_unused_numbers()
                    .into_iter()
                    .reduce(|a, b| a + b)
                    .unwrap();
                println!(
                    "{} , {} -> {}",
                    number,
                    missings,
                    (number as u128) * missings
                );
                return;
            }
        }
    }
}
