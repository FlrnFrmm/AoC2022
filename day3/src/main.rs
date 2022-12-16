use util::read_lines;

fn main() -> anyhow::Result<()> {
    let rucksacks = read_lines(Some("./input.txt"))?;

    let sum_one = rucksacks
        .iter()
        .map(|content| {
            for item in content[..content.len() / 2].chars() {
                if content[content.len() / 2..].contains(item) {
                    return evaluate_value(item);
                }
            }
            0
        })
        .sum::<i32>();
    println!("Part one: {}", sum_one);

    let sum_two = rucksacks
        .as_slice()
        .chunks(3)
        .map(|group| match group {
            [one, two, three] => one
                .chars()
                .find(|item| two.contains(*item) && three.contains(*item))
                .map(|item| evaluate_value(item))
                .unwrap_or(0),
            _ => 0,
        })
        .sum::<i32>();
    println!("Part two: {}", sum_two);

    Ok(())
}

fn evaluate_value(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
