fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input: String = read::file(&path);

    println!("Day Six");
    println!("Part One: {}", part_one(&input));
}

fn part_one(input: &str) -> usize {
    let chars: Vec<(usize, char)> = input.chars().enumerate().collect();

    let mut index: usize = 0;

    for compare in chars.windows(4) {
        let mut c: Vec<&char> = compare.iter().map(|(_, v)| v).collect();
        c.sort();
        c.dedup();
        if c.len() == 4 {
            let (i, _) = compare.last().unwrap();

            index = i + 1;
            break;
        }
    }
    index
}
