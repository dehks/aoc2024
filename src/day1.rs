use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};

use winnow::ascii::{dec_uint, newline, space1};
use winnow::combinator::{separated, seq};
use winnow::error::ContextError;
use winnow::{PResult, Parser};

use crate::parse::aoc_parse;

fn pair(input: &mut &str) -> PResult<(u32, u32)> {
    let num = dec_uint::<_, u32, ContextError>;
    seq!(num, _: space1, num).parse_next(input)
}

fn list(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    separated(1.., pair, newline).parse_next(input)
}

#[aoc_generator(day1)]
pub fn parse(input: &'_ str) -> Result<Vec<(u32, u32)>, Box<dyn Error>> {
    aoc_parse(list, input)
}

#[aoc(day1, part1)]
pub fn part1(pairs: &[(u32, u32)]) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "}
        .trim();
        let parsed = parse(input).unwrap();
        assert_eq!(parsed, vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)]);
    }

    #[test]
    fn test_part1() {
        let pairs = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        assert_eq!(part1(&pairs), 11);
    }
}
