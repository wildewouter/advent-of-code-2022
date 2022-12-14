use std::collections::HashSet;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input: String = read::file(&path);

    println!("Day Six");
    println!("Part One: {}", find_answer(&input, 4).unwrap());
    println!("Part Two: {}", find_answer(&input, 14).unwrap());
}

fn find_answer(input: &str, amount: usize) -> Option<usize> {
    for compare in input
        .chars()
        .enumerate()
        .collect::<Vec<(usize, char)>>()
        .windows(amount)
    {
        if compare
            .iter()
            .map(|(_, v)| v)
            .collect::<HashSet<&char>>()
            .len()
            == amount
        {
            let (i, _) = compare.last().unwrap();

            return Some(i + 1);
        }
    }
    None
}
