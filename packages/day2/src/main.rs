fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let lines: Vec<String> = read::file(&path).lines().map(|c| c.to_string()).collect();

    println!("Day Two");
    println!(
        "Part one: {}",
        lines
            .iter()
            .map(|line| {
                let mut moves = line.split_whitespace();
                let opponent_move = &moves.next().unwrap().to_string();
                let player_move = &moves.next().unwrap().to_string();

                determine_player_scores_part_one(&(opponent_move, player_move))
            })
            .sum::<usize>()
    );
    println!(
        "Part two: {}",
        lines
            .iter()
            .map(|line| {
                let mut moves = line.split_whitespace();
                let opponent_move = &moves.next().unwrap().to_string();
                let player_move = &moves.next().unwrap().to_string();

                determine_player_scores_part_two(&(opponent_move, player_move))
            })
            .sum::<usize>()
    );
}

fn determine_player_scores_part_one(play: &(&str, &str)) -> usize {
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

fn determine_player_scores_part_two(play: &(&str, &str)) -> usize {
    match play {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        _ => 0,
    }
}
