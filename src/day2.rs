use std::collections::HashSet;
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

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum State {
    Start,
    Pos(usize, i32, bool),
    End,
}

// The approach here is to treat the report as a NFA (non-deterministic finite automaton).
// This models the possible paths through the report including valid direct transitions
// and valid transitions that skip one. We maintain a list of states to be explored, starting
// with the Start state. At each iteration step we replace this list with the list of valid
// successor states. We stop when we run out of more states to explore (failure) or reach
// the End state (success). Knowledge of the current direction (if present), and whether we have
// already used our one skip, is encoded into the states.
fn report_safety(report: &[u32], skip_enabled: bool) -> Safety {
    let mut states = vec![State::Start];
    let mut tmp_states: HashSet<State> = HashSet::new();

    while !states.is_empty() {
        for state in states.drain(..) {
            match state {
                State::Start => {
                    // We can always start with the first element, or skip to the second.
                    if !report.is_empty() {
                        tmp_states.insert(State::Pos(0, 0, false));
                    }
                    if report.len() > 1 && skip_enabled {
                        tmp_states.insert(State::Pos(1, 0, true));
                    }
                }
                State::Pos(i, sign, skipped) => {
                    let assess_move = |to| {
                        let delta = report[to] as i32 - report[i] as i32;
                        let delta_sign = delta.signum();
                        (
                            (delta_sign + sign) != 0 && (1..=3).contains(&delta.abs()),
                            delta_sign,
                        )
                    };

                    if i == report.len() - 1 {
                        // On the last element the only thing we can do is end.
                        tmp_states.insert(State::End);
                    } else {
                        // Otherwise we might be able to move to the next element.
                        // But only if the direction and magnitude of the movement is valid.
                        let (valid, new_sign) = assess_move(i + 1);
                        if valid {
                            tmp_states.insert(State::Pos(i + 1, new_sign, skipped));
                        }

                        // If we haven't already skipped, we might be able to skip now.
                        if !skipped && skip_enabled {
                            if i == report.len() - 2 {
                                tmp_states.insert(State::End);
                            } else {
                                let (valid, new_sign) = assess_move(i + 2);
                                if valid {
                                    tmp_states.insert(State::Pos(i + 2, new_sign, true));
                                }
                            }
                        }
                    }
                }
                State::End => {
                    // If we reached an end state we found a valid path and can return.
                    return Safety::Safe;
                }
            }
        }
        states.extend(tmp_states.drain());
    }

    // If we got here we never managed to reach the end state, so the report was not safe.
    Safety::Unsafe
}

#[aoc(day2, part1)]
pub fn part1(pairs: &[Vec<u32>]) -> usize {
    pairs
        .iter()
        .map(|r| report_safety(r, false))
        .filter(|s| *s == Safety::Safe)
        .count()
}

#[aoc(day2, part2)]
pub fn part2(pairs: &[Vec<u32>]) -> usize {
    pairs
        .iter()
        .map(|r| report_safety(r, true))
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
    fn test_report_safety_without_skipping() {
        assert_eq!(report_safety(&[7, 6, 4, 2, 1], false), Safety::Safe);
        assert_eq!(report_safety(&[1, 2, 7, 8, 9], false), Safety::Unsafe);
        assert_eq!(report_safety(&[9, 7, 6, 2, 1], false), Safety::Unsafe);
        assert_eq!(report_safety(&[1, 3, 2, 4, 5], false), Safety::Unsafe);
        assert_eq!(report_safety(&[8, 6, 4, 4, 1], false), Safety::Unsafe);
        assert_eq!(report_safety(&[1, 3, 6, 7, 9], false), Safety::Safe);
    }

    #[test]
    fn test_report_safety_with_skipping() {
        assert_eq!(report_safety(&[7, 6, 4, 2, 1], true), Safety::Safe);
        assert_eq!(report_safety(&[1, 2, 7, 8, 9], true), Safety::Unsafe);
        assert_eq!(report_safety(&[9, 7, 6, 2, 1], true), Safety::Unsafe);
        assert_eq!(report_safety(&[1, 3, 2, 4, 5], true), Safety::Safe);
        assert_eq!(report_safety(&[8, 6, 4, 4, 1], true), Safety::Safe);
        assert_eq!(report_safety(&[1, 3, 6, 7, 9], true), Safety::Safe);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&example()), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&example()), 4);
    }
}
