use nom::{
    character::complete::{i32, newline},
    combinator::{eof, opt},
    multi::many_till,
    sequence::tuple,
    IResult,
};

pub fn parse(source: &str) -> IResult<&str, Vec<i32>> {
    let (tail, (values, _)) = many_till(number_and_spaces, eof)(source)?;
    Ok((tail, values.into_iter().map(|(i, _)| i).collect()))
}

fn number_and_spaces(source: &str) -> IResult<&str, (i32, Option<char>)> {
    tuple((i32, opt(newline)))(source)
}

macro_rules! one_number_per_line_tests {
    ($($label:ident: $input:expr, $expected_output:expr,)*) => {
    #[cfg(test)]
    mod tests {
        use super::*;
        $(
            #[test]
            fn $label() {
                match parse($input) {
                    Ok((tail, values)) => {
                        assert_eq!(tail, "");
                        assert_eq!(values, $expected_output);
                    },
                    Err(err) => panic!("Error: {}", err.to_string())
                }
            }
        )*
    }
    };
}

one_number_per_line_tests! {
    no_number:
        "",
        Vec::new(),
    one_number_without_newline_at_the_end:
        "1",
        vec![1],
    one_number_with_newline_at_the_end:
        "1\n",
        vec![1],
    multiple_numbers_without_newline_at_the_end:
        "1\n2\n3",
        vec![1, 2, 3],
    multiple_numbers_with_newline_at_the_end:
        "1\n2\n3\n",
        vec![1, 2, 3],
}
