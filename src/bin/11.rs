use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some((to, value)) = monkeys[i].inspect_one() {
                monkeys[to].catch(value);
            }
        }
    }
    Some(
        monkeys
            .into_iter()
            .map(|monkey| monkey.inspected_count)
            .sorted()
            .rev()
            .take(2)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = input.split("\n\n").map(parse_monkey).collect::<Vec<_>>();
    let modulo = monkeys.iter().map(|monkey| monkey.divisibility).product();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some((to, value)) = monkeys[i].inspect_one_modulo(modulo) {
                monkeys[to].catch(value);
            }
        }
    }
    Some(
        monkeys
            .into_iter()
            .map(|monkey| monkey.inspected_count)
            .sorted()
            .rev()
            .take(2)
            .product(),
    )
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines().skip(1);
    let starting_items = lines.next().unwrap();
    let (_, starting_items) = starting_items.split_once(':').unwrap();
    let starting_items = starting_items
        .trim()
        .split(", ")
        .map(str::parse)
        .collect::<Result<VecDeque<u64>, _>>()
        .unwrap();
    let operation = lines.next().unwrap();
    let (_, operation) = operation.split_once(" = ").unwrap();
    let divisibility = lines.next().unwrap()[21..].parse::<u64>().unwrap();
    let if_true = lines.next().unwrap()[29..].parse::<usize>().unwrap();
    let if_false = lines.next().unwrap()[30..].parse::<usize>().unwrap();

    Monkey {
        items: starting_items,
        operation: parse_operation(operation),
        divisibility,
        if_true,
        if_false,
        inspected_count: 0,
    }
}

fn parse_operation(input: &str) -> Operation {
    let mut parts = input.split_whitespace();
    let left = parts.next().unwrap();
    let operator = parts.next().unwrap();
    let right = parts.next().unwrap();
    let operator = match operator {
        "*" => Operator::Multiply,
        "+" => Operator::Plus,
        _ => unreachable!(),
    };
    Operation {
        left: parse_operand(left),
        right: parse_operand(right),
        operator,
    }
}

fn parse_operand(input: &str) -> Operand {
    match input {
        "old" => Operand::Old,
        _ => Operand::Constant(input.parse().unwrap()),
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisibility: u64,
    if_true: usize,
    if_false: usize,
    inspected_count: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operand {
    Old,
    Constant(u64),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Plus,
    Multiply,
}

impl Monkey {
    pub fn inspect_one(&mut self) -> Option<(usize, u64)> {
        let front = self.items.pop_front()?;
        let post_inspected = self.operation.apply(front) / 3;
        self.inspected_count += 1;
        let divisible = post_inspected % self.divisibility == 0;
        if divisible {
            Some((self.if_true, post_inspected))
        } else {
            Some((self.if_false, post_inspected))
        }
    }

    pub fn inspect_one_modulo(&mut self, modulo: u64) -> Option<(usize, u64)> {
        let front = self.items.pop_front()?;
        let post_inspected = self.operation.apply(front) % modulo;
        self.inspected_count += 1;
        let divisible = post_inspected % self.divisibility == 0;
        if divisible {
            Some((self.if_true, post_inspected))
        } else {
            Some((self.if_false, post_inspected))
        }
    }

    pub fn catch(&mut self, value: u64) {
        self.items.push_back(value);
    }
}

impl Operation {
    const fn apply(self, old: u64) -> u64 {
        let left = self.left.resolve(old);
        let right = self.right.resolve(old);
        match self.operator {
            Operator::Plus => left + right,
            Operator::Multiply => left * right,
        }
    }
}

impl Operand {
    const fn resolve(self, old: u64) -> u64 {
        match self {
            Operand::Old => old,
            Operand::Constant(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2713310158));
    }
}
