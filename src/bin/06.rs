advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    find_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_marker(input, 14)
}

fn find_marker(input: &str, uniq_len: u32) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .windows(uniq_len as usize)
            .position(is_all_different)
            .unwrap() as u32
            + uniq_len,
    )
}

fn is_all_different(group: &[u8]) -> bool {
    let mut counts = [0; 26];
    for &ch in group {
        let index = (ch - b'a') as usize;
        counts[index] += 1;
        if counts[index] > 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
