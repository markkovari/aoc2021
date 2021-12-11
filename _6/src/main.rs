fn main() {
  let raw_values = include_str!("../input.data")
    .split(",")
    .collect::<Vec<&str>>();

  let mut values = raw_values
    .iter()
    .filter_map(|e| e.parse().ok())
    .collect::<Vec<u128>>();

  println!("{:?}", values);
  let generations: u128 = 256;
  for generation in 0..generations {
    for i in 0..values.len() {
      if values[i] == 0 {
        values[i] = 6;
        values.push(8);
      } else {
        values[i] -= 1;
      }
    }
    println!("{} -> {:?}", generation, values.len());
  }
}
