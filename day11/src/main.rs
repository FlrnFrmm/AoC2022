use std::collections::{HashMap, VecDeque};

use anyhow::Ok;
use util::{read_day11, Monkey, Operation};

fn main() -> anyhow::Result<()> {
    let mut monkeys = read_day11(Some("./input_test.txt".into()))?;
    solve_part_one(&mut monkeys);
    // solve_part_two(&mut monkeys);
    Ok(())
}

fn solve_part_one(monkeys: &mut Vec<Monkey>) {
    let mut monkey_scores: HashMap<usize, usize> = HashMap::new();
    for _ in 1..=20 {
        for (monkey_id, monkey_score) in run_a_round(monkeys) {
            let score = monkey_scores.entry(monkey_id).or_insert(0);
            *score += monkey_score;
        }
    }
    let mut scores: Vec<usize> = monkey_scores.values().into_iter().map(|v| *v).collect();
    scores.sort();
    let result = scores.into_iter().rev().take(2).product::<usize>();
    println!("Part one: {}", result);
}

fn solve_part_two(monkeys: &Vec<Monkey>) {
    // TbD
}

fn run_a_round(monkeys: &mut Vec<Monkey>) -> HashMap<usize, usize> {
    let mut monkey_scores: HashMap<usize, usize> = HashMap::new();
    for monkey_id in 0..monkeys.len() {
        while !monkeys[monkey_id].items.is_empty() {
            let item = monkeys[monkey_id].items.pop_front().unwrap();
            let new_value = match monkeys[monkey_id].modifier {
                Operation::Add(value) => item + value,
                Operation::Multiply(value) => item * value,
                Operation::MultiplySelf => item * item,
            };
            let relieved_value = new_value / 3;
            let to = if relieved_value % monkeys[monkey_id].test == 0 {
                monkeys[monkey_id].test_true_to
            } else {
                monkeys[monkey_id].test_false_to
            };
            let monkey_score = monkey_scores.entry(monkey_id).or_insert(0);
            *monkey_score += 1;
            monkeys[to].items.push_back(relieved_value);
        }
    }
    monkey_scores
}
