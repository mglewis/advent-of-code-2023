use advent_of_code_2023::to_u64;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Range {
    fn convert(&self, source: u64) -> Option<u64> {
        let range_offset = source as i64 - self.source_range_start as i64;
        if range_offset < 0 || range_offset >= self.range_length as i64 {
            return None;
        }
        Some(self.destination_range_start + range_offset as u64)
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.splitn(3, " ").map(|x| to_u64(x)).collect::<Vec<u64>>();

        if values.len() != 3 {
            return Err(());
        }
        Ok(Range {
            destination_range_start: values[0],
            source_range_start: values[1],
            range_length: values[2],
        })
    }
}

fn mapping_str_to_mapping(mapping_str: &str) -> Vec<Range> {
    let ranges_str = &mapping_str.trim().split("\n").collect::<Vec<&str>>()[1..];
    ranges_str
        .into_iter()
        .map(|x| Range::from_str(x).unwrap())
        .collect::<Vec<Range>>()
}

fn build_mappings(body_str: &str) -> Vec<Vec<Range>> {
    body_str
        .split("\n\n")
        .map(|x| mapping_str_to_mapping(x))
        .collect::<Vec<Vec<Range>>>()
}

fn convert_id(id: u64, ranges: &Vec<Range>) -> u64 {
    ranges.iter().find_map(|r| r.convert(id)).unwrap_or(id)
}

pub fn part_a(input: &str) -> i64 {
    let (seed_str, body_str) = input.split_once("\n").unwrap();
    let seed_ids = seed_str
        .replace("seeds: ", "")
        .trim()
        .split(" ")
        .map(|x| to_u64(x))
        .collect::<Vec<u64>>();
    let mappings = build_mappings(body_str);

    let final_ids = seed_ids
        .iter()
        .map(|&seed_id| {
            mappings
                .iter()
                .fold(seed_id, |id, ranges| convert_id(id, ranges))
        })
        .collect::<Vec<u64>>();

    final_ids.into_iter().min().unwrap() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_range_from_str() {
        let input = "39 0 15";
        let expected = Range {
            destination_range_start: 39,
            source_range_start: 0,
            range_length: 15,
        };
        assert_eq!(Range::from_str(input).unwrap(), expected);
    }

    #[test]
    fn test_range_convert() {
        let range = Range {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        };
        assert_eq!(range.convert(97), None);
        assert_eq!(range.convert(98), Some(50));
        assert_eq!(range.convert(99), Some(51));
        assert_eq!(range.convert(100), None);
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(5);
        assert_eq!(part_a(&input), 35);
    }
}
