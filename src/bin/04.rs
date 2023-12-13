advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let (left, right) = line.split_once(',').unwrap();
                let (left, right) = (parse_range(left), parse_range(right));
                right.0 >= left.0 && right.1 <= left.1 || left.0 >= right.0 && left.1 <= right.1
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| {
                let (left, right) = line.split_once(',').unwrap();
                let (left, right) = (parse_range(left), parse_range(right));
                !(right.0 > left.1 || right.1 < left.0)
            })
            .count() as u32,
    )
}

fn parse_range(s: &str) -> (u32, u32) {
    let (start, end) = s.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
