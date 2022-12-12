use std::collections::HashMap;
use std::i32;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let mut positions_visited: HashMap<(i32, i32), i32> = HashMap::new();
    let _input = read::file(&path).split('\n').into_iter().fold(
        ((0, 0), (0, 0)),
        |positions: ((i32, i32), (i32, i32)), next_move| {
            let split: Vec<&str> = next_move.split_whitespace().collect();
            let direction = split[0];
            let amount = split[1].parse::<i32>().unwrap();

            let (mut new_head_position, mut tail_position) = positions;

            for _ in 0..amount {
                let new_positions =
                    calculate_new_positions(&(new_head_position, tail_position), direction);

                new_head_position = new_positions.0;
                tail_position = new_positions.1;

                positions_visited.insert(
                    tail_position,
                    positions_visited.get(&tail_position).unwrap_or(&0) + 1,
                );
            }

            (new_head_position, tail_position)
        },
    );

    println!("Day nine");
    println!("Part one: {}", &positions_visited.keys().len());
    println!("Part two: {}", 0);
}

fn calculate_new_positions(
    position: &((i32, i32), (i32, i32)),
    direction: &str,
) -> ((i32, i32), (i32, i32)) {
    let new_position = match direction {
        "R" => move_right(position),
        "L" => move_left(position),
        "U" => move_up(position),
        "D" => move_down(position),
        _ => *position,
    };

    if new_position.0 == position.1 {
        return (new_position.0, new_position.0);
    }

    new_position
}

fn move_right(
    ((x_head, y_head), (x_tail, y_tail)): &((i32, i32), (i32, i32)),
) -> ((i32, i32), (i32, i32)) {
    if ((y_head - 1 == *y_tail || y_head + 1 == *y_tail) && x_head == x_tail)
        || leave_tail_in_place(&((*x_head, *y_head), (*x_tail, *y_tail)), "R")
    {
        return ((x_head + 1, *y_head), (*x_tail, *y_tail));
    }

    ((x_head + 1, *y_head), (*x_head, *y_head))
}

fn move_left(
    ((x_head, y_head), (x_tail, y_tail)): &((i32, i32), (i32, i32)),
) -> ((i32, i32), (i32, i32)) {
    if ((y_head - 1 == *y_tail || y_head + 1 == *y_tail) && x_head == x_tail)
        || leave_tail_in_place(&((*x_head, *y_head), (*x_tail, *y_tail)), "L")
    {
        return ((x_head - 1, *y_head), (*x_tail, *y_tail));
    }

    ((x_head - 1, *y_head), (*x_head, *y_head))
}

fn move_down(
    ((x_head, y_head), (x_tail, y_tail)): &((i32, i32), (i32, i32)),
) -> ((i32, i32), (i32, i32)) {
    if ((x_head - 1 == *x_tail || x_head + 1 == *x_tail) && y_head == y_tail)
        || leave_tail_in_place(&((*x_head, *y_head), (*x_tail, *y_tail)), "D")
    {
        return ((*x_head, y_head - 1), (*x_tail, *y_tail));
    }

    ((*x_head, y_head - 1), (*x_head, *y_head))
}

fn move_up(
    ((x_head, y_head), (x_tail, y_tail)): &((i32, i32), (i32, i32)),
) -> ((i32, i32), (i32, i32)) {
    if ((x_head - 1 == *x_tail || x_head + 1 == *x_tail) && y_head == y_tail)
        || leave_tail_in_place(&((*x_head, *y_head), (*x_tail, *y_tail)), "U")
    {
        return ((*x_head, y_head + 1), (*x_tail, *y_tail));
    }

    ((*x_head, y_head + 1), (*x_head, *y_head))
}

fn leave_tail_in_place(
    ((x_head, y_head), (x_tail, y_tail)): &((i32, i32), (i32, i32)),
    direction: &str,
) -> bool {
    match direction {
        "R" => i32::abs(y_head - y_tail) == 1 && i32::abs(x_head + 1 - x_tail) == 0,
        "U" => i32::abs(y_head + 1 - y_tail) == 0 && i32::abs(x_head - x_tail) == 1,
        "L" => i32::abs(y_head - y_tail) == 1 && i32::abs(x_head - 1 - x_tail) == 0,
        "D" => i32::abs(y_head - 1 - y_tail) == 0 && i32::abs(x_head - x_tail) == 1,
        _ => false,
    }
}
