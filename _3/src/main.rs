fn main() {
    let content = include_str!("../input.test");
    let binary_numbers: Vec<Vec<u8>> = content.lines().map(|line| line.split("").filter_map(|e| e.parse::<u8>().ok()).collect()).collect();
    println!("{:?}", binary_numbers);
}
