use advent_of_code_2023::to_u32;
use itertools::{iproduct, Itertools};
use std::cmp;

fn get_search_coords(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let min_x = cmp::max(0, (x as i32) - 1) as usize;
    let max_x = cmp::min(x + 1, grid[y].len() - 1);
    let min_y = cmp::max(0, (y as i32) - 1) as usize;
    let max_y = cmp::min(y + 1, grid.len() - 1);

    let x_range = min_x..max_x + 1;
    let y_range = min_y..max_y + 1;

    iproduct!(x_range, y_range)
        .filter(|coords| *coords != (x, y))
        .collect::<Vec<(usize, usize)>>()
}

fn check_validity_around_point(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    for (x_coord, y_coord) in get_search_coords(x, y, grid) {
        let cell = grid[y_coord][x_coord];
        let is_symbol = cell != '.' && !cell.is_ascii_alphanumeric();
        if is_symbol {
            return true;
        }
    }
    false
}

fn find_multipliers(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut multiplier_idxs = Vec::<(usize, usize)>::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '*' {
                multiplier_idxs.push((x, y));
            }
        }
    }
    multiplier_idxs
}

fn find_full_number(x: usize, y: usize, grid: &Vec<Vec<char>>) -> u32 {
    let ref row = grid[y];
    let mut start_idx = x;
    let mut end_idx = x;

    while start_idx >= 1 && row[start_idx - 1].is_digit(10) {
        start_idx -= 1;
    }
    while end_idx < row.len() - 1 && row[end_idx + 1].is_digit(10) {
        end_idx += 1;
    }
    let number_str = &row[start_idx..end_idx + 1].iter().collect::<String>();
    to_u32(number_str)
}

fn calculate_multiplier(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Option<u32> {
    let search_coords = get_search_coords(x, y, grid);
    let neighbouring_digit_coords = search_coords
        .iter()
        .filter(|(x_c, y_c)| grid[*y_c][*x_c].is_digit(10))
        .collect::<Vec<&(usize, usize)>>();

    let neighbouring_numbers = neighbouring_digit_coords
        .iter()
        .map(|(x_c, y_c)| find_full_number(*x_c, *y_c, grid))
        .unique()
        .collect::<Vec<u32>>();

    if neighbouring_numbers.len() == 2 {
        return Some(neighbouring_numbers[0] * neighbouring_numbers[1]);
    }
    None
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn part_a(input: &str) -> u32 {
    let grid = build_grid(input);

    let mut part_sum = 0u32;

    for y in 0..grid.len() {
        let mut current_number = Vec::<char>::new();
        let mut current_number_adjacent_to_symbol = false;

        for x in 0..grid[y].len() {
            let current_char = grid[y][x];

            if current_char.is_digit(10) {
                current_number.push(current_char);
                if !current_number_adjacent_to_symbol {
                    current_number_adjacent_to_symbol = check_validity_around_point(x, y, &grid);
                }
            }

            if !current_char.is_digit(10) || x == grid[y].len() - 1 {
                if current_number.len() > 0 && current_number_adjacent_to_symbol {
                    part_sum += to_u32(&current_number.iter().collect::<String>());
                }
                // reset ready for next number
                current_number = Vec::<char>::new();
                current_number_adjacent_to_symbol = false;
            }
        }
    }
    part_sum
}

pub fn part_b(input: &str) -> u32 {
    let grid = build_grid(input);
    let multipliers = find_multipliers(&grid);
    let mut part_sum = 0u32;

    for (x, y) in multipliers {
        if let Some(result) = calculate_multiplier(x, y, &grid) {
            part_sum += result;
        }
    }
    part_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_check_validity_around_point() {
        let grid = vec![
            vec!['.', '.', '#'],
            vec!['.', 'a', '.'],
            vec!['c', '.', '.'],
        ];
        assert_eq!(check_validity_around_point(1, 1, &grid), true);
        assert_eq!(check_validity_around_point(0, 1, &grid), false);
        assert_eq!(check_validity_around_point(2, 1, &grid), true);
        assert_eq!(check_validity_around_point(0, 2, &grid), false);
    }

    #[test]
    fn test_find_multipliers() {
        let grid = vec![
            vec!['*', '.', '#'],
            vec!['.', '*', '.'],
            vec!['c', '*', '.'],
        ];
        let expected = vec![(0, 0), (1, 1), (1, 2)];
        assert_eq!(find_multipliers(&grid), expected);
    }

    #[test]
    fn test_find_full_number() {
        let grid = vec![vec!['.', '.', '1', '2', '3', '*']];
        assert_eq!(find_full_number(3, 0, &grid), 123);

        let grid_ending_in_number = vec![vec!['.', '1', '2', '5']];
        assert_eq!(find_full_number(1, 0, &grid_ending_in_number), 125);

        let grid_starting_with_number = vec![vec!['6', '6', '.', '.']];
        assert_eq!(find_full_number(1, 0, &grid_starting_with_number), 66);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(3);
        assert_eq!(part_a(&input), 4361);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(3);
        assert_eq!(part_b(&input), 467835);
    }
}
