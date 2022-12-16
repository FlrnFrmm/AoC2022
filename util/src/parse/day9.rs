use nom::{
    character::complete::{anychar, char, i32, newline},
    combinator::eof,
    multi::many_till,
    sequence::tuple,
    IResult,
};

pub fn parse(source: &str) -> IResult<&str, Vec<(char, i32)>> {
    let (tail, (values, _)) = many_till(pair_of_number_pairs, eof)(source)?;
    Ok((tail, values))
}

fn pair_of_number_pairs(source: &str) -> IResult<&str, (char, i32)> {
    let (tail, (direction, _, range, _)) = tuple((anychar, char(' '), i32, newline))(source)?;
    Ok((tail, (direction, range)))
}
