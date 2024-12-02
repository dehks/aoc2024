use std::error::Error;
use std::fmt::Display;

use winnow::error::ParserError;
use winnow::stream::{AsBStr, Stream, StreamIsPartial};
use winnow::Parser;

/// Adapt a winnow parser's error for use with cargo-aoc.
pub fn aoc_parse<I, O, E, P>(mut parser: P, input: I) -> Result<O, Box<dyn Error>>
where
    I: AsBStr,
    I: Stream,
    I: StreamIsPartial,
    E: ParserError<I>,
    E: Display,
    P: Parser<I, O, E>,
{
    parser.parse(input).map_err(|e| e.to_string().into())
}
