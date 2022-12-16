use std::collections::HashSet;

use anyhow::Ok;
use util::read_day9;

fn main() -> anyhow::Result<()> {
    let instructions = read_day9(Some("./input.txt".into()))?;
    println!("Part one: {}", solve_part_one(&instructions));
    println!("Part two: {:?}", solve_part_two(&instructions));
    Ok(())
}

fn solve_part_one(instructions: &Vec<(char, i32)>) -> usize {
    let (mut position_head, mut position_tail) = ((0, 0), (0, 0));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (direction, range) in instructions {
        for _ in 0..*range {
            match direction {
                'U' => {
                    position_head.1 += 1;
                }
                'R' => {
                    position_head.0 += 1;
                }
                'D' => {
                    position_head.1 -= 1;
                }
                'L' => {
                    position_head.0 -= 1;
                }
                _ => {}
            }
            let (change_in_x, change_in_y) = match distance(position_head, position_tail) {
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (1, 2) => (1, 1),
                (1, -2) => (1, -1),
                (-1, 2) => (-1, 1),
                (-1, -2) => (-1, -1),
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                (_, _) => (0, 0),
            };
            position_tail.0 += change_in_x;
            position_tail.1 += change_in_y;
            visited.insert(position_tail);
        }
    }
    visited.len()
}

fn solve_part_two(instructions: &Vec<(char, i32)>) -> usize {
    let mut heads_and_tails = initial_heads_and_tails();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (direction, range) in instructions {
        for _ in 0..*range {
            for index in 0..heads_and_tails.len() - 1 {
                if index == 0 {
                    match direction {
                        'U' => {
                            heads_and_tails[index].1 += 1;
                        }
                        'R' => {
                            heads_and_tails[index].0 += 1;
                        }
                        'D' => {
                            heads_and_tails[index].1 -= 1;
                        }
                        'L' => {
                            heads_and_tails[index].0 -= 1;
                        }
                        _ => {}
                    }
                }
                let (change_in_x, change_in_y) =
                    match distance(heads_and_tails[index], heads_and_tails[index + 1]) {
                        (2, 0) => (1, 0),
                        (-2, 0) => (-1, 0),
                        (0, 2) => (0, 1),
                        (0, -2) => (0, -1),
                        (2, 2) => (1, 1),
                        (-2, 2) => (-1, 1),
                        (2, -2) => (1, -1),
                        (-2, -2) => (-1, -1),
                        (1, 2) => (1, 1),
                        (1, -2) => (1, -1),
                        (-1, 2) => (-1, 1),
                        (-1, -2) => (-1, -1),
                        (2, 1) => (1, 1),
                        (2, -1) => (1, -1),
                        (-2, 1) => (-1, 1),
                        (-2, -1) => (-1, -1),
                        (_, _) => (0, 0),
                    };
                heads_and_tails[index + 1].0 += change_in_x;
                heads_and_tails[index + 1].1 += change_in_y;
                if index == 8 {
                    visited.insert(heads_and_tails[9]);
                }
            }
        }
    }
    visited.len()
}

fn distance((x_one, y_one): (i32, i32), (x_two, y_two): (i32, i32)) -> (i32, i32) {
    (x_one - x_two, y_one - y_two)
}

fn initial_heads_and_tails() -> Vec<(i32, i32)> {
    let mut heads_and_tails = Vec::new();
    for _ in 0..=9 {
        heads_and_tails.push((0, 0));
    }
    heads_and_tails
}
