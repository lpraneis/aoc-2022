#![allow(unused)]

use itertools::Itertools;
use std::collections::HashSet;

fn get_letter_score(c: char) -> u32 {
    let offset = if c.is_uppercase() { 26 } else { 0 };
    let ord = c.to_ascii_lowercase() as u32 - 96;
    ord + offset
}

fn get_common_item((left, right): (&str, &str)) -> char {
    let left: HashSet<char> = HashSet::from_iter(left.chars());
    let right: HashSet<char> = HashSet::from_iter(right.chars());
    *left.intersection(&right).next().unwrap()
}

fn get_badges((first, second, third): (&str, &str, &str)) -> char {
    let first = first.chars().collect::<HashSet<char>>();
    let second = second.chars().collect::<HashSet<char>>();
    let third = third.chars().collect::<HashSet<char>>();
    let first_two: HashSet<char> = HashSet::from_iter(first.intersection(&second).copied());
    *first_two.intersection(&third).next().unwrap()
}

fn part1(input: &'static str) -> u32 {
    input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(get_common_item)
        .map(get_letter_score)
        .sum()
}

fn part2(input: &'static str) -> u32 {
    input
        .lines()
        .into_iter()
        .tuples()
        .map(get_badges)
        .map(get_letter_score)
        .sum()
}

fn main() {
    let input = include_str!("../actual.input");

    let part1 = part1(input);
    println!("Part 1: {part1}");

    let part2 = part2(input);
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn helpers() {
        assert_eq!(get_letter_score('a'), 1);
        assert_eq!(get_letter_score('z'), 26);
        assert_eq!(get_letter_score('A'), 27);
        assert_eq!(get_letter_score('Z'), 52);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../test.input");

        let part1 = part1(input);
        assert_eq!(part1, 157);

        let part2 = part2(input);
        assert_eq!(part2, 70);
    }
}
