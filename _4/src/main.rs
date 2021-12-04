use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Elem(u8, bool);

impl FromStr for Elem {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Elem(s.parse::<u8>()?, false))
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

fn main() {
    let content = include_str!("../input.test")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = content[0]
        .split(",")
        .into_iter()
        .filter_map(|e| e.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    let tables: Vec<Table> = (&content[1..])
        .into_iter()
        .filter_map(|&table| Table::from_str(table).ok())
        .collect::<Vec<Table>>();
    println!("{:?}", tables);
    println!("{:?}", choosen_numbers);
}
