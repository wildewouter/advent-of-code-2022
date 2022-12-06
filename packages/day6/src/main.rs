fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input: String = read::file(&path);

    println!("Day Six");
    println!("Part One: {}", find_answer(&input, 4));
    println!("Part Two: {}", find_answer(&input, 14));
}

fn find_answer(input: &str, amount: usize) -> usize {
    let chars: Vec<(usize, char)> = input.chars().enumerate().collect();

    let mut index: usize = 0;

    for compare in chars.windows(amount) {
        let mut c: Vec<&char> = compare.iter().map(|(_, v)| v).collect();
        c.sort();
        c.dedup();
        if c.len() == amount {
            let (i, _) = compare.last().unwrap();

            index = i + 1;
            break;
        }
    }
    index
}
