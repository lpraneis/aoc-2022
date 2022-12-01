fn cal_per_elf(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|e| {
            e.split_whitespace()
                .map(|calorie| calorie.parse().unwrap())
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect::<Vec<u32>>()
}

fn part1(input: &str) -> u32 {
    let bags = cal_per_elf(input);
    *bags.iter().max().unwrap()
}

fn part2(input: &str) -> u32 {
    let mut bags = cal_per_elf(input);
    bags.sort_unstable();
    bags.iter().rev().take(3).sum()
}

fn main() {
    let input = include_str!("actual.input");
    let answer = part1(input);
    println!("Part 1: {answer}");

    let answer = part2(input);
    println!("Part 2: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("test.input");

        let part1 = part1(input);
        assert_eq!(part1, 24000);

        let part2 = part2(input);
        assert_eq!(part2, 45000);
    }
}
