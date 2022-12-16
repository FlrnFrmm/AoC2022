use anyhow::Ok;
use util::read_day8;

fn main() -> anyhow::Result<()> {
    let grid = read_day8(Some("./input.txt".into()))?;
    println!("Part one: {}", solve_part_one(&grid));
    println!("Part two: {:?}", solve_part_two(&grid));
    Ok(())
}

fn solve_part_one(grid: &Vec<Vec<i32>>) -> i32 {
    grid.iter()
        .enumerate()
        .map(|(col_index, row)| {
            row.iter()
                .enumerate()
                .map(|(row_index, height)| {
                    if col_index == 0 || col_index == grid.len() - 1 {
                        return 1;
                    }
                    if row_index == 0 || row_index == row.len() - 1 {
                        return 1;
                    }
                    let top_visible = (0..col_index)
                        .into_iter()
                        .all(|other_height_index| grid[other_height_index][row_index] < *height);
                    let down_visible = (col_index + 1..grid.len())
                        .into_iter()
                        .all(|other_height_index| grid[other_height_index][row_index] < *height);
                    let left_visible = (row_index + 1..row.len())
                        .into_iter()
                        .all(|other_height_index| row[other_height_index] < *height);
                    let right_visible = (0..row_index)
                        .into_iter()
                        .all(|other_height_index| row[other_height_index] < *height);
                    if top_visible || left_visible || right_visible || down_visible {
                        return 1;
                    }
                    0
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn solve_part_two(grid: &Vec<Vec<i32>>) -> i32 {
    grid.iter()
        .enumerate()
        .fold(0, |total_best_score, (col_index, row)| {
            let best_score = row
                .iter()
                .enumerate()
                .fold(0, |best_score, (row_index, height)| {
                    let mut up_score = 0;
                    for col_index in (0..col_index).into_iter().rev() {
                        up_score += 1;
                        if grid[col_index][row_index] >= *height {
                            break;
                        }
                    }
                    let mut left_score = 0;
                    for row_index in (0..row_index).into_iter().rev() {
                        left_score += 1;
                        if row[row_index] >= *height {
                            break;
                        }
                    }
                    let mut down_score = 0;
                    for col_index in (col_index + 1..grid.len()).into_iter() {
                        down_score += 1;
                        if grid[col_index][row_index] >= *height {
                            break;
                        }
                    }
                    let mut right_score = 0;
                    for row_index in (row_index + 1..row.len()).into_iter() {
                        right_score += 1;
                        if row[row_index] >= *height {
                            break;
                        }
                    }
                    let total_score = up_score * right_score * left_score * down_score;
                    if total_score > best_score {
                        return total_score;
                    }
                    best_score
                });
            if best_score > total_best_score {
                return best_score;
            }
            total_best_score
        })
}
