use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn main() {
    let mut positions: HashMap<char, usize> = HashMap::new();
    for (index, letter) in ALPHABET.chars().enumerate() {
        positions.insert(letter, index + 1);
    }

    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input = read::file(&path);

    let total: usize = input
        .lines()
        .map(|ruck_sack| ruck_sack.split_at(ruck_sack.len() / 2))
        .map(|(left, right)| left.chars().find(|v| right.chars().contains(v)).unwrap())
        .filter_map(|a| positions.get(&a))
        .sum();

    let total2: usize = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(one, two, three)| -> usize {
            let a = HashSet::<char>::from_iter(one.chars());
            let b = HashSet::<char>::from_iter(two.chars());
            let c = HashSet::<char>::from_iter(three.chars());

            let intersection: HashSet<_> = a.intersection(&b).copied().collect();
            let intersection2: HashSet<_> = intersection.intersection(&c).collect();

            intersection2
                .iter()
                .filter_map(|item| positions.get(item))
                .sum()
        })
        .sum();

    println!("Day Three");
    println!("Part One: {}", total);
    println!("Part Two: {}", total2);
}
