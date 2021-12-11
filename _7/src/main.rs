use std::collections::HashMap;

fn get_sum_of_until(until: i64) -> i64 {
    until * (until + 1) / 2
}

fn main() {
    let values: Vec<&str> = include_str!("../input.data").lines().collect();

    let numbers = values[0]
        .split(",")
        .filter_map(|n| n.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let min = *numbers.iter().min().unwrap() as usize;
    let max = *numbers.iter().max().unwrap() as usize;

    let mut costs: HashMap<usize, i64> = HashMap::default();

    for i in min..max {
        let mut distances: i64 = 0;
        for number in numbers.iter() {
            distances += get_sum_of_until((*number as i64 - i as i64).abs() as i64);
        }
        costs.insert(i, distances);
    }
    let mut at: i8 = i8::MIN;
    let mut value: i64 = i64::MAX;

    for (at_index, cost) in costs.into_iter() {
        if cost < value {
            at = at_index as i8;
            value = cost;
        }
    }

    println!("{} : {}", at, value);
}
