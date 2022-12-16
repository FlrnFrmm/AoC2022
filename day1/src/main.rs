use anyhow::Ok;
use util::read_day1;

fn main() -> anyhow::Result<()> {
    let mut values = read_day1(Some("./input.txt".into()))?
        .into_iter()
        .map(|values| values.into_iter().sum::<i32>())
        .collect::<Vec<i32>>();

    let result = values.iter().max();
    println!("{}", result.unwrap_or(&0));

    values.sort();
    let top_three: i32 = values.iter().rev().take(3).sum();
    println!("{}", top_three);

    Ok(())
}
