use std::collections::HashMap;
use std::i32;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));
    let input: Vec<String> = read::file(&path)
        .split('\n')
        .map(|c| c.to_string())
        .collect();

    println!("Day nine");
    println!("Part one: {}", &part_one(&input));
    println!("Part two: {}", &part_two(&input));
}

fn part_one(input: &[String]) -> usize {
    let mut positions_visited: HashMap<(i32, i32), i32> = HashMap::new();

    input.iter().fold(
        ((0, 0), (0, 0)),
        |positions: ((i32, i32), (i32, i32)), next_move| {
            let split: Vec<&str> = next_move.split_whitespace().collect();
            let direction = split[0];
            let amount = split[1].parse::<i32>().unwrap();

            let (mut head_position, mut tail_position) = positions;

            for _ in 0..amount {
                head_position = move_head(direction, &head_position);
                tail_position = find_next_tail_position(&(head_position, tail_position));

                positions_visited.insert(
                    tail_position,
                    positions_visited.get(&tail_position).unwrap_or(&0) + 1,
                );
            }

            (head_position, tail_position)
        },
    );

    positions_visited.keys().len()
}

fn move_head(direction: &str, (x, y): &(i32, i32)) -> (i32, i32) {
    match direction {
        "R" => (x + 1, *y),
        "L" => (x - 1, *y),
        "U" => (*x, y + 1),
        "D" => (*x, y - 1),
        _ => (*x, *y),
    }
}

fn part_two(input: &[String]) -> usize {
    let mut positions_visited: HashMap<(i32, i32), i32> = HashMap::new();

    let mut rope: Vec<(i32, i32)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for next_move in input.iter() {
        let split: Vec<&str> = next_move.split_whitespace().collect();
        let direction = split[0];
        let amount = split[1].parse::<i32>().unwrap();

        for _ in 0..amount {
            rope[0] = move_head(direction, &rope[0]);

            for index in 1..=9 {
                rope[index] = find_next_tail_position(&(rope[index - 1], rope[index]));
            }

            positions_visited.insert(rope[9], positions_visited.get(&rope[9]).unwrap_or(&0) + 1);
        }
    }

    positions_visited.keys().len()
}

fn find_next_tail_position(
    ((new_x_head, new_y_head), (old_x_tail, old_y_tail)): &((i32, i32), (i32, i32)),
) -> (i32, i32) {
    let x_delta = new_x_head - old_x_tail; //2,0 1,2 = 1
    let y_delta = new_y_head - old_y_tail; // 2

    if x_delta == 0 && i32::abs(y_delta) > 1 {
        let x = (*new_x_head, new_y_head - (y_delta / i32::abs(y_delta)));
        return x;
    }

    if y_delta == 0 && i32::abs(x_delta) > 1 {
        let x1 = (new_x_head - (x_delta / i32::abs(x_delta)), *new_y_head);
        return x1;
    }

    if i32::abs(x_delta) == 1 && i32::abs(y_delta) > 1 {
        let x2 = (*new_x_head, new_y_head - (y_delta / i32::abs(y_delta)));
        return x2;
    }

    if i32::abs(y_delta) == 1 && i32::abs(x_delta) > 1 {
        let x3 = (new_x_head - (x_delta / i32::abs(x_delta)), *new_y_head);
        return x3;
    }

    if i32::abs(y_delta) == 2 && i32::abs(x_delta) == 2 {
        return (
            new_x_head - (x_delta / i32::abs(x_delta)),
            new_y_head - (y_delta / i32::abs(y_delta)),
        );
    }

    (*old_x_tail, *old_y_tail)
}
