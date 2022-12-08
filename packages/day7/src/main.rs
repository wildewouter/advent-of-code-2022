use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input = read::file(&path);

    let mut folder_size: HashMap<String, usize> = HashMap::new();
    let mut all_directories: BTreeMap<String, HashSet<String>> = BTreeMap::new();

    let directory_match = Regex::new(r"cd (?P<directory>(\w|/))").unwrap();
    let file_size_match = Regex::new(r"(?P<file_size>\d+) \w(\.\w+)?").unwrap();
    let upwards_dir_match = Regex::new(r"cd \.\.").unwrap();
    let sub_dir_match = Regex::new(r"dir (?P<directory>\w)").unwrap();

    let mut current_dir: String = "/".to_string();
    let mut previous_dir: String = String::new();

    let lines = input.split('$').collect::<Vec<&str>>();

    for command_and_output in lines {
        if command_and_output.is_empty() {
            continue;
        }

        let default: HashSet<String> = HashSet::new();
        all_directories.insert(
            current_dir.clone(),
            match all_directories.get(&current_dir) {
                Some(val) => val.clone(),
                None => default.clone(),
            },
        );

        let text: Vec<&str> = command_and_output.lines().collect();
        let dir = match directory_match.captures(text[0]) {
            Some(captures) => Ok(captures["directory"].parse::<String>().unwrap()),
            None => Err(()),
        };

        if let Ok(cur_dir) = dir {
            previous_dir = current_dir;
            current_dir = cur_dir;
            continue;
        }

        if upwards_dir_match.captures(text[0]).is_some() {
            current_dir = previous_dir.clone();
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

            let sub_dir = match sub_dir_match.captures(line) {
                Some(captures) => Ok(captures["directory"].parse::<String>().unwrap()),
                None => Err(()),
            };

            if let Ok(dir) = sub_dir {
                let mut dirs = match all_directories.get(&current_dir) {
                    Some(dirs) => dirs.clone(),
                    None => default.clone(),
                };

                dirs.insert(dir);

                all_directories.insert(current_dir.clone(), dirs);
            }
        }

        folder_size.insert(current_dir.clone(), total_size);
    }

    let mut answer: HashMap<String, usize> = HashMap::new();

    for current in all_directories.keys() {
        answer.insert(
            current.clone(),
            drill(current, &all_directories, &folder_size),
        );
    }

    println!("Day S7v7n: /// WHATS IN THE BOX");
    println!(
        "Part One: {}",
        answer
            .iter()
            .filter(|(_, &size)| size <= 100000_usize)
            .map(|(_, &size)| size)
            .sum::<usize>()
    );
}

fn drill(
    sub_dir: &str,
    all_directories: &BTreeMap<String, HashSet<String>>,
    folder_size: &HashMap<String, usize>,
    tail_size: usize,
) -> usize {
    let sub_dirs = &all_directories.get(sub_dir).unwrap();

    let size = *folder_size.get(sub_dir).unwrap();

    if sub_dirs.is_empty() {
        return tail_size + size;
    }

    // sub_dirs.nex

    // for sub_dir in subs.iter() {
    //     drill(sub_dir, all_directories, folder_size, tail_size);
    // }
}
