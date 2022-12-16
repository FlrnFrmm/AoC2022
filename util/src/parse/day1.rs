use nom::{
    character::complete::{i32, newline},
    combinator::eof,
    multi::many_till,
    sequence::tuple,
    IResult,
};

pub fn parse(source: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (tail, (values, _)) = many_till(numbers_and_newlines, eof)(source)?;
    Ok((tail, values))
}

fn numbers_and_newlines(source: &str) -> IResult<&str, Vec<i32>> {
    let (tail, (values, _)) = many_till(tuple((i32, newline)), newline)(source)?;
    Ok((tail, values.into_iter().map(|(i, _)| i).collect()))
}

macro_rules! day1_tests {
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

day1_tests! {
  one_group_with_one_number:
      "1\n\n",
      vec![vec![1]],
  one_group_with_three_number:
      "1\n2\n3\n\n",
      vec![vec![1,2,3]],
  two_groups_with_one_number:
      "1\n\n2\n\n",
      vec![vec![1], vec![2]],
  two_groups_with_three_number:
      "11\n12\n13\n\n21\n22\n23\n\n",
      vec![vec![11, 12, 13], vec![21, 22, 23]],
  test_input:
      "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n",
      vec![
        vec![1000, 2000, 3000],
        vec![4000],
        vec![5000, 6000],
        vec![7000, 8000, 9000],
        vec![10000],
      ],
}
