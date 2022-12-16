use std::collections::HashMap;

use anyhow::Ok;
use util::{read_day10, Instruction, Instruction::*};

fn main() -> anyhow::Result<()> {
    let instructions = read_day10(Some("./input.txt".into()))?;
    println!("Part one: {}", solve_part_one(&instructions));
    println!("Part two:");
    solve_part_two(&instructions);
    Ok(())
}

fn solve_part_one(instructions: &Vec<Instruction>) -> i32 {
    let mut cycle_counter = 0;
    let mut changes: HashMap<usize, i32> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Noop => {
                cycle_counter += 1;
            }
            Addx(x) => {
                cycle_counter += 2;
                changes.insert(cycle_counter + 1, *x);
            }
        }
    }
    (20..)
        .step_by(40)
        .take_while(|cycle| *cycle <= cycle_counter)
        .fold(0, |sum, cycle| {
            sum + get_register_value_at_cycle(cycle, &changes) * cycle as i32
        })
}

fn get_register_value_at_cycle(cycle: usize, changes: &HashMap<usize, i32>) -> i32 {
    changes.iter().fold(1, |mut sum, (cycle_index, change)| {
        if *cycle_index <= cycle {
            sum += *change;
        }
        sum
    })
}

fn solve_part_two(instructions: &Vec<Instruction>) -> usize {
    let mut cycle_counter = 0;
    let mut changes: HashMap<usize, i32> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Noop => {
                cycle_counter += 1;
            }
            Addx(x) => {
                cycle_counter += 2;
                changes.insert(cycle_counter + 1, *x);
            }
        }
    }
    const COLUM_COUNT: usize = 6;
    const ROW_COUNT: usize = 40;
    for colum_index in 0..COLUM_COUNT {
        let mut line = "".to_string();
        for row_index in 0..ROW_COUNT {
            let cycle = colum_index * ROW_COUNT + row_index + 1;
            let sprite_center = get_register_value_at_cycle(cycle, &changes);
            let sprite_pixels: Vec<i32> = (sprite_center..sprite_center + 3).collect();
            if sprite_pixels
                .iter()
                .any(|position| *position as usize == row_index + 1)
            {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
    0
}
