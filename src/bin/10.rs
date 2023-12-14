use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<i32> {
    let mut cycle = 1;
    let mut x = 1;
    let mut signal_strength_sum = 0;
    for line in input.lines() {
        if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
            signal_strength_sum += cycle * x;
        }
        let noop = line.starts_with(|ch| ch == 'n');
        cycle += 1;
        if noop {
            continue;
        }
        if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
            signal_strength_sum += cycle * x;
        }
        let v = line[5..].parse::<i32>().expect("valid instruction");
        cycle += 1;
        x += v;
    }
    Some(signal_strength_sum)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crt = Crt::new();
    let mut x = 1;
    for line in input.lines() {
        crt.do_cycle(x);
        let noop = line.starts_with(|ch| ch == 'n');
        if noop {
            continue;
        }
        crt.do_cycle(x);
        let v = line[5..].parse::<i32>().expect("valid instruction");
        x += v;
    }
    Some(crt.draw())
}

struct Crt {
    cycle: usize,
    screen: [[char; 40]; 6],
}

impl Crt {
    const fn new() -> Self {
        Self {
            cycle: 0,
            screen: [[' '; 40]; 6],
        }
    }

    pub fn draw(&self) -> String {
        self.screen
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n")
    }

    pub fn do_cycle(&mut self, x: i32) {
        let (row, col) = (self.cycle / 40, self.cycle % 40);
        if x.abs_diff(col as i32) <= 1 {
            self.screen[row][col] = '#';
        } else {
            self.screen[row][col] = '.';
        }
        self.cycle += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let image = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;
        assert_eq!(result, Some(image.to_owned()));
    }
}
