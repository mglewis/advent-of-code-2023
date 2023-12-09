use advent_of_code_2023::to_u32;
use itertools::iproduct;
use std::cmp;

fn check_validity_around_point(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    let min_x = cmp::max(0, (x as i32) - 1) as usize;
    let max_x = cmp::min(x + 1, grid[y].len() - 1);
    let min_y = cmp::max(0, (y as i32) - 1) as usize;
    let max_y = cmp::min(y + 1, grid.len() - 1);

    let x_range = min_x..max_x + 1;
    let y_range = min_y..max_y + 1;

    let search_coords = iproduct!(x_range, y_range)
        .filter(|coords| *coords != (x, y))
        .collect::<Vec<(usize, usize)>>();

    for (x_coord, y_coord) in search_coords {
        let cell = grid[y_coord][x_coord];
        let is_symbol = cell != '.' && !cell.is_ascii_alphanumeric();
        if is_symbol {
            return true;
        }
    }
    false
}

pub fn part_a(input: &str) -> u32 {
    let grid = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

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
    fn test_part_a() {
        let input = read_test_file(3);
        assert_eq!(part_a(&input), 4361);
    }
}
