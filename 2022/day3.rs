use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::aoc;

#[cfg(test)]
mod tests {
    use crate::day3;

    #[test]
    fn test_main() {
        let (part1, part2) = day3::main("day3/input.txt");
        assert_eq!(part1, 7997);
        assert_eq!(part2, 2545);
    }
}

fn split_compartments(sack: &str) -> Vec<&str> {
    let mid = sack.len() / 2;
    vec![&sack[..mid], &sack[mid..]]
}

fn same_types(left: &str, right: &str) -> Vec<char> {
    let mut l: Vec<char> = left.chars().collect();
    let mut r: Vec<char> = right.chars().collect();

    l.sort();
    r.sort();
    let mut li = 0;
    let mut ri = 0;

    let mut overlap = Vec::new();

    while li < l.len() && ri < r.len() {
        match l[li].cmp(&r[ri]) {
            Ordering::Less => {
                li += 1;
            }
            Ordering::Equal => {
                overlap.push(l[li]);
                li += 1;
                ri += 1;
            }
            Ordering::Greater => {
                ri += 1;
            }
        }
    }
    overlap
}

fn priority(elem: char) -> usize {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    alphabet.chars().position(|x| x == elem).unwrap() + 1
}

pub fn main(path: &str) -> (usize, usize) {
    let data = aoc::load_data(path);
    let sacks: Vec<&str> = data.split('\n').collect();

    let sames: Vec<Vec<char>> = sacks
        .iter()
        .filter(|x| !x.is_empty())
        .map(|sack| {
            let compartments = split_compartments(sack);
            let left = compartments[0];
            let right = compartments[1];
            same_types(left, right)
        })
        .collect();

    let distinct_priority: Vec<usize> = sames
        .iter()
        .map(|x| {
            let unique: HashSet<&char> = HashSet::from_iter(x);
            unique.iter().map(|c| priority(**c)).sum::<usize>()
        })
        .collect();

    let part1 = distinct_priority.iter().sum::<usize>();

    let shared_elem: Vec<char> = sacks
        .iter()
        .filter(|x| !x.is_empty())
        .chunks(3)
        .into_iter()
        .map(|x| {
            let elems: Vec<&str> = x.copied().collect();
            let a: HashSet<char> = HashSet::from_iter(elems[0].chars());
            let b: HashSet<char> = HashSet::from_iter(elems[1].chars());
            let c: HashSet<char> = HashSet::from_iter(elems[2].chars());
            let sets = [&b, &c];

            let shared: Vec<&char> = a
                .iter()
                .filter(|x| sets.iter().all(|y| y.contains(x)))
                .collect();
            *shared[0]
        })
        .collect();

    let part2 = shared_elem.iter().map(|c| priority(*c)).sum::<usize>();
    (part1, part2)
}
