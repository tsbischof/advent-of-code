use crate::aoc;
pub fn main(data_path: &str) {
    let data = aoc::load_data(data_path);
    let elves = data.split("\n\n");
    let mut totals : Vec<u64> = elves.map(
        |elf| elf
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .sum::<u64>()
    ).collect();
    totals.sort();
    totals.reverse();

    println!("max: {}", totals.first().expect("empty"));
    println!("top3: {}", totals[0..=2].iter().sum::<u64>())
}

