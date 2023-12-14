use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let heights = parse_heights(input);
    let mut visible = vec![vec![false; heights[0].len()]; heights.len()];
    for (row_idx, row) in heights.iter().enumerate() {
        let mut max_from_left = -1;
        for (col_idx, &height) in row.iter().enumerate() {
            if height > max_from_left {
                visible[row_idx][col_idx] = true;
                max_from_left = height;
            }
        }
        let mut max_from_right = -1;
        for (col_idx, &height) in row.iter().enumerate().rev() {
            if height > max_from_right {
                visible[row_idx][col_idx] = true;
                max_from_right = height;
            }
        }
    }
    for col_idx in 0..heights[0].len() {
        let mut max_from_top = -1;
        for row_idx in 0..heights.len() {
            let height = heights[row_idx][col_idx];
            if height > max_from_top {
                visible[row_idx][col_idx] = true;
                max_from_top = height;
            }
        }
        let mut max_from_bottom = -1;
        for row_idx in (0..heights.len()).rev() {
            let height = heights[row_idx][col_idx];
            if height > max_from_bottom {
                visible[row_idx][col_idx] = true;
                max_from_bottom = height;
            }
        }
    }
    Some(
        visible
            .into_iter()
            .flat_map(|r| r.into_iter())
            .filter(|&v| v)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let heights = parse_heights(input);
    (1..heights.len() - 1)
        .into_par_iter()
        .flat_map(|r| (1..heights[r].len() - 1).map(move |c| (r, c)).par_bridge())
        .map(|(r, c)| visibility(&heights, r, c))
        .max()
}

fn visibility(heights: &Vec<Vec<i8>>, row: usize, col: usize) -> u32 {
    let height = heights[row][col];
    let left_visibility = (1..=col)
        .find(|o| heights[row][col - o] >= height)
        .unwrap_or(col) as u32;
    let right_visibility = (1..=(heights[row].len() - col - 1))
        .find(|o| heights[row][col + o] >= height)
        .unwrap_or(heights[row].len() - col - 1) as u32;
    let up_visibility = (1..=row)
        .find(|o| heights[row - o][col] >= height)
        .unwrap_or(row) as u32;
    let down_visibility = (1..=(heights.len() - row - 1))
        .find(|o| heights[row + o][col] >= height)
        .unwrap_or(heights.len() - row - 1) as u32;
    left_visibility * right_visibility * up_visibility * down_visibility
}

fn parse_heights(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("a digit") as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
