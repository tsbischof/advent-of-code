use std::env;
use std::iter::FromIterator;
use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn split_compartments(sack: &str) -> Vec<&str> {
    let mid = sack.len() / 2;
    let mut c = Vec::new();
    c.push(&sack[..mid]);
    c.push(&sack[mid..]);
    c
}

fn same_types(left: &str, right: &str) -> Vec<char> {
    let mut l : Vec<char> = left.chars().collect();
    let mut r : Vec<char> = right.chars().collect();

    l.sort();
    r.sort();
    let mut li = 0;
    let mut ri = 0;

    let mut overlap = Vec::new();

    while li < l.len() && ri < r.len() {
        if l[li] == r[ri] {
            overlap.push(l[li]);
            li += 1;
            ri += 1;
        } else if l[li] > r[ri] {
            ri += 1;
        } else {
            li += 1;
        }
    }
    overlap
}

fn priority(elem: char) -> usize {
    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();
    alphabet.chars().position(|x| x == elem).unwrap() + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data_path = if args.len() == 1 { "input.txt" } else { &args[1] };

    let data = fs::read_to_string(data_path).expect("could not read input.xt");
    let sacks : Vec<&str> = data.split("\n").collect();

    let sames : Vec<Vec<char>> = sacks
        .iter()
        .filter(|x| !x.is_empty())
        .map(|sack| {
            let compartments = split_compartments(sack);
            let left = compartments[0];
            let right = compartments[1];
            let same = same_types(left, right);
            same
        })
        .collect();

    let distinct_priority : Vec<usize> = sames
        .iter()
        .map(|x| {
            let unique : HashSet<&char> = HashSet::from_iter(x);
            unique.iter().map(|c| priority(**c)).sum::<usize>()
        })
        .collect();

    println!("{}", distinct_priority.iter().sum::<usize>());

    let shared_elem : Vec<char> = sacks
        .iter()
        .filter(|x| !x.is_empty())
        .chunks(3)
        .into_iter()
        .map(|x| {
            let elems : Vec<&str> = x.map(|y| *y).collect();
            let a : HashSet<char> = HashSet::from_iter(elems[0].chars());
            let b : HashSet<char> = HashSet::from_iter(elems[1].chars());
            let c : HashSet<char> = HashSet::from_iter(elems[2].chars());
            let sets = [&b,&c];
            
            let shared : Vec<&char> = a.iter().filter(|x| sets.iter().all(|y| y.contains(x))).collect();
            *shared[0]
        })
        .collect();

    println!("shared: {}", shared_elem.iter().map(|c| priority(*c)).sum::<usize>());
}
