use std::fs;

fn calculate_growings(numbers: Vec<u64>) -> u64 {
    let mut count = 0;
    for i in 0..(numbers.len() - 1) {
        if numbers[i] < numbers[i + 1] {
            count += 1;
        }
    }
    count
}
fn main() {
    let file_name = "input.txt";
    let content = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let depths: std::vec::Vec<u64> = content
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect::<Vec<u64>>();

    println!("{}", calculate_growings(depths));
}
