use std::str::FromStr;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let matches: Vec<Match> = read::file(&path)
        .lines()
        .filter_map(|line| line.parse::<Match>().ok())
        .collect();

    println!("Day Two");
    println!(
        "Part one: {}",
        matches
            .iter()
            .map(|played_match| played_match.player_score)
            .sum::<usize>()
    );
}

struct Match {
    player_score: usize,
    opponent_score: usize,
}

impl Match {
    fn new((opponent_score, player_score): (usize, usize)) -> Self {
        Match {
            player_score,
            opponent_score,
        }
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(input: &str) -> Result<Match, ()> {
        let mut moves = input.split_whitespace();
        let opponent_move = &moves.next().unwrap().to_string();
        let player_move = &moves.next().unwrap().to_string();

        Ok(Match::new(determine_player_scores((
            opponent_move,
            player_move,
        ))))
    }
}

fn determine_player_scores(play: (&str, &str)) -> (usize, usize) {
    match play {
        ("A", "X") => (4, 4),
        ("A", "Y") => (1, 8),
        ("A", "Z") => (7, 3),
        ("B", "X") => (7, 1),
        ("B", "Y") => (5, 5),
        ("B", "Z") => (2, 9),
        ("C", "X") => (3, 7),
        ("C", "Y") => (9, 2),
        ("C", "Z") => (6, 6),
        _ => (0, 0),
    }
}
