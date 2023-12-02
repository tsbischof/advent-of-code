use regex::Regex;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2023::day1;

    #[test]
    fn test_main() {
        let (part1, part2) = day1::main(&crate::default_path(2023, 1));
        assert_eq!(part1, 55172);
        assert_eq!(part2, 54925);
    }
}

fn word_to_digit(word: &str) -> Option<u32> {
    match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "0" => Some(0),
        "1" => Some(1),
        "2" => Some(2),
        "3" => Some(3),
        "4" => Some(4),
        "5" => Some(5),
        "6" => Some(6),
        "7" => Some(7),
        "8" => Some(8),
        "9" => Some(9),
        _ => None,
    }
}

fn first_last_digit(text: &str) -> [&str; 2] {
    let re: Regex = Regex::new(r"\d").unwrap();
    let matches: Vec<&str> = re.find_iter(text).map(|x| x.as_str()).collect();
    [
        matches.first().unwrap_or(&""),
        matches.last().unwrap_or(&""),
    ]
}

fn reverse(text: &str) -> String {
    let mut r = String::new();
    for c in text.chars() {
        r.insert(0, c);
    }
    r
}

fn first_last_digit_word(text: &str) -> [String; 2] {
    let forward = match Regex::new(r"(\d|zero|one|two|three|four|five|six|seven|eight|nine)")
        .unwrap()
        .find(text)
    {
        Some(val) => val.as_str(),
        None => "",
    };
    let text_rev = reverse(text);
    let backward = match Regex::new(r"(\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)")
        .unwrap()
        .find(text_rev.as_str())
    {
        Some(val) => reverse(val.as_str()),
        None => String::from(""),
    };
    [String::from(forward), backward]
}

pub fn main(data_path: &PathBuf) -> (u64, u64) {
    let data = crate::load_data(data_path);
    let readings = data.split('\n');
    let digits: Vec<[&str; 2]> = readings
        .clone()
        .map(first_last_digit)
        .filter(|m| !m[0].is_empty())
        .collect();
    let digit_values: Vec<u32> = digits
        .iter()
        .map(|s| s[0].parse::<u32>().unwrap() * 10 + s[1].parse::<u32>().unwrap())
        .collect();

    let words: Vec<[String; 2]> = readings
        .clone()
        .map(first_last_digit_word)
        .filter(|m| m[0].is_empty())
        .collect();
    let word_values: Vec<u32> = words
        .iter()
        .map(|s| word_to_digit(s[0].as_str()).unwrap() * 10 + word_to_digit(s[1].as_str()).unwrap())
        .collect();

    let part1 = digit_values.iter().sum::<u32>() as u64;
    let part2 = word_values.iter().sum::<u32>() as u64;

    (part1, part2)
}
