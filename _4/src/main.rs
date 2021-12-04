use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Elem(u8, bool);

#[derive(Debug)]
struct Row(Vec<Elem>, bool);

#[derive(Debug)]
struct Table(Vec<Row>, bool);

impl FromStr for Elem {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<u8>()?;
        Ok(Elem(num, false))
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
        .map(|&table| {
            Table(
                table
                    .split("\n")
                    .map(|row| {
                        Row(
                            row.split(" ")
                                .filter_map(|elem| Elem::from_str(elem).ok())
                                .collect::<Vec<Elem>>(),
                            false,
                        )
                    })
                    .collect::<Vec<Row>>(),
                false,
            )
        })
        .collect::<Vec<Table>>();
    println!("{:?}", tables);
    println!("{:?}", choosen_numbers);
}
