use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let read_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input = read::file(&read_path);

    let mut folder_size: HashMap<Vec<String>, i32> = HashMap::new();

    let move_into_dir_match = Regex::new(r"cd (?P<directory>(\w|/)+)").unwrap();
    let file_size_match = Regex::new(r"(?P<file_size>\d+) \w(\.\w+)?").unwrap();
    let upwards_dir_match = Regex::new(r"cd \.\.").unwrap();

    let mut path: Vec<String> = Vec::new();

    for command_and_output in input.split('$').collect::<Vec<&str>>() {
        if command_and_output.is_empty() {
            continue;
        }

        let text: Vec<&str> = command_and_output.lines().collect();

        let move_into_dir_command = match move_into_dir_match.captures(text[0]) {
            Some(captures) => Ok(captures["directory"].parse::<String>().unwrap()),
            None => Err(()),
        };

        if let Ok(new_dir) = move_into_dir_command {
            path.push(new_dir);
            continue;
        }

        if upwards_dir_match.captures(text[0]).is_some() {
            path.pop();
            continue;
        };

        let mut total_size: i32 = 0;

        for line in text {
            let file_size = match file_size_match.captures(line) {
                Some(captures) => Ok(captures["file_size"].parse::<i32>().unwrap()),
                None => Err(()),
            };

            if let Ok(size) = file_size {
                total_size += size
            }
        }

        folder_size.insert(path.clone(), total_size);
    }

    let aggregated_sizes = folder_size
        .iter()
        .map(|(k, _)| {
            let key = k.join("/");
            (
                key.clone(),
                folder_size
                    .iter()
                    .filter(|(i, _)| i.join("/").starts_with(&key))
                    .map(|(_, size)| size)
                    .sum::<i32>(),
            )
        })
        .collect::<Vec<(String, i32)>>();

    println!("Day S7v7n: /// WHATS IN THE BOX");
    println!(
        "Part One: {}",
        aggregated_sizes
            .iter()
            .filter(|(_, size)| *size <= 100000)
            .map(|(_, v)| v)
            .sum::<i32>()
    );
    println!("Part Two: {}", part_two(&aggregated_sizes))
}

fn part_two(aggs: &[(String, i32)]) -> i32 {
    let max = 70000000;

    let root_size = aggs
        .iter()
        .find(|(key, _)| key == "/")
        .map(|(_, v)| v)
        .unwrap();

    let needed = 30000000 - (max - root_size);

    *aggs
        .iter()
        .filter_map(|(_, v)| match v >= &needed {
            true => Some(*v),
            false => None,
        })
        .sorted()
        .collect::<Vec<i32>>()
        .first()
        .unwrap()
}
