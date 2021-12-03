use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_lines<'a, T: FromStr + 'a>(input: &'a str) -> impl Iterator<Item=T> + 'a
    where T::Err: Debug
{
    parse_lines_with(input, |line| line.parse().unwrap())
}

pub fn parse_lines_with<'a, T: 'a, P: FnMut(&'a str) -> T + 'a>(input: &'a str, parser: P) -> impl Iterator<Item=T> + 'a {
    input.lines()
        .map(parser)
}
