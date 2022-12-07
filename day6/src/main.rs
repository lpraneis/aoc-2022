use std::collections::HashSet;

fn solve(input: &'static str, bytes: usize) -> usize {
    input
        .as_bytes()
        .windows(bytes)
        .position(|x| x.iter().collect::<HashSet<_>>().len() == bytes)
        .map(|x| x + bytes)
        .unwrap()
}
fn part1(input: &'static str) -> usize {
    solve(input, 4)
}

fn part2(input: &'static str) -> usize {
    solve(input, 14)
}

fn main() {
    let input = include_str!("../actual.input");

    let part1 = part1(input);
    println!("Part 1: {part1}");

    let part2 = part2(input);
    println!("Part 2: {part2}");
}
