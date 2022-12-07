use crate::{Crate, CrateStack, Move};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{self, anychar, line_ending},
    combinator::{map, value},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

fn parse_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), complete::u32),
            preceded(tag(" from "), complete::u32),
            preceded(tag(" to "), complete::u32),
        )),
        |(amt, from, to)| Move {
            from: from - 1,
            to: to - 1,
            amt,
        },
    )(input)
}

fn all_moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, parse_move)(input)
}

fn parse_crate(input: &str) -> IResult<&str, Option<Crate>> {
    alt((
        map(parse_crate_number, Some),
        map(parse_blank_spot, |_| None),
    ))(input)
}

fn parse_crate_number(input: &str) -> IResult<&str, Crate> {
    map(delimited(tag("["), anychar, tag("]")), Crate)(input)
}

fn parse_blank_spot(input: &str) -> IResult<&str, ()> {
    value((), tag("   "))(input)
}

fn all_crate_lines(input: &str) -> IResult<&str, CrateStack> {
    terminated(
        map(
            separated_list1(
                line_ending,
                separated_list1(complete::char(' '), parse_crate),
            ),
            transform_rows_to_stack,
        ),
        complete::char('\n'),
    )(input)
}

fn ignored_line(input: &str) -> IResult<&str, ()> {
    value((), terminated(take_until("\n"), line_ending))(input)
}

fn transform_rows_to_stack(v: Vec<Vec<Option<Crate>>>) -> CrateStack {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    CrateStack(
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .rev()
                    .filter_map(|n| n.next().unwrap())
                    .collect::<Vec<Crate>>()
            })
            .collect(),
    )
}

pub(crate) fn parse(input: &str) -> (CrateStack, Vec<Move>) {
    map(
        tuple((all_crate_lines, ignored_line, ignored_line, all_moves)),
        |(lines, _, _, moves)| (lines, moves),
    )(input)
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn move_parsing() {
        let (_, m) = parse_move("move 3 from 8 to 6").unwrap();
        assert_eq!(
            m,
            Move {
                from: 7,
                to: 5,
                amt: 3
            }
        )
    }

    #[test]
    fn full_parse() {
        let input = include_str!("../test.input");
        let (crates, moves) = parse(input);
        assert_eq!(
            moves,
            vec![
                Move {
                    from: 1,
                    to: 0,
                    amt: 1
                },
                Move {
                    from: 0,
                    to: 2,
                    amt: 3
                },
                Move {
                    from: 1,
                    to: 0,
                    amt: 2
                },
                Move {
                    from: 0,
                    to: 1,
                    amt: 1
                },
            ]
        );
        assert_eq!(
            crates,
            CrateStack(vec![
                vec![Crate('Z'), Crate('N')],
                vec![Crate('M'), Crate('C'), Crate('D')],
                vec![Crate('P')],
            ])
        );
    }
}
