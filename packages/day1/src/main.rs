use itertools::Itertools;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let summed_calories: Vec<usize> = read::file(&path)
        .split("\n\n")
        .map(|caloric_bag| {
            caloric_bag
                .lines()
                .filter_map(|amount| amount.parse::<usize>().ok())
                .into_iter()
                .sum()
        })
        .sorted()
        .rev()
        .collect();

    println!("Day One");
    println!("Part one: {}", summed_calories.iter().max().unwrap());
    println!(
        "Part two: {}",
        summed_calories.iter().take(3).sum::<usize>()
    );
}
