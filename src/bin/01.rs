use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|elf| elf.lines().flat_map(str::parse::<u32>).sum())
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|elf| elf.lines().flat_map(str::parse::<u32>).sum::<u32>())
            .sorted()
            .rev()
            .take(3)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
