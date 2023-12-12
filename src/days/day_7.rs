use advent_of_code_2023::to_u32;
use itertools::Itertools;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: [u32; 5],
    bid: u32,
}

impl Hand {
    fn score(&self) -> u32 {
        let card_counts = self
            .cards
            .iter()
            .sorted()
            .group_by(|&&item| item)
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev()
            .collect::<Vec<usize>>();

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

    #[allow(dead_code)] // useful for debugging / viz but not currently used
    fn hand_type(&self) -> &'static str {
        match self.score() {
            6 => "Five of a kind",
            5 => "Four of a kind",
            4 => "Full house",
            3 => "Three of a kind",
            2 => "Two pair",
            1 => "Pair",
            _ => "High Card",
        }
    }
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

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(" ").unwrap();
        let cards = cards_str
            .chars()
            .map(card_char_to_rank)
            .collect::<Vec<u32>>();
        Ok(Hand {
            cards: cards.try_into().unwrap(),
            bid: to_u32(bid_str),
        })
    }
}

fn card_char_to_rank(c: char) -> u32 {
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

pub fn part_a(input: &str) -> u32 {
    let hands = input
        .split("\n")
        .map(|x| Hand::from_str(x).unwrap())
        .collect::<Vec<Hand>>();
    let sorted_hands = hands.iter().sorted().collect::<Vec<&Hand>>();

    sorted_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &hand)| acc + ((idx as u32 + 1) * hand.bid))
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
            .map(|h| Hand::from_str(h).unwrap())
            .collect::<Vec<Hand>>();
        let sorted_hands = hands.iter().cloned().sorted().collect::<Vec<Hand>>();
        let actual_bids = sorted_hands.iter().map(|h| h.bid).collect::<Vec<u32>>();
        let expected_bids = vec![765, 220, 28, 684, 483];
        assert_eq!(actual_bids, expected_bids);
    }

    #[test]
    fn test_hand_score() {
        let full_house = Hand {
            cards: [2, 2, 5, 5, 5],
            bid: 123,
        };
        assert_eq!(full_house.score(), 4);
        let two_pair = Hand {
            cards: [2, 2, 14, 14, 3],
            bid: 123,
        };
        assert_eq!(two_pair.score(), 2);
    }

    #[test]
    fn test_card_char_to_rank() {
        assert_eq!(card_char_to_rank('3'), 3);
        assert_eq!(card_char_to_rank('J'), 11);
        assert_eq!(card_char_to_rank('A'), 14);
    }

    #[test]
    fn test_hand_from_str() {
        let input = "32T3K 765";
        let actual = Hand::from_str(input).unwrap();
        let expected = Hand {
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(7);
        assert_eq!(part_a(&input), 6440);
    }
}
