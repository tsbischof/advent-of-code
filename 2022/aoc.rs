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

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].to_owned()[..];
    let default_path = format!("day{}/input.txt", day);
    let data_path = if args.len() > 2 {
        &args[2]
    } else {
        &default_path
    };

    match day {
        "1" => day1::main(data_path),
        "2" => day2::main(data_path),
        "3" => day3::main(data_path),
        "4" => day4::main(data_path),
        "5" => day5::main(data_path),
        _ => println!("could not find day {}", day),
    }
}
