use std::str::FromStr;
use itertools::Itertools;
use crate::aoc;

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
        (self.start <= other.start && other.start <= self.stop) ||
            (self.start <= other.stop && other.stop <= self.stop) ||
            (other.start <= self.start && self.start <= other.stop) ||
            (other.start <= self.stop && self.stop <= other.stop)
    }

    fn from_str(text: &str) -> Result<Self, ParsePointError> {
        let Some((start_raw, stop_raw)) = text.split("-").collect_tuple() else {panic!("could not parse range")};
        let start = start_raw.parse::<T>().map_err(|_| ParsePointError)?;
        let stop = stop_raw.parse::<T>().map_err(|_| ParsePointError)?;

        Ok(Range{ start: start, stop: stop })
    }
}

pub fn main(path: &str) {
    let data = aoc::load_data(path);
    let pairs : Vec<(Range<u64>, Range<u64>)> = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let Some((left_raw, right_raw)) = x.split(",").collect_tuple() else {panic!("could not parse row")};
            let left = Range::<u64>::from_str(left_raw).unwrap();
            let right = Range::<u64>::from_str(right_raw).unwrap();
            (left, right)
        })
        .collect();

    let contained : Vec<bool> = pairs.iter().map(|(a,b)| a.within(b) || b.within(a)).collect();
    println!("{}", contained.iter().filter(|b| **b).count());

    let overlapped : Vec<bool> = pairs.iter().map(|(a,b)| a.overlaps(b)).collect();
    println!("{}", overlapped.iter().filter(|b| **b).count());
}
