use std::collections::{HashMap, VecDeque};

use anyhow::Ok;
use util::read_day5;

fn main() -> anyhow::Result<()> {
    let values = read_day5(Some("./input.txt".into()))?;
    part_one(&values);
    part_two(&values);
    Ok(())
}

fn part_one(values: &Vec<(i32, i32, i32)>) {
    let mut stacks = create_stacks();
    for &(count, from, to) in values.iter() {
        let items = pop_n_front(&mut stacks, from, count);
        let to_stack = stacks.get_mut(&to).unwrap();
        for item in items.chars() {
            to_stack.push_front(item);
        }
    }
    println!("{:?}", stacks);
}

fn part_two(values: &Vec<(i32, i32, i32)>) {
    let mut stacks = create_stacks();
    for &(count, from, to) in values.iter() {
        let items = pop_n_front(&mut stacks, from, count);
        let to_stack = stacks.get_mut(&to).unwrap();
        for item in items.chars().rev() {
            to_stack.push_front(item);
        }
    }
    println!("{:?}", stacks);
}

fn pop_n_front(stacks: &mut HashMap<i32, VecDeque<char>>, from: i32, count: i32) -> String {
    let mut items = String::from("");
    let from_stack = stacks.get_mut(&from).unwrap();
    for _ in (0..count) {
        let item = from_stack.pop_front().unwrap();
        items.push(item);
    }
    items
}

//         [Q] [B]         [H]
//     [F] [W] [D] [Q]     [S]
//     [D] [C] [N] [S] [G] [F]
//     [R] [D] [L] [C] [N] [Q]     [R]
// [V] [W] [L] [M] [P] [S] [M]     [M]
// [J] [B] [F] [P] [B] [B] [P] [F] [F]
// [B] [V] [G] [J] [N] [D] [B] [L] [V]
// [D] [P] [R] [W] [H] [R] [Z] [W] [S]
//  1   2   3   4   5   6   7   8   9

fn create_stacks() -> HashMap<i32, VecDeque<char>> {
    let mut stacks = HashMap::new();
    stacks.insert(1, stack_from_string("VJBD"));
    stacks.insert(2, stack_from_string("FDRWBVP"));
    stacks.insert(3, stack_from_string("QWCDLFGR"));
    stacks.insert(4, stack_from_string("BDNLMPJW"));
    stacks.insert(5, stack_from_string("QSCPBNH"));
    stacks.insert(6, stack_from_string("GNSBDR"));
    stacks.insert(7, stack_from_string("HSFQMPBZ"));
    stacks.insert(8, stack_from_string("FLW"));
    stacks.insert(9, stack_from_string("RMFVS"));
    stacks
}

fn stack_from_string(stack_as_string: &str) -> VecDeque<char> {
    let mut stack = VecDeque::with_capacity(stack_as_string.len());
    for c in stack_as_string.chars() {
        stack.push_back(c);
    }
    stack
}
