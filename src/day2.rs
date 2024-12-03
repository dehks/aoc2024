use std::error::Error;

use aoc_runner_derive::{aoc, aoc_generator};

use winnow::ascii::{dec_uint, newline, space1};
use winnow::combinator::separated;
use winnow::error::ContextError;
use winnow::{PResult, Parser};

use crate::parse::aoc_parse;

fn report(input: &mut &str) -> PResult<Vec<u32>> {
    let num = dec_uint::<_, u32, ContextError>;
    separated(1.., num, space1).parse_next(input)
}

fn reports(input: &mut &str) -> PResult<Vec<Vec<u32>>> {
    separated(1.., report, newline).parse_next(input)
}

#[aoc_generator(day2)]
pub fn parse(input: &'_ str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    aoc_parse(reports, input)
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    Unsafe,
}

fn safety(report: &[u32]) -> Safety {
    let mut current_sign = None;
    for pair in report.windows(2) {
        let diff = pair[1] as i32 - pair[0] as i32;
        match (current_sign, diff.signum()) {
            (None, sign) => {
                current_sign.replace(sign);
            }
            (Some(cs), sign) if cs != sign => {
                return Safety::Unsafe;
            }
            _ => {}
        }
        if !(1..=3).contains(&diff.abs()) {
            return Safety::Unsafe;
        }
    }
    Safety::Safe
}

#[aoc(day2, part1)]
pub fn part1(pairs: &[Vec<u32>]) -> usize {
    pairs
        .iter()
        .map(|r| safety(r))
        .filter(|s| *s == Safety::Safe)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn example() -> Vec<Vec<u32>> {
        vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]
    }

    #[test]
    fn test_parse() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "}
        .trim();
        assert_eq!(parse(input).unwrap(), example());
    }

    #[test]
    fn test_safety() {
        assert_eq!(safety(&[7, 6, 4, 2, 1]), Safety::Safe);
        assert_eq!(safety(&[1, 2, 7, 8, 9]), Safety::Unsafe);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&example()), 2);
    }
}
