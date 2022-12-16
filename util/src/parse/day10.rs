use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i32, newline},
    combinator::eof,
    multi::many_till,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

pub fn parse(source: &str) -> IResult<&str, Vec<Instruction>> {
    let (tail, (values, _)) = many_till(instructions, eof)(source)?;
    Ok((tail, values))
}

fn instructions(source: &str) -> IResult<&str, Instruction> {
    let (tail, instruction) = alt((noop, addx))(source)?;
    Ok((tail, instruction))
}

fn noop(source: &str) -> IResult<&str, Instruction> {
    let (tail, _) = tuple((tag("noop"), newline))(source)?;
    Ok((tail, Instruction::Noop))
}
fn addx(source: &str) -> IResult<&str, Instruction> {
    let (tail, (_, _, value, _)) = tuple((tag("addx"), char(' '), i32, newline))(source)?;
    Ok((tail, Instruction::Addx(value)))
}
