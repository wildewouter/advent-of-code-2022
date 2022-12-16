use regex::Regex;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let ans = read::file(&path)
        .split("\n")
        .map(|command| match command {
            "noop" => (0, 1),
            _ => {
                let amount = Regex::new(r"(?P<amount>-?\d+)").unwrap();

                match amount.captures(command) {
                    Some(captures) => (captures["amount"].parse::<i32>().unwrap(), 2),
                    None => (0, 1),
                }
            }
        })
        .fold(
            vec![(0, 1)],
            |cycle_map: Vec<(usize, i32)>, (amount, cycles)| {
                let last_entry = cycle_map.last().unwrap_or(&(0_usize, 1));

                let mut cycle_map_new = cycle_map.clone();

                for index in 1..cycles {
                    cycle_map_new.push((last_entry.0 + index, last_entry.1));
                }

                cycle_map_new.push((last_entry.0 + cycles, amount + last_entry.1));

                cycle_map_new
            },
        );

    println!("Day ten:");
    println!(
        "Part One: {}",
        vec![20, 60, 100, 140, 180, 220]
            .iter()
            .fold(0, |result, next| {
                ans[(next - 1) as usize].1 * next + result
            })
    );
}
