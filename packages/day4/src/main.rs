use array_tool::vec::Intersect;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input = read::file(&path);

    println!("Day Four:");
    println!("Part One: {}", part_one(&input));
}

fn part_one(input: &str) -> usize {
    let duplicate_ranges: Vec<Vec<Vec<usize>>> = input
        .lines()
        .map(|pair| -> Vec<Vec<usize>> {
            pair.split(',')
                .map(|split_pair| -> Vec<usize> {
                    let positions: Vec<&str> = split_pair.split('-').collect();

                    (positions[0].parse::<usize>().unwrap()
                        ..(positions[1].parse::<usize>().unwrap()) + 1)
                        .collect()
                })
                .collect()
        })
        .filter(|pair| {
            let total_intersect = pair[0].intersect(pair[1].clone());
            total_intersect.len() == pair[0].len() || total_intersect.len() == pair[1].len()
        })
        .collect();

    duplicate_ranges.len()
}
