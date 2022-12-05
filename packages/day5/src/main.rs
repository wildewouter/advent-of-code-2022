use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input: String = read::file(&path);

    let split_input: Vec<&str> = input.split("\n\n").collect();
    let crates = split_input[0];
    let moves = split_input[1];

    let regex_for_row_positions = Regex::new(r"(\n(?P<positions>( +(\d))+))").unwrap();
    let mut row_positions: HashMap<usize, char> = HashMap::new();
    let positional_row = match regex_for_row_positions.captures(crates) {
        Some(captures) => Ok(captures["positions"].parse::<String>().unwrap()),
        None => Err(()),
    }
    .unwrap();
    for (index, position) in positional_row.chars().enumerate() {
        if position.is_numeric() {
            row_positions.insert(index, position);
        }
    }

    let mut dock = build_initial_dock(crates, &row_positions);
    let moves = build_move_list(moves);

    println!("Day Five");
    println!("Part One: {}", part_one(&mut dock.clone(), &moves));
    println!("Part Two: {}", part_two(&mut dock, &moves));
}

fn part_one(mut dock: &mut HashMap<char, Vec<char>>, moves: &Vec<(usize, char, char)>) -> String {
    for (amount, origin, destination) in moves {
        move_to_row_in_dock(&amount, &mut dock, &origin, &destination)
    }

    let mut ans: String = String::new();

    for key in dock.keys().sorted() {
        ans.push(*dock.get(key).unwrap().first().unwrap());
    }
    ans
}

fn part_two(mut dock: &mut HashMap<char, Vec<char>>, moves: &Vec<(usize, char, char)>) -> String {
    for (amount, origin, destination) in moves {
        move_multiple_to_row_in_dock(&amount, &mut dock, &origin, &destination)
    }

    let mut ans: String = String::new();

    for key in dock.keys().sorted() {
        ans.push(*dock.get(key).unwrap().first().unwrap());
    }
    ans
}

fn build_initial_dock(
    crates: &str,
    row_positions: &HashMap<usize, char>,
) -> HashMap<char, Vec<char>> {
    let mut dock: HashMap<char, Vec<char>> = HashMap::new();

    for line in crates.lines() {
        for (index, character) in line.chars().enumerate() {
            if character.is_alphabetic() {
                let position = row_positions.get(&index).unwrap();
                let default_row: Vec<char> = Vec::new();
                let mut dock_row: Vec<char> = match dock.get(position) {
                    Some(row) => row,
                    None => &default_row,
                }
                .clone();
                dock_row.push(character);
                dock.insert(*position, dock_row);
            }
        }
    }

    dock
}

fn move_to_row_in_dock(
    move_amount: &usize,
    dock: &mut HashMap<char, Vec<char>>,
    origin: &char,
    destination: &char,
) {
    let mut origin_row = dock.get(origin).unwrap().clone();
    let mut destination_row = dock.get(destination).unwrap().clone();

    let characters: Vec<char> = origin_row.drain(0..*move_amount).collect();

    dock.insert(*origin, origin_row);

    destination_row.reverse();
    for char in characters {
        destination_row.push(char);
    }
    destination_row.reverse();
    dock.insert(*destination, destination_row);
}

fn move_multiple_to_row_in_dock(
    move_amount: &usize,
    dock: &mut HashMap<char, Vec<char>>,
    origin: &char,
    destination: &char,
) {
    let mut origin_row = dock.get(origin).unwrap().clone();
    let mut destination_row = dock.get(destination).unwrap().clone();

    let mut characters: Vec<char> = origin_row.drain(0..*move_amount).collect();
    characters.reverse();

    dock.insert(*origin, origin_row);

    destination_row.reverse();
    for char in characters {
        destination_row.push(char);
    }
    destination_row.reverse();
    dock.insert(*destination, destination_row);
}

fn build_move_list(moves: &str) -> Vec<(usize, char, char)> {
    moves
        .lines()
        .into_iter()
        .map(|line| {
            match Regex::new(r"move (?P<amount>\d+) from (?P<origin>\d+) to (?P<destination>\d+)")
                .unwrap()
                .captures(line)
            {
                Some(captures) => Ok((
                    captures["amount"].parse::<usize>().unwrap(),
                    captures["origin"].parse::<char>().unwrap(),
                    captures["destination"].parse::<char>().unwrap(),
                )),
                None => Err(()),
            }
            .unwrap()
        })
        .collect()
}
