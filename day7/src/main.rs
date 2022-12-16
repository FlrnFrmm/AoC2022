use std::collections::HashMap;

use anyhow::Ok;
use util::read_lines;

fn main() -> anyhow::Result<()> {
    let commands = read_lines(Some("./input.txt".into()))?;
    let mut directories: HashMap<String, usize> = HashMap::new();
    read_system(commands, &mut directories);
    solve_part_one(&directories);
    solve_part_two(&mut directories);
    Ok(())
}

fn read_system(commands: Vec<String>, directories: &mut HashMap<String, usize>) {
    let mut pwd: Vec<String> = Vec::new();
    commands.iter().for_each(|command| {
        match command.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["$", "cd", dir] => match dir {
                &"/" => {
                    pwd.clear();
                    pwd.push("/".into())
                }
                &".." => {
                    pwd.pop();
                }
                name => {
                    pwd.push(format!("{}/", name));
                }
            },
            ["$", "ls"] => {}
            ["dir", dir] => {}
            [filesize, _filename] => {
                let size = filesize.parse::<usize>().unwrap();
                let mut tmp_pwd = pwd.clone();
                while !tmp_pwd.is_empty() {
                    let entry = directories.entry(tmp_pwd.join("")).or_insert(0);
                    *entry += size;
                    tmp_pwd.pop();
                }
            }
            _ => println!("Unknown command"),
        }
    })
}

fn solve_part_one(directories: &HashMap<String, usize>) {
    let part_one = directories
        .values()
        .into_iter()
        .filter(|size| **size <= 100000)
        .sum::<usize>();
    println!("{:?}", part_one);
}

fn solve_part_two(directories: &mut HashMap<String, usize>) {
    let free_space = 70000000 - *directories.entry("/".into()).or_insert(0);
    let part_two = directories
        .values()
        .into_iter()
        .filter(|size| **size + free_space >= 30000000)
        .min();
    println!("{:?}", part_two.unwrap_or(&0));
}
