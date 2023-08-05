use std::env;
use std::fs;

mod aoc2022;

pub fn load_data(path: &str) -> String {
    fs::read_to_string(path).expect("cannot read path")
}

pub fn default_path(year: u32, day: u8) -> String {
    format!("data/{}/{}/input.txt", year, day)
}
pub fn sample_path(year: u32, day: u8) -> String {
    format!("data/{}/{}/sample.txt", year, day)
}

fn cast_str<T, U>((val0, val1): (T, U)) -> (String, String)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    (val0.to_string(), val1.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = &args[1].to_owned()[..];
    let day = &args[2].to_owned()[..];
    let default = crate::default_path(year.parse().unwrap(), day.parse().unwrap());
    let data_path = if args.len() > 3 { &args[3] } else { &default };

    let (part1, part2) = match (year, day) {
        ("2022", "1") => cast_str(aoc2022::day1::main(data_path)),
        ("2022", "2") => cast_str(aoc2022::day2::main(data_path)),
        ("2022", "3") => cast_str(aoc2022::day3::main(data_path)),
        ("2022", "4") => cast_str(aoc2022::day4::main(data_path)),
        ("2022", "5") => cast_str(aoc2022::day5::main(data_path)),
        ("2022", "6") => cast_str(aoc2022::day6::main(data_path)),
        _ => ("".to_string(), "".to_string()),
    };
    if !part1.is_empty() || !part2.is_empty() {
        println!("{}", part1);
        println!("{}", part2);
    }
}
