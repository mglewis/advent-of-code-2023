use advent_of_code_2023::to_u32;

struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    fn record_beating_permutations(&self) -> u32 {
        let distance_target = (self.distance + 1) as f64;
        let time_to_race = -1.0 * self.time as f64;
        let (x1, x2) = solve_quadratic(1.0, time_to_race, distance_target);
        let lower_bound = f64::min(x1, x2).ceil() as u32;
        let upper_bound = f64::max(x1, x2) as u32;
        upper_bound - lower_bound + 1
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant <= 0.0 {
        panic!("Unable to find a valid solution");
    }
    let sqrt_discriminant = discriminant.sqrt();
    let x1 = (-b + sqrt_discriminant) / (2.0 * a);
    let x2 = (-b - sqrt_discriminant) / (2.0 * a);
    (x1, x2)
}

fn to_numeric_list(raw_str: &str, label: &str) -> Vec<u32> {
    let cleaned_str = raw_str.replace(label, "").trim().to_string();
    cleaned_str
        .split(" ")
        .filter(|x| x.len() != 0)
        .map(|x| to_u32(x))
        .collect::<Vec<u32>>()
}

pub fn part_a(input: &str) -> u32 {
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let times = to_numeric_list(time_str, "Time:");
    let distances = to_numeric_list(distance_str, "Distance:");

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| Race {
            time: t,
            distance: d,
        })
        .collect::<Vec<Race>>();

    let race_permutations = races
        .iter()
        .map(|r| r.record_beating_permutations())
        .collect::<Vec<u32>>();
    race_permutations.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2023::read_test_file;

    #[test]
    fn test_race_record_beating_permutations() {
        let race = Race {
            time: 30,
            distance: 200,
        };
        assert_eq!(race.record_beating_permutations(), 9)
    }

    #[test]
    fn test_part_a() {
        let input = read_test_file(6);
        assert_eq!(part_a(&input), 288);
    }
}
