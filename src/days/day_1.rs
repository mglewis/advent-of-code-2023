use advent_of_code_2023::to_u32;

fn find_first_digit(input: &str) -> u32 {
    input.chars().find_map(|c| c.to_digit(10)).unwrap()
}


fn calc_calibration_value(line: &str) -> u32 {
    let first_digit = find_first_digit(line);
    let last_digit = find_first_digit(&line.chars().rev().collect::<String>());
    let formatted_str = format!("{}{}", first_digit.to_string(), last_digit.to_string());
    to_u32(&formatted_str)
}

pub fn part_a(input: &str) -> u32 {
    let calibration_lines = input.split("\n").collect::<Vec<&str>>();
    let calibration_values = calibration_lines
        .iter()
        .map(|x| calc_calibration_value(*x))
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
    fn test_part_a() {
        let input = read_test_file(1);
        assert_eq!(part_a(&input), 142);
    }
}
