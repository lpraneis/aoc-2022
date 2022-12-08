use crate::dirtree::all_dir_sizes;

pub mod dirtree;
pub mod parsers;

fn main() {
    let input = include_str!("../actual.input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &'static str) -> u64 {
    let tree = parsers::parse_input(input);
    all_dir_sizes(tree)
        .1
        .into_iter()
        .filter(|x| x <= &100_000)
        .sum()
}

fn part2(input: &'static str) -> u64 {
    let tree = parsers::parse_input(input);
    let (total, mut dirs) = all_dir_sizes(tree);
    let unused = 70_000_000 - total;
    let need = 30_000_000 - unused;
    dirs.sort();
    for d in dirs {
        if d >= need {
            return d;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../test.input");

        let part1 = part1(input);
        assert_eq!(part1, 95437);

        let part2 = part2(input);
        assert_eq!(part2, 24933642);
    }
}
