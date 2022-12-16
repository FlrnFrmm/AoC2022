use nom::{
    character::complete::{char, i32, newline},
    combinator::eof,
    multi::many_till,
    sequence::tuple,
    IResult,
};

pub fn parse(source: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    let (tail, (values, _)) = many_till(pair_of_number_pairs, eof)(source)?;
    Ok((tail, values))
}

fn pair_of_number_pairs(source: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (tail, (i, _, j, _, k, _, l, _)) =
        tuple((i32, char('-'), i32, char(','), i32, char('-'), i32, newline))(source)?;
    Ok((tail, ((i, j), (k, l))))
}
