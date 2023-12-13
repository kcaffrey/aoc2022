use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .as_bytes()
            .split(|&c| c == b'\n')
            .map(|line| {
                let mut in_first_compartment = [false; 52];
                for &ch in &line[..line.len() / 2] {
                    in_first_compartment[index(ch) as usize] = true;
                }
                for &ch in &line[line.len() / 2..] {
                    let i = index(ch);
                    if in_first_compartment[i as usize] {
                        return i + 1;
                    }
                }
                unreachable!()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .as_bytes()
            .split(|&c| c == b'\n')
            .chunks(3)
            .into_iter()
            .map(|group| {
                let mut counts = [0; 52];
                for elf in group {
                    let mut counted = [false; 52];
                    for &item in elf {
                        let i = index(item);
                        if !counted[i as usize] {
                            counted[i as usize] = true;
                            counts[i as usize] += 1;
                            if counts[i as usize] == 3 {
                                return i + 1;
                            }
                        }
                    }
                }
                unreachable!()
            })
            .sum(),
    )
}

const fn index(ch: u8) -> u32 {
    match ch {
        b'a'..=b'z' => (ch - b'a') as u32,
        b'A'..=b'Z' => (ch - b'A' + 26) as u32,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
