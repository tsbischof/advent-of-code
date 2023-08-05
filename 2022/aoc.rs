use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod aoc {
    use std::fs;
    pub fn load_data(path: &str) -> String {
        fs::read_to_string(path).expect("cannot read path")
    }
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
    let day = &args[1].to_owned()[..];
    let default_path = format!("day{}/input.txt", day);
    let data_path = if args.len() > 2 {
        &args[2]
    } else {
        &default_path
    };

    let (part1, part2) = match day {
        "1" => cast_str(day1::main(data_path)),
        "2" => cast_str(day2::main(data_path)),
        "3" => cast_str(day3::main(data_path)),
        "4" => cast_str(day4::main(data_path)),
        "5" => cast_str(day5::main(data_path)),
        _ => ("".to_string(), "".to_string()),
    };
    if !part1.is_empty() || !part2.is_empty() {
        println!("{}", part1);
        println!("{}", part2);
    }
}
