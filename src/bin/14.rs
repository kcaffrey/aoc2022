use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::parse(input);
    let mut count = 0;
    while map.drop_sand() {
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::parse(input);
    map.use_floor = true;
    let mut count = 0;
    while map.drop_sand() {
        count += 1;
    }
    Some(count)
}

#[derive(Debug, Clone, Default)]
struct Map {
    objects: HashMap<Coordinate, Object>,
    max_y: u16,
    use_floor: bool,
    prev_before_last_settle: Option<Coordinate>,
}

impl Map {
    pub fn parse(input: &str) -> Self {
        let mut objects = HashMap::new();
        let mut max_y = 0;
        for line in input.lines() {
            let mut prev: Option<Coordinate> = None;
            for coord in line.split(" -> ") {
                let (x, y) = coord.split_once(',').expect("should be \"x,y\"");
                let x = x.parse::<u16>().expect("valid x coordinate");
                let y = y.parse::<u16>().expect("valid y coordinate");
                if y > max_y {
                    max_y = y;
                }
                let cur = Coordinate::new(x, y);
                if let Some(prev) = prev {
                    prev.line_to(cur, |coord| {
                        objects.insert(coord, Object::Rock);
                    });
                }
                prev = Some(cur);
            }
        }
        Self {
            objects,
            max_y,
            use_floor: false,
            prev_before_last_settle: None,
        }
    }

    pub fn drop_sand(&mut self) -> bool {
        if !self.drop_sand_with_cache(true) {
            self.drop_sand_with_cache(false)
        } else {
            true
        }
    }

    fn drop_sand_with_cache(&mut self, with_cache: bool) -> bool {
        let mut pos = self
            .prev_before_last_settle
            .filter(|_| with_cache)
            .unwrap_or(Coordinate::new(500, 0));
        while let Some(next) = self.next_available_spot(pos) {
            self.prev_before_last_settle = Some(pos);
            if !self.use_floor && next.y > self.max_y {
                return false;
            }
            pos = next;
        }
        self.objects.insert(pos, Object::Sand).is_none()
    }

    pub fn next_available_spot(&self, coord: Coordinate) -> Option<Coordinate> {
        if let Some(next) = Some(coord.down()).filter(|&c| !self.blocked(c)) {
            return Some(next);
        }
        if let Some(next) = Some(coord.down_left()).filter(|&c| !self.blocked(c)) {
            return Some(next);
        }
        if let Some(next) = Some(coord.down_right()).filter(|&c| !self.blocked(c)) {
            return Some(next);
        }
        None
    }

    fn blocked(&self, coord: Coordinate) -> bool {
        self.objects.contains_key(&coord) || self.use_floor && coord.y >= self.max_y + 2
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Object {
    Rock,
    Sand,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn line_to<F: FnMut(Self)>(self, other: Self, mut f: F) {
        if self.x == other.x {
            let from = self.y.min(other.y);
            let to = self.y.max(other.y);
            for y in from..=to {
                f(Self::new(self.x, y));
            }
        } else if self.y == other.y {
            let from = self.x.min(other.x);
            let to = self.x.max(other.x);
            for x in from..=to {
                f(Self::new(x, self.y));
            }
        } else {
            unreachable!("can't do a horizontal or vertical line when neither coordinate matches");
        }
    }

    pub const fn down(self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub const fn down_left(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    pub const fn down_right(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = 500;
        let mut max_x = 500;
        for key in self.objects.keys() {
            if key.x < min_x {
                min_x = key.x;
            }
            if key.x > max_x {
                max_x = key.x;
            }
        }
        for y in 0..=self.max_y {
            for x in min_x..=max_x {
                let coord = Coordinate::new(x, y);
                let ch = match (coord, self.objects.get(&coord)) {
                    (Coordinate { x, y }, _) if x == 500 && y == 0 => '+',
                    (_, None) => '.',
                    (_, Some(Object::Rock)) => '#',
                    (_, Some(Object::Sand)) => 'o',
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(93));
    }
}
