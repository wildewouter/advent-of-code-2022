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
    println!("Part two: {}", part_two(&summed_calories));
}

fn part_two(summed_calories: &[usize]) -> usize {
    let mut sorted_calories: Vec<usize> = summed_calories.to_vec();
    sorted_calories.sort();
    sorted_calories.iter().rev().take(3).sum()
}
