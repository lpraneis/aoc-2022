#![allow(unused)]

fn play_score(a: &str) -> u32 {
    match a {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    }
}

fn win_score(op: &str, me: &str) -> u32 {
    match (op, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        ("X", "A") => 3,
        ("X", "B") => 6,
        ("X", "C") => 0,
        ("Y", "A") => 0,
        ("Y", "B") => 3,
        ("Y", "C") => 6,
        ("Z", "A") => 6,
        ("Z", "B") => 0,
        ("Z", "C") => 3,
        _ => unreachable!(),
    }
}

fn choose_win_or_loss(op: &'static str, outcome: &'static str) -> &'static str {
    match (op, outcome) {
        ("A", "X") => "Z",
        ("A", "Y") => "X",
        ("A", "Z") => "Y",
        ("B", "X") => "X",
        ("B", "Y") => "Y",
        ("B", "Z") => "Z",
        ("C", "X") => "Y",
        ("C", "Y") => "Z",
        ("C", "Z") => "X",
        _ => unreachable!(),
    }
}

fn get_plays(input: &'static str) -> impl Iterator<Item = (&str, &str)> {
    input
        .lines()
        .into_iter()
        .map(|play| play.split_once(' ').unwrap())
}

fn part1(input: &'static str) -> u32 {
    get_plays(input)
        .map(|(op, me)| win_score(op, me) + play_score(me))
        .sum()
}

fn part2(input: &'static str) -> u32 {
    get_plays(input)
        .map(|(op, outcome)| (op, choose_win_or_loss(op, outcome)))
        .map(|(op, me)| win_score(op, me) + play_score(me))
        .sum()
}

fn main() {
    let input = include_str!("actual.input");

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
        let input = include_str!("test.input");

        let part1 = part1(input);
        assert_eq!(part1, 15);

        let part2 = part2(input);
        assert_eq!(part2, 12);
    }
}
