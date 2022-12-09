use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    for (y, line) in read::file(&path).lines().enumerate() {
        for (x, value) in line.split("").filter(|char| !char.is_empty()).enumerate() {
            grid.insert((x as i32, y as i32), value.parse::<i32>().unwrap());
        }
    }

    println!("Day nine");
    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}

fn part_one(grid: &HashMap<(i32, i32), i32>) -> usize {
    let mut number_of_trees_visible = 0;

    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();

    for (position, value) in grid.iter() {
        let check_x_to_right = |(x, y): &(i32, i32)| (*x + 1, *y);
        let check_x_to_left = |(x, y): &(i32, i32)| (*x - 1, *y);
        let check_y_to_right = |(x, y): &(i32, i32)| (*x, *y + 1);
        let check_y_to_left = |(x, y): &(i32, i32)| (*x, *y - 1);

        if is_visible(check_x_to_right, position, grid, value, &(*max_x, *max_y))
            || is_visible(check_x_to_left, position, grid, value, &(*max_x, *max_y))
            || is_visible(check_y_to_right, position, grid, value, &(*max_x, *max_y))
            || is_visible(check_y_to_left, position, grid, value, &(*max_x, *max_y))
        {
            number_of_trees_visible += 1;
        }
    }

    number_of_trees_visible
}

fn part_two(grid: &HashMap<(i32, i32), i32>) -> i32 {
    let mut scenic_scores: Vec<Vec<i32>> = Vec::new();

    for (position, value) in grid.iter() {
        let check_x_to_right = |(x, y): &(i32, i32)| (*x + 1, *y);
        let check_x_to_left = |(x, y): &(i32, i32)| (*x - 1, *y);
        let check_y_to_right = |(x, y): &(i32, i32)| (*x, *y + 1);
        let check_y_to_left = |(x, y): &(i32, i32)| (*x, *y - 1);

        scenic_scores.push(vec![
            get_number_of_trees(check_x_to_right, position, grid, value),
            get_number_of_trees(check_x_to_left, position, grid, value),
            get_number_of_trees(check_y_to_right, position, grid, value),
            get_number_of_trees(check_y_to_left, position, grid, value),
        ]);
    }

    scenic_scores
        .iter()
        .map(|scores| scores.iter().product())
        .max()
        .unwrap()
}

fn is_visible(
    get_next_position: impl Fn(&(i32, i32)) -> (i32, i32),
    (x, y): &(i32, i32),
    grid: &HashMap<(i32, i32), i32>,
    check_height: &i32,
    (max_x, max_y): &(i32, i32),
) -> bool {
    if *x == 0 || *y == 0 || x == max_x || y == max_y {
        return true;
    }

    let mut current_position = get_next_position(&(*x, *y));
    let mut next_height = grid.get(&current_position);
    let mut is_visible = true;

    while next_height.is_some() && is_visible {
        let current_height = next_height.unwrap();
        is_visible = current_height < check_height;

        current_position = get_next_position(&current_position);
        next_height = grid.get(&current_position);
    }

    is_visible
}

fn get_number_of_trees(
    get_next_position: impl Fn(&(i32, i32)) -> (i32, i32),
    (x, y): &(i32, i32),
    grid: &HashMap<(i32, i32), i32>,
    check_height: &i32,
) -> i32 {
    let mut current_position = get_next_position(&(*x, *y));

    let mut next_height = grid.get(&current_position);
    let mut amount = 0;

    while next_height.is_some() {
        amount += 1;

        if next_height.unwrap() >= check_height {
            return amount;
        }

        current_position = get_next_position(&current_position);
        next_height = grid.get(&current_position);
    }

    amount
}
