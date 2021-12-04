#[derive(Debug)]
struct Elem(u8, bool);

#[derive(Debug)]
struct Row(Vec<Elem>, bool);

fn main() {
    let content = include_str!("../input.test")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = content[0]
        .split(",")
        .into_iter()
        .filter_map(|e| e.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    let tables_raw = &content[1..];

    let tables: Vec<Vec<Row>> = tables_raw
        .into_iter()
        .map(|&table| {
            table
                .split("\n")
                .map(|row| {
                    Row(
                        row.split(" ")
                            .filter_map(|elem| elem.parse::<u8>().ok())
                            .map(|e| Elem(e, false))
                            .collect::<Vec<Elem>>(),
                        false,
                    )
                })
                .collect::<Vec<Row>>()
        })
        .collect::<Vec<Vec<Row>>>();
    println!("{:?}", tables);
    println!("{:?}", choosen_numbers);
}
