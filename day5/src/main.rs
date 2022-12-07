mod parsers;
use itertools::Itertools;
use parsers::parse;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Move {
    from: u32,
    to: u32,
    amt: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Crate(char);

#[derive(Debug, PartialEq, Eq)]
struct CrateStack(Vec<Vec<Crate>>);

impl CrateStack {
    fn read_top(&self) -> String {
        self.0.iter().map(|x| x.last().unwrap().0).join("")
    }

    fn apply_old_crane(&mut self, moves: Vec<Move>) {
        for m in moves {
            for _ in 0..m.amt {
                let c = self.0[m.from as usize].pop().unwrap();
                self.0[m.to as usize].push(c);
            }
        }
    }
    fn apply_crate_mover_9001(&mut self, moves: Vec<Move>) {
        for m in moves {
            let mut popped: Vec<_> = (0..m.amt)
                .map(|_| self.0[m.from as usize].pop().unwrap())
                .collect();
            popped.reverse();
            self.0[m.to as usize].extend(popped);
        }
    }
}

fn part1(input: &'static str) -> String {
    let (mut crates, moves) = parse(input);
    crates.apply_old_crane(moves);
    crates.read_top()
}

fn part2(input: &'static str) -> String {
    let (mut crates, moves) = parse(input);
    crates.apply_crate_mover_9001(moves);
    crates.read_top()
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
        assert_eq!(part1, "CMZ");

        let part2 = part2(input);
        assert_eq!(part2, "MCD");
    }
}
