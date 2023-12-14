use std::collections::HashSet;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    solve::<2>(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve::<10>(input)
}

fn solve<const N: usize>(input: &str) -> Option<u32> {
    let mut snake = Snake::<N>::new();
    let mut visited = HashSet::new();
    visited.insert(snake.tail());
    for motion in input.lines().map(Motion::parse) {
        for _ in 0..motion.steps {
            snake.do_step(motion.direction);
            visited.insert(snake.tail());
        }
    }
    Some(visited.len() as u32)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Snake<const N: usize> {
    knots: [Position; N],
}

impl<const N: usize> Snake<N> {
    pub const fn new() -> Self {
        Self {
            knots: [Position::origin(); N],
        }
    }

    pub const fn tail(self) -> Position {
        self.knots[N - 1]
    }

    pub fn do_step(&mut self, direction: Direction) {
        self.knots[0] = self.knots[0].go(direction);
        for i in 1..N {
            self.knots[i] = self.knots[i].follow(self.knots[i - 1]);
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    const fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    const fn go(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.x, self.y + 1),
            Direction::Down => Self::new(self.x, self.y - 1),
            Direction::Left => Self::new(self.x - 1, self.y),
            Direction::Right => Self::new(self.x + 1, self.y),
        }
    }

    const fn follow(self, head: Self) -> Self {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            return self;
        }
        Self {
            x: self.x + x_diff.signum(),
            y: self.y + y_diff.signum(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Motion {
    steps: u8,
    direction: Direction,
}

impl Motion {
    fn parse(line: &str) -> Self {
        let (direction, steps) = line.split_once(' ').expect("should be a valid motion");
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let steps = steps
            .parse::<u8>()
            .expect("should be an integer number of steps");
        Self { steps, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
