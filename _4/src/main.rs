fn main() {
    let content = include_str!("../input.test")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let choosen_numbers = content[0]
        .split(",")
        .into_iter()
        .filter_map(|e| e.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    println!("{:?}", choosen_numbers);
}
