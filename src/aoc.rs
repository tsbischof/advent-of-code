use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod aoc2022;
mod aoc2023;

pub fn load_data(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("cannot read path")
}

pub fn default_path(year: u32, day: u8) -> std::path::PathBuf {
    let mut path = PathBuf::new();
    path.push("data");
    path.push(format!("{}", year));
    path.push(format!("{}", day));
    path.push("input.txt");
    path
}

pub fn sample_path(year: u32, day: u8) -> std::path::PathBuf {
    let mut path = PathBuf::new();
    path.push("data");
    path.push(format!("{}", year));
    path.push(format!("{}", day));
    path.push("sample.txt");
    path
}

fn display_parts<T, U>((part1, part2): (T, U))
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    println!("{}", part1);
    println!("{}", part2);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(short, long)]
    year: u32,
    #[arg(short, long)]
    day: u8,
    #[arg(short='i', long="input-file", default_value=None)]
    data_path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();
    let year = args.year;
    let day = args.day;
    let data_path = match args.data_path {
        Some(path) => path,
        None => crate::default_path(year, day),
    };

    match (year, day) {
        (2022, 1) => display_parts(aoc2022::day1::main(&data_path)),
        (2022, 2) => display_parts(aoc2022::day2::main(&data_path)),
        (2022, 3) => display_parts(aoc2022::day3::main(&data_path)),
        (2022, 4) => display_parts(aoc2022::day4::main(&data_path)),
        (2022, 5) => display_parts(aoc2022::day5::main(&data_path)),
        (2022, 6) => display_parts(aoc2022::day6::main(&data_path)),
        //       ("2022", "7") => display_parts(aoc2022::day6::main(data_path)),
        (2023, 1) => display_parts(aoc2023::day1::main(&data_path)),
        (2023, 2) => display_parts(aoc2023::day2::main(&data_path)),
        (2023, 3) => display_parts(aoc2023::day3::main(&data_path)),
        _ => println!("could not find year {} day {}", year, day),
    };
}
