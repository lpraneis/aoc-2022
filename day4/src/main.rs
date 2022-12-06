#![allow(unused)]

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
struct CleaningRange {
    start: u32,
    end: u32,
}
impl From<&str> for CleaningRange {
    fn from(a: &str) -> Self {
        let (start, end) = a.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

impl CleaningRange {
    fn within(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
    fn overlaps(&self, other: &Self) -> bool {
        (self.end >= other.start && self.end <= other.end)
            || (self.start >= other.start && self.start <= other.end)
    }
}

#[derive(Clone, Copy, Debug)]
struct CleaningAssignment {
    first: CleaningRange,
    second: CleaningRange,
}
impl CleaningAssignment {
    fn fully_contains(&self) -> bool {
        let CleaningAssignment { first, second } = self;
        first.within(second) || second.within(first)
    }
    fn overlaps(&self) -> bool {
        let CleaningAssignment { first, second } = self;
        first.overlaps(second) || second.overlaps(first)
    }
}
impl From<(&str, &str)> for CleaningAssignment {
    fn from((a, b): (&str, &str)) -> Self {
        Self {
            first: a.into(),
            second: b.into(),
        }
    }
}

fn part1(input: &'static str) -> u32 {
    input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(CleaningAssignment::from)
        .filter_map(|x| x.fully_contains().then_some(0))
        .count()
        .try_into()
        .unwrap()
}

fn part2(input: &'static str) -> u32 {
    input
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(CleaningAssignment::from)
        .filter_map(|x| x.overlaps().then_some(0))
        .count()
        .try_into()
        .unwrap()
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
    fn test_input() {
        let input = include_str!("../test.input");

        let part1 = part1(input);
        assert_eq!(part1, 2);

        let part2 = part2(input);
        assert_eq!(part2, 4);
    }
}
