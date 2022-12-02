use std::str::FromStr;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let matches: Vec<usize> = read::file(&path)
        .lines()
        .map(|line| {
            let mut moves = line.split_whitespace();
            let opponent_move = &moves.next().unwrap().to_string();
            let player_move = &moves.next().unwrap().to_string();

            determine_player_scores(&(opponent_move, player_move))
        })
        .collect();

    println!("Day Two");
    println!("Part one: {}", matches.iter().sum::<usize>());
}

fn determine_player_scores(play: &(&str, &str)) -> usize {
    match play {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => 0,
    }
}
