use advent_of_code_2023::to_i64;
use itertools::Itertools;

fn process_line(line: &str) -> Vec<i64> {
    line.split_whitespace().map(to_i64).collect_vec()
}

fn process_sequence(seq: Vec<i64>) -> i64 {
    let differences = seq
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect_vec();

    if differences.iter().all(|&x| x == 0) {
        return *seq.last().unwrap();
    } else {
        return seq.last().unwrap() + process_sequence(differences);
    }
}

pub fn part_a(input: &str) -> i64 {
    let sequences = input.lines().map(process_line).collect_vec();
    let next_values = sequences.into_iter().map(process_sequence).collect_vec();
    next_values.iter().sum()
}

pub fn part_b(input: &str) -> i64 {
    let sequences = input
        .lines()
        .map(|x| process_line(x).into_iter().rev().collect_vec())
        .collect_vec();
    let next_values = sequences.into_iter().map(process_sequence).collect_vec();
    next_values.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_process_sequence() {
        let input1 = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(process_sequence(input1), 18);

        let input2 = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(process_sequence(input2), 68);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(9);
        assert_eq!(part_a(&input), 114);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(9);
        assert_eq!(part_b(&input), 2);
    }
}
