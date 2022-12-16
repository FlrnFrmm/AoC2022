use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i128, i32, newline},
    combinator::{eof, opt},
    multi::many1,
    multi::many_till,
    sequence::tuple,
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Operation {
    Add(i128),
    Multiply(i128),
    MultiplySelf,
}

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: VecDeque<i128>,
    pub modifier: Operation,
    pub test: i128,
    pub test_true_to: usize,
    pub test_false_to: usize,
}

pub fn parse(source: &str) -> IResult<&str, Vec<Monkey>> {
    let (tail, (values, _)) = many_till(monkey, eof)(source)?;
    Ok((tail, values))
}

fn monkey(source: &str) -> IResult<&str, Monkey> {
    let (tail, (id, items, modifier, test, test_true_to, test_false_to, _)) = tuple((
        id,
        items,
        modifier,
        test,
        test_true_to,
        test_false_to,
        opt(newline),
    ))(source)?;
    let monkey = Monkey {
        id,
        items,
        modifier,
        test,
        test_true_to,
        test_false_to,
    };
    Ok((tail, monkey))
}

fn id(source: &str) -> IResult<&str, usize> {
    let (tail, (_, id, _, _)) = tuple((tag("Monkey "), i32, char(':'), newline))(source)?;
    Ok((tail, id as usize))
}

fn items(source: &str) -> IResult<&str, VecDeque<i128>> {
    let (tail, (_, values, _)) = tuple((tag("  Starting items: "), list_of_i32, newline))(source)?;
    Ok((tail, values.into()))
}

pub fn list_of_i32(source: &str) -> IResult<&str, Vec<i128>> {
    let (tail, values) = many1(tuple((i128, opt(tag(", ")))))(source)?;
    Ok((tail, values.iter().map(|item| item.0).collect()))
}

fn modifier(source: &str) -> IResult<&str, Operation> {
    let (tail, (_, operation, _)) = tuple((
        tag("  Operation: new = "),
        alt((number_operation, self_operation)),
        newline,
    ))(source)?;
    Ok((tail, operation))
}

fn number_operation(source: &str) -> IResult<&str, Operation> {
    let (tail, (_, ops_tag, value)) =
        tuple((tag("old"), alt((tag(" + "), tag(" * "))), i128))(source)?;
    let operation = match ops_tag {
        " + " => Operation::Add(value),
        " * " => Operation::Multiply(value),
        _ => Operation::Add(0),
    };
    Ok((tail, operation))
}

fn self_operation(source: &str) -> IResult<&str, Operation> {
    let (tail, _) = tag("old * old")(source)?;
    Ok((tail, Operation::MultiplySelf))
}

fn test(source: &str) -> IResult<&str, i128> {
    let (tail, (_, id, _)) = tuple((tag("  Test: divisible by "), i128, newline))(source)?;
    Ok((tail, id))
}

fn test_true_to(source: &str) -> IResult<&str, usize> {
    let (tail, (_, id, _)) = tuple((tag("    If true: throw to monkey "), i32, newline))(source)?;
    Ok((tail, id as usize))
}

fn test_false_to(source: &str) -> IResult<&str, usize> {
    let (tail, (_, id, _)) = tuple((tag("    If false: throw to monkey "), i32, newline))(source)?;
    Ok((tail, id as usize))
}
