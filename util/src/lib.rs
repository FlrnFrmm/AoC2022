mod parse;

use anyhow::Result;
pub use parse::{Instruction, Monkey, Operation};
use std::fs::read_to_string;

pub fn read_string(path_to_file: Option<&str>) -> Result<String> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let content = read_to_string(file_path)?;
    Ok(content.chars().filter(|c| !c.is_whitespace()).collect())
}

pub fn read_lines(path_to_file: Option<&str>) -> Result<Vec<String>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    Ok(contents
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect())
}

pub fn read_day1(path_to_file: Option<&str>) -> Result<Vec<Vec<i32>>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day1(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

pub fn read_day4(path_to_file: Option<&str>) -> Result<Vec<((i32, i32), (i32, i32))>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day4(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

pub fn read_day5(path_to_file: Option<&str>) -> Result<Vec<(i32, i32, i32)>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day5(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

pub fn read_day8(path_to_file: Option<&str>) -> Result<Vec<Vec<i32>>> {
    let content = read_lines(path_to_file)?;
    let values = content
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string()).parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    Ok(values)
}

pub fn read_day9(path_to_file: Option<&str>) -> Result<Vec<(char, i32)>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day9(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

pub fn read_day10(path_to_file: Option<&str>) -> Result<Vec<Instruction>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day10(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

pub fn read_day11(path_to_file: Option<&str>) -> Result<Vec<Monkey>> {
    let file_path = path_to_file.unwrap_or("./input.txt");
    let contents = read_to_string(file_path)?;
    match parse::day11(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}

macro_rules! read_day1_tests {
  ($($label:ident: $input:expr, $expected_output:expr,)*) => {
  #[cfg(test)]
  mod tests {
      use super::*;
      $(
          #[test]
          fn $label() {
              match read_day1($input) {
                  Ok(values) => {
                      assert_eq!(values, $expected_output);
                  },
                  Err(err) => panic!("Error: {}", err.to_string())
              }
          }
      )*
  }
  };
}

read_day1_tests! {
  read_day1_file:
    Some("./examples/input_day1.txt"),
    vec![
      vec![1000, 2000, 3000],
      vec![4000],
      vec![5000, 6000],
      vec![7000, 8000, 9000],
      vec![10000],
    ],
}
