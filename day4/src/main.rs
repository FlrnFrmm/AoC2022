use anyhow::Ok;
use util::read_day4;

fn main() -> anyhow::Result<()> {
    let pairs = read_day4(Some("./input.txt"))?;

    let sum_part_one = pairs
        .iter()
        .filter(|&&((first_start, first_end), (second_start, second_end))| {
            let first_in_second = first_start >= second_start && first_end <= second_end;
            let second_in_first = second_start >= first_start && second_end <= first_end;
            first_in_second || second_in_first
        })
        .count();
    println!("{:?}", sum_part_one);

    let sum_part_two = pairs
        .iter()
        .filter(|&&((first_start, first_end), (second_start, second_end))| {
            (first_start..=first_end)
                .into_iter()
                .any(|v| (second_start..=second_end).into_iter().contains(&v))
        })
        .count();
    println!("{:?}", sum_part_two);

    Ok(())
}
