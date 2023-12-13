advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                score_round(line[0], choose_play_part1(line[2]))
            })
            .sum(),
    )
}

const fn choose_play_part1(yours: u8) -> u8 {
    match yours {
        b'X' => b'A',
        b'Y' => b'B',
        b'Z' => b'C',
        _ => unreachable!(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                score_round(line[0], choose_play_part2(line[0], line[2]))
            })
            .sum(),
    )
}

const fn choose_play_part2(theirs: u8, yours: u8) -> u8 {
    match yours {
        b'X' => loser(theirs),
        b'Y' => theirs,
        b'Z' => winner(theirs),
        _ => unreachable!(),
    }
}

const fn winner(play: u8) -> u8 {
    match play {
        b'A' => b'B',
        b'B' => b'C',
        b'C' => b'A',
        _ => unreachable!(),
    }
}

const fn loser(play: u8) -> u8 {
    match play {
        b'A' => b'C',
        b'B' => b'A',
        b'C' => b'B',
        _ => unreachable!(),
    }
}

const fn score_round(theirs: u8, yours: u8) -> u32 {
    score_play(yours)
        + match (theirs, yours) {
            (a, b) if a == b => 3,
            (a, b) if b == winner(a) => 6,
            _ => 0,
        }
}

const fn score_play(play: u8) -> u32 {
    match play {
        b'A' => 1,
        b'B' => 2,
        b'C' => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
