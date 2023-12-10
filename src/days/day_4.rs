use advent_of_code_2023::to_u32;
use num::pow;
use std::cmp;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Scratchcard {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Scratchcard {
    fn matching_numbers(&self) -> usize {
        self.drawn_numbers
            .iter()
            .filter(|&&d| self.winning_numbers.contains(&&d))
            .count()
    }

    fn winning_points(&self) -> u32 {
        let matches = self.matching_numbers();
        if matches == 0 {
            return 0;
        }
        pow(2, matches - 1)
    }
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_str, body_str) = s.split_once(":").unwrap();
        let id = to_u32(&id_str.replace("Card ", "").trim());
        let (winning_str, drawing_str) = body_str.split_once("|").unwrap();

        let winning_numbers = str_to_num_vec(winning_str);
        let drawn_numbers = str_to_num_vec(drawing_str);

        Ok(Scratchcard {
            id,
            winning_numbers,
            drawn_numbers,
        })
    }
}

fn str_to_num_vec(s: &str) -> Vec<u32> {
    s.trim()
        .replace("  ", " ")
        .split(" ")
        .map(|x| to_u32(x.trim()))
        .collect::<Vec<u32>>()
}

pub fn part_a(input: &str) -> u32 {
    let scratchcards = input
        .split("\n")
        .map(|x| Scratchcard::from_str(x).unwrap())
        .collect::<Vec<Scratchcard>>();
    scratchcards.iter().map(|s| s.winning_points()).sum()
}

pub fn part_b(input: &str) -> u32 {
    let scratchcards = input
        .split("\n")
        .map(|x| Scratchcard::from_str(x).unwrap())
        .collect::<Vec<Scratchcard>>();
    let scratchcard_matches = scratchcards
        .iter()
        .map(|s| s.matching_numbers())
        .collect::<Vec<usize>>();
    let mut scratchcard_plays = vec![1u32; scratchcard_matches.len()];

    for (match_idx, matching_numbers) in scratchcard_matches.iter().enumerate() {
        let range_start_idx = cmp::min(match_idx + 1, scratchcard_plays.len());
        let range_end_idx = cmp::min(match_idx + matching_numbers + 1, scratchcard_plays.len());
        let plays = scratchcard_plays[match_idx];
        for plays_idx in range_start_idx..range_end_idx {
            scratchcard_plays[plays_idx] += plays;
        }
    }
    scratchcard_plays.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_scratchcard_from_str() {
        let input = "Card 2: 13 32 | 61 30 68  9";
        let actual = Scratchcard::from_str(input).unwrap();
        let expected = Scratchcard {
            id: 2,
            winning_numbers: vec![13, 32],
            drawn_numbers: vec![61, 30, 68, 9],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(4);
        assert_eq!(part_a(&input), 13);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(4);
        assert_eq!(part_b(&input), 30);
    }
}
