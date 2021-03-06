use std::io::{stdin, BufRead};
use std::collections::HashSet;

fn main() {
    let changes: Vec<i32> = stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect();

    println!("Part 1: {}", changes.iter().sum::<i32>());

    let mut reached_frequencies = HashSet::new();
    let frequency = changes
        .iter()
        .cycle()
        .scan(0, |frequency, &change| {
            *frequency += change;
            Some(*frequency)
        })
        .find(|frequency| !reached_frequencies.insert(*frequency))
        .unwrap();
    println!("Part 2: {}", frequency);
}
