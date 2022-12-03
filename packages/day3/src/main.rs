use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Iterator;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn main() {
    let mut positions: HashMap<char, usize> = HashMap::new();
    for (index, letter) in ALPHABET.chars().enumerate() {
        positions.insert(letter, index + 1);
    }

    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let total: usize = read::file(&path)
        .lines()
        .map(|ruck_sack| ruck_sack.split_at(ruck_sack.len() / 2))
        .map(|(left, right)| left.chars().find(|v| right.chars().contains(v)).unwrap())
        .filter_map(|a| positions.get(&a))
        .sum();

    println!("Day Three");
    println!("Part One: {}", total);
}
