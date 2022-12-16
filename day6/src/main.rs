use anyhow::Ok;
use util::read_string;

fn main() -> anyhow::Result<()> {
    let input = read_string(Some("./input.txt".into()))?;
    println!("Part one: {}", solve(&input, 4).unwrap_or_default());
    println!("Part two: {}", solve(&input, 14).unwrap_or_default());
    Ok(())
}

fn solve(input: &String, length: usize) -> Option<usize> {
    (0..input.len() - (length - 1))
        .into_iter()
        .fold(None, |acc, index| {
            if acc.is_none() {
                return check_slice(input, length, index);
            }
            acc
        })
}

fn check_slice(input: &str, length: usize, index: usize) -> Option<usize> {
    input[index..index + length]
        .chars()
        .all(|c| {
            input[index..index + length]
                .chars()
                .filter(|cc| c == *cc)
                .count()
                == 1
        })
        .then(|| index + length)
}
