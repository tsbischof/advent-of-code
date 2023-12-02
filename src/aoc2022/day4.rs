use itertools::Itertools;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2022::day4;

    #[test]
    fn test_main() {
        let (part1, part2) = day4::main(&crate::default_path(2022, 4));
        assert_eq!(part1, 498);
        assert_eq!(part2, 859);
    }

    #[test]
    fn test_main_sample() {
        let (part1, part2) = day4::main(&crate::sample_path(2022, 4));
        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Range<T> {
    start: T,
    stop: T,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl<T: std::cmp::PartialOrd + std::str::FromStr> Range<T> {
    fn within(self: &Range<T>, other: &Range<T>) -> bool {
        self.start >= other.start && self.stop <= other.stop
    }

    fn overlaps(self: &Range<T>, other: &Range<T>) -> bool {
        (self.start <= other.start && other.start <= self.stop)
            || (self.start <= other.stop && other.stop <= self.stop)
            || (other.start <= self.start && self.start <= other.stop)
            || (other.start <= self.stop && self.stop <= other.stop)
    }

    fn from_str(text: &str) -> Result<Self, ParsePointError> {
        let Some((start_raw, stop_raw)) = text.split('-').collect_tuple() else {panic!("could not parse range")};
        let start = start_raw.parse::<T>().map_err(|_| ParsePointError)?;
        let stop = stop_raw.parse::<T>().map_err(|_| ParsePointError)?;

        Ok(Range { start, stop })
    }
}

pub fn main(path: &PathBuf) -> (usize, usize) {
    let data = crate::load_data(path);
    let pairs : Vec<(Range<u64>, Range<u64>)> = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            let Some((left_raw, right_raw)) = x.split(',').collect_tuple() else {panic!("could not parse row")};
            let left = Range::<u64>::from_str(left_raw).unwrap();
            let right = Range::<u64>::from_str(right_raw).unwrap();
            (left, right)
        })
        .collect();

    let contained: Vec<bool> = pairs
        .iter()
        .map(|(a, b)| a.within(b) || b.within(a))
        .collect();
    let part1 = contained.iter().filter(|b| **b).count();

    let overlapped: Vec<bool> = pairs.iter().map(|(a, b)| a.overlaps(b)).collect();
    let part2 = overlapped.iter().filter(|b| **b).count();

    (part1, part2)
}
