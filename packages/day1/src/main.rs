use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let summed_calories: Vec<usize> = read::file(&path)
        .split("\n\n")
        .filter_map(|caloric_bag| {
            let calories: usize = caloric_bag
                .lines()
                .filter_map(|amount| amount.parse::<usize>().ok())
                .into_iter()
                .sum();

            Some(calories)
        })
        .collect();

    println!("Day One");
    println!("Part one: {}", summed_calories.iter().max().unwrap());
}
