use crate::days::day_7::Part::{PartA, PartB};
use advent_of_code_2023::to_u32;
use itertools::Itertools;
use std::cmp::Ordering;
use std::convert::TryInto;

enum Part {
    PartA,
    PartB,
}

type Cards = [u32; 5];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: Cards,
    bid: u32,
    score_func: fn(Cards) -> u32,
}

impl Hand {
    fn score(&self) -> u32 {
        (self.score_func)(self.cards)
    }
}

fn part_a_score(cards: Cards) -> u32 {
    calculate_score(&cards.to_vec(), 0)
}

fn part_b_score(cards: Cards) -> u32 {
    let cards_without_jokers = cards.into_iter().filter(|&x| x != 1).collect::<Vec<u32>>();
    let joker_count = cards.len() - cards_without_jokers.len();
    calculate_score(&cards_without_jokers, joker_count)
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_score = self.score();
        let other_score = other.score();
        match self_score.cmp(&other_score) {
            Ordering::Equal => {
                // zip the two hands together into pairs
                let pairs = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .collect::<Vec<(&u32, &u32)>>();
                // find the first pair where one card is different to the other
                let comparison = pairs
                    .iter()
                    .find_map(|&(s, o)| {
                        if s.cmp(o).is_eq() {
                            None
                        } else {
                            Some(s.cmp(o))
                        }
                    })
                    .unwrap();
                comparison
            }
            ordering => ordering,
        }
    }
}

fn part_a_card_char_to_rank(c: char) -> u32 {
    if c.is_digit(10) {
        return c.to_digit(10).unwrap();
    }
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unexpected char '{}' when converting to rank", c),
    }
}

fn part_b_card_char_to_rank(c: char) -> u32 {
    if c.is_digit(10) {
        return c.to_digit(10).unwrap();
    }
    match c {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unexpected char '{}' when converting to rank", c),
    }
}

fn calculate_score(cards: &Vec<u32>, joker_count: usize) -> u32 {
    let mut card_counts = cards
        .iter()
        .sorted()
        .group_by(|&&item| item)
        .into_iter()
        .map(|(_, group)| group.count())
        .sorted()
        .rev()
        .collect::<Vec<usize>>();

    if card_counts.len() > 0 {
        card_counts[0] += joker_count;
    } else {
        card_counts = vec![joker_count]
    }

    match card_counts.as_slice() {
        &[5] => 6,          // 5 of a kind
        &[4, 1] => 5,       // 4 of a kind
        &[3, 2] => 4,       // full house
        &[3, 1, 1] => 3,    // three of a kind
        &[2, 2, 1] => 2,    // two pair
        &[2, 1, 1, 1] => 1, // pair
        _ => 0,             // high card
    }
}

fn sort_rank_and_sum_bids(hands: &Vec<Hand>) -> u32 {
    let sorted_hands = hands.iter().cloned().sorted().collect::<Vec<Hand>>();

    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &hand)| acc + ((idx as u32 + 1) * hand.bid))
}

fn hand_from_str(s: &str, part: Part) -> Hand {
    // it's necessary to annotate the function definitions here to avoid a compile time error
    let (score_func, rank_func): (fn(Cards) -> u32, fn(char) -> u32) = match part {
        PartA => (part_a_score, part_a_card_char_to_rank),
        PartB => (part_b_score, part_b_card_char_to_rank),
    };

    let (cards_str, bid_str) = s.split_once(" ").unwrap();
    let cards = cards_str.chars().map(rank_func).collect::<Vec<u32>>();
    Hand {
        cards: cards.try_into().unwrap(),
        bid: to_u32(bid_str),
        score_func: score_func,
    }
}

pub fn part_a(input: &str) -> u32 {
    let hands = input
        .split("\n")
        .map(|x| hand_from_str(x, PartA))
        .collect::<Vec<Hand>>();
    sort_rank_and_sum_bids(&hands)
}

pub fn part_b(input: &str) -> u32 {
    let hands = input
        .split("\n")
        .map(|x| hand_from_str(x, PartB))
        .collect::<Vec<Hand>>();
    sort_rank_and_sum_bids(&hands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_hand_ordering() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let hands = input
            .iter()
            .map(|h| hand_from_str(h, PartA))
            .collect::<Vec<Hand>>();
        let sorted_hands = hands.iter().cloned().sorted().collect::<Vec<Hand>>();
        let actual_bids = sorted_hands.iter().map(|h| h.bid).collect::<Vec<u32>>();
        let expected_bids = vec![765, 220, 28, 684, 483];
        assert_eq!(actual_bids, expected_bids);
    }

    #[test]
    fn test_part_a_hand_score() {
        let full_house = Hand {
            cards: [2, 2, 5, 5, 5],
            bid: 123,
            score_func: part_a_score,
        };
        assert_eq!(full_house.score(), 4);
        let two_pair = Hand {
            cards: [2, 2, 14, 14, 3],
            bid: 123,
            score_func: part_a_score,
        };
        assert_eq!(two_pair.score(), 2);
    }

    #[test]
    fn test_card_char_to_rank() {
        assert_eq!(part_a_card_char_to_rank('3'), 3);
        assert_eq!(part_a_card_char_to_rank('J'), 11);
        assert_eq!(part_a_card_char_to_rank('A'), 14);
        assert_eq!(part_b_card_char_to_rank('6'), 6);
        assert_eq!(part_b_card_char_to_rank('J'), 1);
    }

    #[test]
    fn test_hand_from_str() {
        let input = "32T3J 765";
        let part_a_actual = hand_from_str(input, PartA);
        let part_b_expected = Hand {
            cards: [3, 2, 10, 3, 11],
            bid: 765,
            score_func: part_a_score,
        };
        assert_eq!(part_a_actual, part_b_expected);
        let part_a_actual = hand_from_str(input, PartB);
        let part_b_expected = Hand {
            cards: [3, 2, 10, 3, 1],
            bid: 765,
            score_func: part_b_score,
        };
        assert_eq!(part_a_actual, part_b_expected);
    }

    #[test]
    fn test_calculate_score() {
        assert_eq!(calculate_score(&vec!(10, 8, 9), 2), 3);
        assert_eq!(calculate_score(&Vec::new(), 5), 6);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(7);
        assert_eq!(part_a(&input), 6440);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(7);
        assert_eq!(part_b(&input), 5905);
    }
}
