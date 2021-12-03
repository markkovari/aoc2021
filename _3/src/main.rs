fn add_vectors(a: Vec<u128>, b: Vec<u128>) -> Vec<u128> {
    a.iter()
        .zip(b)
        .map(|(a, b)| a + b)
        .collect::<Vec<u128>>()
        .clone()
}

fn main() {
    let binary_numbers: Vec<Vec<u128>> = include_str!("../input.data")
        .lines()
        .map(|line| {
            line.split("")
                .filter_map(|e| e.parse::<u128>().ok())
                .collect()
        })
        .collect();

    let lines_length = binary_numbers.len();

    let sum_of_binaries = binary_numbers
        .into_iter()
        .reduce(|acc, item| add_vectors(acc, item))
        .unwrap();

    let below_or_above_average = sum_of_binaries
        .iter()
        .map(|item| *item > (lines_length / 2) as u128)
        .collect::<Vec<_>>();

    println!("{:?}", below_or_above_average);
    let element_count = below_or_above_average.len();
    println!("element count:{}", element_count);
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
}
