use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{complete, is_alphanumeric},
    combinator::{map, value},
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::dirtree::{create_directory, ChangeDirectory, Command, Directory, Line, Output};

pub(crate) fn parse_input(input: &str) -> Directory {
    create_directory(
        input
            .lines()
            .map(|x| parse_line(x).unwrap())
            .map(|x| x.1)
            .collect(),
    )
}

fn is_path_character(chr: char) -> bool {
    is_alphanumeric(chr as u8) | "./".contains(chr)
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    preceded(
        tag("$ "),
        alt((map(parse_ls, |_| Command::Ls), map(parse_cd, Command::Cd))),
    )(i)
}

fn parse_cd(i: &str) -> IResult<&str, ChangeDirectory> {
    preceded(tag("cd "), map(take_while1(is_path_character), Into::into))(i)
}
fn parse_ls(i: &str) -> IResult<&str, ()> {
    value((), tag("ls"))(i)
}

fn parse_output(i: &str) -> IResult<&str, Output> {
    alt((
        map(parse_dir_output, Output::Dir),
        map(parse_file_output, |(size, name)| Output::File(size, name)),
    ))(i)
}

fn parse_dir_output(i: &str) -> IResult<&str, String> {
    preceded(
        tag("dir "),
        map(take_while1(is_path_character), |x: &str| x.to_owned()),
    )(i)
}
fn parse_file_output(i: &str) -> IResult<&str, (u64, String)> {
    map(
        separated_pair(complete::u64, tag(" "), take_while1(is_path_character)),
        |(size, path): (u64, &str)| (size, path.to_owned()),
    )(i)
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_output, Line::Output),
        map(parse_command, Line::Command),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_ls() {
        let line = "$ ls";
        let (extra, c) = parse_command(line).unwrap();
        assert!(matches!(c, Command::Ls));
        assert!(extra.is_empty());
    }

    #[test]
    fn test_parses_cd() {
        let line = "$ cd /";
        let (extra, c) = parse_command(line).unwrap();
        assert_eq!(c, Command::Cd(ChangeDirectory::from("/")));
        assert!(extra.is_empty());
    }

    #[test]
    fn test_parses_dir() {
        let line = "dir asdf";
        let (extra, c) = parse_output(line).unwrap();
        assert!(extra.is_empty());
        assert_eq!(c, Output::Dir("asdf".to_owned()));
    }
    #[test]
    fn test_parses_file() {
        let line = "123 a";
        let (extra, c) = parse_output(line).unwrap();
        assert!(extra.is_empty());
        assert_eq!(c, Output::File(123, "a".to_owned()));

        let line = "123 a";
        let (extra, c) = parse_line(line).unwrap();
        assert!(extra.is_empty());
        assert_eq!(c, Line::Output(Output::File(123, "a".to_owned())));
    }
}


