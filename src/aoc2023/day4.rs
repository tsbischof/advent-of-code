use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, u8},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2023::day4;

    #[test]
    fn test_main() {
        let (part1, part2) = day4::main(&crate::default_path(2023, 4));
        assert_eq!(part1, 27454);
        assert_eq!(part2, 6857330);
    }

    #[test]
    fn test_main_sample() {
        let (part1, part2) = day4::main(&crate::sample_path(2023, 4));
        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }
}

#[derive(Debug, Clone)]
struct Play {
    id: u8,
    winners: Vec<u8>,
    picked: Vec<u8>,
}

impl Play {
    pub fn value(&self) -> u64 {
        if self.n_matches() == 0 {
            0
        } else {
            (2_u64).pow((self.n_matches() as u32) - 1)
        }
    }

    pub fn n_matches(&self) -> usize {
        HashSet::<&u8>::from_iter(self.winners.iter())
            .intersection(&HashSet::<&u8>::from_iter(self.picked.iter()))
            .count()
    }
}

fn u8s_parser(text: &str) -> IResult<&str, Vec<u8>> {
    many1(delimited(space0, u8, space0))(text)
}

fn play_parser(text: &str) -> IResult<&str, Play> {
    map(
        tuple((
            tag("Card"),
            delimited(space0, u8, tag(": ")),
            terminated(u8s_parser, tag("|")),
            u8s_parser,
        )),
        |(_, id, winners, picked)| Play {
            id,
            winners,
            picked,
        },
    )(text)
}

pub fn main(data_path: &PathBuf) -> (u64, u64) {
    let data = crate::load_data(data_path);
    let (_, plays) = separated_list0(newline, play_parser)(data.as_str()).unwrap();
    let mut counts = HashMap::new();
    let values = plays.iter().map(|p| p.value());

    for play in plays.iter() {
        counts.entry(play.id).or_insert(1);
        let copies = *counts.get(&play.id).unwrap();
        for offset in 1..=play.n_matches() {
            let id = play.id + (offset as u8);
            counts
                .entry(id)
                .and_modify(|c| *c += copies)
                .or_insert(copies + 1);
        }
    }

    let part1 = values.sum();
    let part2 = counts.values().sum();

    (part1, part2)
}
