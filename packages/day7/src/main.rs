use regex::Regex;
use std::collections::HashMap;

fn main() {
    let read_path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input = read::file(&read_path);

    let mut folder_size: HashMap<Vec<String>, usize> = HashMap::new();

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

        let mut total_size: usize = 0;

        for line in text {
            let file_size = match file_size_match.captures(line) {
                Some(captures) => Ok(captures["file_size"].parse::<usize>().unwrap()),
                None => Err(()),
            };

            if let Ok(size) = file_size {
                total_size += size
            }
        }

        folder_size.insert(path.clone(), total_size);
    }

    let ans = folder_size
        .iter()
        .map(|(k, _)| {
            folder_size
                .iter()
                .filter(|(i, _)| i.join("/").starts_with(&k.join("/")))
                .map(|(_, size)| size)
                .sum::<usize>()
        })
        .filter(|size| *size <= 100000_usize)
        .sum::<usize>();

    println!("Day S7v7n: /// WHATS IN THE BOX");
    println!("Part One: {}", ans);
}
