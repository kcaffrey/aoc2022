use smallvec::SmallVec;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let mut input = parse_input(input);
    for inst in input.procedure {
        for _ in 0..inst.amount {
            let c = input.stacks[inst.from].pop().unwrap();
            input.stacks[inst.to].push(c);
        }
    }
    Some(
        input
            .stacks
            .into_iter()
            .map(|s| *s.last().unwrap())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let mut input = parse_input(input);
    for inst in input.procedure {
        let crane_storage = {
            let from = &mut input.stacks[inst.from];
            from.drain(from.len() - inst.amount..)
                .collect::<SmallVec<[char; 64]>>()
        };
        input.stacks[inst.to].extend(crane_storage);
    }
    Some(
        input
            .stacks
            .into_iter()
            .map(|s| *s.last().unwrap())
            .collect(),
    )
}

#[derive(Debug, Clone)]
struct Input {
    stacks: Vec<Vec<char>>,
    procedure: Vec<Instruction>,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> Input {
    let (stacks, procedure) = input.split_once("\n\n").unwrap();
    Input {
        stacks: parse_stacks(stacks),
        procedure: procedure.trim().lines().map(parse_instruction).collect(),
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let input = input.as_bytes();
    let cols = input.iter().position(|&c| c == b'\n').unwrap();
    let rows = (input.len() + 2) / (cols + 1);
    let num_stacks = (cols + 1) / 4;
    let mut stacks = vec![vec![]; num_stacks];
    for row in (0..rows - 1).rev() {
        for stack in 0..num_stacks {
            let starting_crate = input[row * (cols + 1) + stack * 4 + 1];
            if starting_crate != b' ' {
                stacks[stack].push(starting_crate as char);
            }
        }
    }
    stacks
}

fn parse_instruction(input: &str) -> Instruction {
    let mut parts = input
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .map(|s| s.parse::<usize>().unwrap());
    Instruction {
        amount: parts.next().unwrap(),
        from: parts.next().unwrap() - 1,
        to: parts.next().unwrap() - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".to_owned()));
    }
}
