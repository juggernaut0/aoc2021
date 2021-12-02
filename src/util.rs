use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> impl Iterator<Item=T> + '_
    where T::Err: Debug
{
    input.lines()
        .map(|line| line.parse().unwrap())
}