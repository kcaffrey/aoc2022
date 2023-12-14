use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let dir_sizes = calculate_dir_sizes(input);
    Some(dir_sizes.into_values().filter(|&v| v <= 100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let dir_sizes = calculate_dir_sizes(input);
    let &root_size = dir_sizes.get("")?;
    let needed_space = 30_000_000_u32.checked_sub(70_000_000 - root_size)?;
    dir_sizes.into_values().filter(|&v| v >= needed_space).min()
}

fn calculate_dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut dir_sizes = HashMap::new();
    let mut cur_path = vec![];
    for line in input.lines() {
        if line.starts_with(char::is_numeric) {
            let (size, _) = line.split_once(' ').unwrap();
            let size = size.parse::<u32>().unwrap();
            for i in 0..=cur_path.len() {
                let path = cur_path[..i].join("/");
                let entry = dir_sizes.entry(path).or_insert(0u32);
                *entry += size;
            }
        } else {
            // command or directory listing
            match &line[..4.min(line.len())] {
                "$ cd" => match &line[5..7.min(line.len())] {
                    "/" => cur_path.clear(),
                    ".." => {
                        cur_path.pop();
                    }
                    _ => cur_path.push(&line[5..]),
                },
                "$ ls" => continue,
                "dir " => continue,
                _ => unreachable!(),
            }
        }
    }
    dir_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24933642));
    }
}
