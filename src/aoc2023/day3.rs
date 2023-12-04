use array2d::Array2D;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2023::day3;

    #[test]
    fn test_main() {
        let (part1, part2) = day3::main(&crate::default_path(2023, 3));
        assert_eq!(part1, 543867);
        assert_eq!(part2, 79613331);
    }

    #[test]
    fn test_main_sample() {
        let (part1, part2) = day3::main(&crate::sample_path(2023, 3));
        assert_eq!(part1, 4361);
        assert_eq!(part2, 467835);
    }
}

#[derive(Debug)]
struct PartNumber {
    row: usize,
    col: usize,
    value: u64,
    len: usize,
}

impl PartNumber {
    pub fn new(row: usize, col: usize, value: u64, len: usize) -> Self {
        Self {
            row,
            col,
            value,
            len,
        }
    }

    pub fn from_seed(grid: &Array2D<char>, row: usize, col: usize) -> PartNumber {
        let mut start = col;
        let mut stop = col;
        while 0 < start {
            if grid[(row, start - 1)].is_ascii_digit() {
                start -= 1;
            } else {
                break;
            }
        }
        while stop + 1 < grid.num_columns() {
            if grid[(row, stop + 1)].is_ascii_digit() {
                stop += 1;
            } else {
                break;
            }
        }
        PartNumber::new(
            row,
            start,
            word_to_value(grid, row, start, stop),
            stop - start + 1,
        )
    }
}

#[derive(Debug)]
struct Gear {
    row: usize,
    col: usize,
}

impl Gear {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn word_to_value(grid: &Array2D<char>, row: usize, start: usize, stop: usize) -> u64 {
    (start..=stop)
        .map(|p| -> u64 {
            (10_u64).pow((stop - p).try_into().unwrap())
                * (grid[(row, p)].to_digit(10).unwrap() as u64)
        })
        .sum()
}

fn process_word(grid: &Array2D<char>, row: usize, start: usize, stop: usize) -> PartNumber {
    let value = word_to_value(grid, row, start, stop);
    PartNumber::new(row, start, value, stop - start + 1)
}

fn find_numbers(grid: &Array2D<char>) -> Vec<PartNumber> {
    let mut locs = Vec::new();
    let mut start = 0;
    let mut in_number = false;

    for r in 0..grid.num_rows() {
        for c in 0..grid.num_columns() {
            if grid[(r, c)].is_ascii_digit() {
                if !in_number {
                    start = c;
                    in_number = true;
                }
            } else if in_number {
                let stop = c - 1;
                in_number = false;
                locs.push(process_word(grid, r, start, stop));
            }
        }

        if in_number {
            let stop = grid.num_columns() - 1;
            locs.push(process_word(grid, r, start, stop));
            in_number = false;
        }
    }
    locs
}

fn is_part_number(grid: &Array2D<char>, part: &PartNumber) -> bool {
    let low_r = if part.row == 0 { 0 } else { part.row - 1 };
    let high_r = if part.row + 1 < grid.num_rows() {
        part.row + 1
    } else {
        part.row
    };
    for r in low_r..=high_r {
        let low_c = if part.col == 0 { 0 } else { part.col - 1 };
        let high_c = if part.col + part.len < grid.num_columns() {
            part.col + part.len
        } else {
            part.col + part.len - 1
        };
        for c in low_c..=high_c {
            if !grid[(r, c)].is_ascii_digit() && grid[(r, c)] != '.' {
                return true;
            }
        }
    }
    false
}

fn find_gears(grid: &Array2D<char>) -> Vec<Gear> {
    let mut gears = Vec::new();
    for r in 0..grid.num_rows() {
        for c in 0..grid.num_columns() {
            if grid[(r, c)] == '*' {
                gears.push(Gear::new(r, c));
            }
        }
    }
    gears
}

fn find_pairs_near_gears(grid: &Array2D<char>, gears: Vec<Gear>) -> Vec<(PartNumber, PartNumber)> {
    let mut pairs = Vec::new();
    for gear in gears {
        let mut seeds = Vec::new();

        if gear.col > 0 {
            let r = gear.row;
            let c = gear.col - 1;
            if grid[(r, c)].is_ascii_digit() {
                seeds.push((r, c));
            }
        }
        if gear.col + 1 < grid.num_columns() {
            let r = gear.row;
            let c = gear.col + 1;
            if grid[(r, c)].is_ascii_digit() {
                seeds.push((r, c));
            }
        }
        if gear.row > 0 {
            let r = gear.row - 1;
            if grid[(r, gear.col)].is_ascii_digit() {
                seeds.push((r, gear.col));
            } else {
                if gear.col > 0 {
                    let c = gear.col - 1;
                    if grid[(r, c)].is_ascii_digit() {
                        seeds.push((r, c));
                    }
                }
                if gear.col + 1 < grid.num_columns() {
                    let c = gear.col + 1;
                    if grid[(r, c)].is_ascii_digit() {
                        seeds.push((r, c));
                    }
                }
            }
        }
        if gear.row + 1 < grid.num_rows() {
            let r = gear.row + 1;
            if grid[(r, gear.col)].is_ascii_digit() {
                seeds.push((r, gear.col));
            } else {
                if gear.col > 0 {
                    let c = gear.col - 1;
                    if grid[(r, c)].is_ascii_digit() {
                        seeds.push((r, c));
                    }
                }
                if gear.col + 1 < grid.num_columns() {
                    let c = gear.col + 1;
                    if grid[(r, c)].is_ascii_digit() {
                        seeds.push((r, c));
                    }
                }
            }
        }

        if seeds.len() == 2 {
            pairs.push((
                PartNumber::from_seed(grid, seeds[0].0, seeds[0].1),
                PartNumber::from_seed(grid, seeds[1].0, seeds[1].1),
            ));
        }
    }
    pairs
}

pub fn main(data_path: &PathBuf) -> (u64, u64) {
    let data = crate::load_data(data_path);
    let rows: Vec<Vec<char>> = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let grid = match Array2D::from_rows(&rows) {
        Ok(g) => g,
        Err(error) => panic!("Could not convert vector of chars to 2d array: {:?}", error),
    };
    let part_numbers: Vec<u64> = find_numbers(&grid)
        .iter()
        .filter(|p| is_part_number(&grid, p))
        .map(|p| p.value)
        .collect();

    let gears = find_gears(&grid);
    let gear_ratios = find_pairs_near_gears(&grid, gears);

    let part1 = part_numbers.iter().sum();
    let part2 = gear_ratios.iter().map(|(a, b)| a.value * b.value).sum();

    (part1, part2)
}
