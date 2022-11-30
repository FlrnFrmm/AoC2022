mod parse;

use anyhow::Result;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn read_one_number_per_line(path_to_file: Option<PathBuf>) -> Result<Vec<i32>> {
    let file_path = path_to_file.unwrap_or("./input.txt".into());
    let contents = read_to_string(file_path)?;
    match parse::one_number_per_line(&contents) {
        Ok((_, values)) => Ok(values),
        Err(err) => anyhow::bail!(err.to_string()),
    }
}
