use anyhow;
use util::read_lines;

#[derive(Debug)]
enum Token {
    Rock,
    Paper,
    Scissor,
}

fn main() -> anyhow::Result<()> {
    let values = read_lines(Some("./input.txt".into()))?
        .into_iter()
        .map(from_line_to_tokens)
        .collect::<anyhow::Result<Vec<(Token, Token)>>>()?;

    println!(
        "Score part one: {:?}",
        values.iter().map(calculate_score_part_one).sum::<i32>()
    );

    println!(
        "Score part two: {:?}",
        values.iter().map(calculate_score_part_two).sum::<i32>()
    );

    Ok(())
}

fn from_line_to_tokens(line: String) -> anyhow::Result<(Token, Token)> {
    use Token::*;
    match line.as_str() {
        "A X" => Ok((Rock, Rock)),
        "A Y" => Ok((Rock, Paper)),
        "A Z" => Ok((Rock, Scissor)),
        "B X" => Ok((Paper, Rock)),
        "B Y" => Ok((Paper, Paper)),
        "B Z" => Ok((Paper, Scissor)),
        "C X" => Ok((Scissor, Rock)),
        "C Y" => Ok((Scissor, Paper)),
        "C Z" => Ok((Scissor, Scissor)),
        _ => anyhow::bail!("Unrecognizable string"),
    }
}

fn calculate_score_part_one(tokens: &(Token, Token)) -> i32 {
    use Token::*;
    match tokens {
        (Rock, Rock) => 3 + 1,
        (Rock, Paper) => 6 + 2,
        (Rock, Scissor) => 0 + 3,
        (Paper, Rock) => 0 + 1,
        (Paper, Paper) => 3 + 2,
        (Paper, Scissor) => 6 + 3,
        (Scissor, Rock) => 6 + 1,
        (Scissor, Paper) => 0 + 2,
        (Scissor, Scissor) => 3 + 3,
    }
}

fn calculate_score_part_two(tokens: &(Token, Token)) -> i32 {
    use Token::*;
    match tokens {
        (Rock, Rock) => 0 + 3,       // lose -> Scissor
        (Rock, Paper) => 3 + 1,      // draw -> Rock
        (Rock, Scissor) => 6 + 2,    // win -> Paper
        (Paper, Rock) => 0 + 1,      // lose -> Rock
        (Paper, Paper) => 3 + 2,     // draw -> Paper
        (Paper, Scissor) => 6 + 3,   // win -> Scissor
        (Scissor, Rock) => 0 + 2,    // lose -> Paper
        (Scissor, Paper) => 3 + 3,   // draw -> Scissor
        (Scissor, Scissor) => 6 + 1, // win -> Rock
    }
}
