#![feature(drain_filter)]
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
    let content = include_str!("../input.test")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = ChoosenNumbers::from_str(content[0]).unwrap();

    let mut tables: Vec<Table> = (&content[1..])
        .into_iter()
        .filter_map(|&table| Table::from_str(table.trim()).ok())
        .collect::<Vec<Table>>();
    let mut found_table_indicies: Vec<usize> = vec![];
    // let tables_length = tables.len();
    for number in choosen_numbers.0 {
        tables = tables
            .drain_filter(|table| {
                table.toggle(number);
                table.found_full()
            })
            .collect();
        // tables.retain(|table| (*table).found_full())
    }
    // tables.retain(|table| {
    //     (*table).toggle(number);
    //     true
    // })
    // for (i, table) in tables.iter_mut().enumerate() {
    //     table.toggle(number);
    //     if table.found_full() && !found_table_indicies.contains(&i) {
    //         println!("{:?} <-> {} ", table, number);
    //         found_table_indicies.push(i);
    //     }
    //     table.delete();
    // if found_table_indicies.len() == tables_length - 1 {
    //     println!("{}", number);
    //     let not_found_indicies = (0..tables_length)
    //         .clone()
    //         .filter(|idx| !found_table_indicies.contains(idx))
    //         .collect::<Vec<usize>>();
    //     let not_found_index = not_found_indicies.get(0).unwrap();
    //     let missing_numbers = tables
    //         .get(*not_found_index)
    //         .unwrap()
    //         .clone()
    //         .get_unused_numbers();
    //     let missings = missing_numbers
    //         .clone()
    //         .into_iter()
    //         .reduce(|a, b| a + b)
    //         .unwrap();
    //     println!(
    //         "{} , {}, {:?} -> {}",
    //         number,
    //         missings,
    //         missing_numbers,
    //         (number as u128) * missings
    //     );
    //     return;
    // }
    // if table.found_full() {
    //     let missings = table
    //         .clone()
    //         .get_unused_numbers()
    //         .into_iter()
    //         .reduce(|a, b| a + b)
    //         .unwrap();
    //     println!(
    //         "{} , {} -> {}",
    //         number,
    //         missings,
    //         (number as u128) * missings
    //     );
    //     return;
    // }
    // }
    // }
}
