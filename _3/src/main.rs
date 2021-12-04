fn add_vectors(a: Vec<u128>, b: Vec<u128>) -> Vec<u128> {
    a.iter()
        .zip(b)
        .map(|(a, b)| a + b)
        .collect::<Vec<u128>>()
        .clone()
}

fn get_most_common(numbers: Vec<Vec<u128>>, most_commons: Vec<u128>) -> Vec<u128> {
    let mut remaining: Vec<Vec<u128>> = numbers.clone();
    for (position, bit) in most_commons.iter().enumerate() {
        if remaining.len() == 1 {
            return remaining.iter().nth(0).unwrap().to_vec();
        } else if remaining
            .clone()
            .into_iter()
            .all(|e| &e == remaining.get(0).unwrap())
        {
            return remaining.iter().nth(0).unwrap().to_vec();
        }
        remaining = remaining
            .into_iter()
            .filter(|e| e.get(position).unwrap() == bit)
            .collect::<Vec<_>>();
    }
    remaining.iter().nth(0).unwrap().to_vec()
}

fn main() {
    let binary_numbers: Vec<Vec<u128>> = include_str!("../input.test")
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|e| e.parse::<u128>().ok())
                .collect()
        })
        .collect();

    let lines_length = binary_numbers.len();

    let sum_of_binaries = binary_numbers
        .clone()
        .into_iter()
        .reduce(|acc, item| add_vectors(acc, item))
        .unwrap();

    let below_or_above_average = sum_of_binaries
        .iter()
        .map(|item| *item > (lines_length / 2) as u128)
        .collect::<Vec<_>>();

    let most_common = below_or_above_average
        .iter()
        .copied()
        .map(|e| if e { 1 } else { 0 })
        .collect::<Vec<u128>>();

    println!("{:?}", sum_of_binaries);
    let most_common_one_if_eq: Vec<u128> = sum_of_binaries
        .iter()
        .copied()
        .map(|e| {
            return if e >= (lines_length / 2) as u128 {
                1
            } else {
                0
            };
        })
        .collect::<Vec<u128>>();

    println!("{:?}", most_common_one_if_eq);
    let gamma_value = below_or_above_average
        .iter()
        .enumerate()
        .map(|(e, i)| {
            if *i {
                u128::pow(
                    2,
                    (below_or_above_average.len() - 1 - e).try_into().unwrap(),
                )
            } else {
                0
            }
        })
        .reduce(|a, b| a + b)
        .unwrap();

    let epsilon_value =
        u128::pow(2, (below_or_above_average.len()).try_into().unwrap()) - gamma_value - 1;

    println!(
        "gamma {:?} epsilon {:?} result {:?}",
        gamma_value,
        epsilon_value,
        (epsilon_value * gamma_value)
    );

    println!(
        "{:?}",
        get_most_common(binary_numbers, most_common_one_if_eq)
    );
}
