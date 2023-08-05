use crate::aoc;

#[cfg(test)]
mod tests {
    use crate::day1;

    #[test]
    fn test_main() {
        let (part1, part2) = day1::main("day1/input.txt");
        assert_eq!(part1, 67027);
        assert_eq!(part2, 197291);
    }
}

pub fn main(data_path: &str) -> (u64, u64) {
    let data = aoc::load_data(data_path);
    let elves = data.split("\n\n");
    let mut totals: Vec<u64> = elves
        .map(|elf| {
            elf.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect();
    totals.sort();
    totals.reverse();

    let part1 = totals.first().expect("empty");
    let part2 = totals[0..=2].iter().sum::<u64>();

    (*part1, part2)
}
