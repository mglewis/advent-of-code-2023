use advent_of_code_2023::to_u32;

const DIGIT_MAPPINGS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(input: &str) -> u32 {
    input.chars().find_map(|c| c.to_digit(10)).unwrap()
}

fn calc_calibration_value_part_a(line: &str) -> u32 {
    let first_digit = find_first_digit(line);
    let last_digit = find_first_digit(&line.chars().rev().collect::<String>());
    let formatted_str = format!("{}{}", first_digit.to_string(), last_digit.to_string());
    to_u32(&formatted_str)
}

fn matches_digit(current_char: char, idx: usize, input: &str) -> Option<u32> {
    if current_char.is_digit(10) {
        return current_char.to_digit(10);
    }

    for &(word, digit) in DIGIT_MAPPINGS.iter() {
        if input[idx..].starts_with(word) {
            return Some(digit);
        }
    }
    None
}

fn find_first_digit_incl_text_repr(input: &str) -> u32 {
    input
        .chars()
        .enumerate()
        .find_map(|(idx, c)| matches_digit(c, idx, input))
        .unwrap()
}

fn find_last_digit_incl_text_repr(input: &str) -> u32 {
    for n in (0..input.len()).rev() {
        let c = input.chars().nth(n).unwrap();
        if let Some(digit) = matches_digit(c, n, &input) {
            return digit;
        }
    }
    panic!("The input {} should always contain a digit", input);
}

fn calc_calibration_value_part_b(line: &str) -> u32 {
    let first_digit = find_first_digit_incl_text_repr(line);
    let last_digit = find_last_digit_incl_text_repr(line);
    let formatted_str = format!("{}{}", first_digit.to_string(), last_digit.to_string());
    to_u32(&formatted_str)
}

pub fn part_a(input: &str) -> u32 {
    let calibration_lines = input.split("\n").collect::<Vec<&str>>();
    let calibration_values = calibration_lines
        .iter()
        .map(|x| calc_calibration_value_part_a(*x))
        .collect::<Vec<u32>>();
    calibration_values.iter().sum::<u32>()
}

pub fn part_b(input: &str) -> u32 {
    let calibration_lines = input.split("\n").collect::<Vec<&str>>();
    let calibration_values = calibration_lines
        .iter()
        .map(|x| calc_calibration_value_part_b(&x))
        .collect::<Vec<u32>>();
    calibration_values.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_find_first_digit() {
        let input = "abcde123";
        assert_eq!(find_first_digit(input), 1);
    }

    #[test]
    fn test_find_first_digit_incl_text_repr() {
        let input = "five2threefour";
        assert_eq!(find_first_digit_incl_text_repr(input), 5);
    }

    #[test]
    fn test_find_last_digit_incl_text_repr() {
        let input = "twosix8nine";
        assert_eq!(find_last_digit_incl_text_repr(input), 9);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(1);
        assert_eq!(part_a(&input), 165);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(1);
        assert_eq!(part_b(&input), 159);
    }
}
