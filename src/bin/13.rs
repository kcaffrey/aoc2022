use std::cmp::Ordering;

advent_of_code::solution!(13);

macro_rules! entry {
    // Base case for a single element
    ($elem:expr) => {
        Entry::from($elem)
    };

    // Recursively handle nested arrays
    ($($elem:expr),*) => {
        Entry::from(vec![$(entry!($elem)),*])
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|input| {
                let (left, right) = input
                    .trim()
                    .split_once('\n')
                    .expect("should be two entries in the pair");
                (parse(left).1, parse(right).1)
            })
            .enumerate()
            .filter_map(|(i, (left, right))| {
                (left.cmp(&right) == Ordering::Less).then_some(i as u32 + 1)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let divider1 = entry![[2]];
    let divider2 = entry![[6]];
    let mut entries = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| parse(s.trim()).1)
        .collect::<Vec<_>>();
    entries.extend([divider1.clone(), divider2.clone()]);
    entries.sort();
    Some(
        (entries.binary_search(&divider1).ok()? as u32 + 1)
            * (entries.binary_search(&divider2).ok()? as u32 + 1),
    )
}

fn parse(input: &str) -> (&str, Entry) {
    if input.is_empty() {
        unreachable!("got unexpected empty input");
    }
    if let Some(inner) = input.strip_prefix('[') {
        return parse_array(inner);
    }
    let end = input
        .find(|ch: char| !ch.is_numeric())
        .unwrap_or(input.len());
    (
        &input[end..],
        Entry::Scalar(input[..end].parse::<u8>().unwrap()),
    )
}

fn parse_array(mut input: &str) -> (&str, Entry) {
    let mut items = vec![];
    while !input.starts_with(']') {
        let (leftover, entry) = parse(input);
        items.push(entry);
        input = leftover;
        if let Some(stripped) = input.strip_prefix(',') {
            input = stripped;
        } else if !input.starts_with(']') {
            unreachable!("bad array input: {}", input.chars().next().unwrap_or(' '));
        }
    }
    (&input[1..], Entry::List(items))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Entry {
    Scalar(u8),
    List(Vec<Entry>),
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Entry::Scalar(a), Entry::Scalar(b)) => a.cmp(b),
            (Entry::Scalar(a), Entry::List(b)) if b.is_empty() || b[0] < Entry::Scalar(*a) => {
                std::cmp::Ordering::Greater
            }
            (Entry::Scalar(a), Entry::List(b)) if b.len() == 1 && b[0] == Entry::Scalar(*a) => {
                std::cmp::Ordering::Equal
            }
            (Entry::Scalar(_), Entry::List(_)) => std::cmp::Ordering::Less,
            (Entry::List(a), Entry::List(b)) => {
                for i in 0..a.len().min(b.len()) {
                    match a[i].cmp(&b[i]) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                a.len().cmp(&b.len())
            }
            _ => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<u8> for Entry {
    fn from(value: u8) -> Self {
        Self::Scalar(value)
    }
}

impl<T> From<&T> for Entry
where
    T: Into<Entry>,
{
    fn from(value: &T) -> Self {
        value.into()
    }
}

impl<T> From<Vec<T>> for Entry
where
    T: Into<Entry>,
{
    fn from(entrys: Vec<T>) -> Self {
        Entry::List(entrys.into_iter().map(|e| e.into()).collect())
    }
}

impl<T> From<&[T]> for Entry
where
    T: Into<Entry>,
{
    fn from(entrys: &[T]) -> Self {
        Entry::List(entrys.iter().map(|e| e.into()).collect())
    }
}

impl<T, const N: usize> From<[T; N]> for Entry
where
    T: Into<Entry>,
{
    fn from(entrys: [T; N]) -> Self {
        Entry::List(entrys.into_iter().map(|e| e.into()).collect())
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_entry_ordering() {
        let a = entry![1, 1, 3, 1, 1];
        let b = entry![1, 1, 5, 1, 1];
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less, "{:?}, {:?}", a, b);
        let a = entry![[1], [2, 3, 4]];
        let b = entry![[1], 4];
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less, "{:?}, {:?}", a, b);
        let a = entry![9];
        let b = entry![[8, 7, 6]];
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater, "{:?}, {:?}", a, b);
        let a = entry![[4, 4], 4, 4];
        let b = entry![[4, 4], 4, 4, 4];
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less, "{:?}, {:?}", a, b);
    }

    #[test]
    fn test_entry_parse() {
        let (remaining, entry) = parse("[[1],[2,3,4]]");
        assert_eq!(remaining, "");
        assert_eq!(entry, entry![[1], [2, 3, 4]]);
    }
}
