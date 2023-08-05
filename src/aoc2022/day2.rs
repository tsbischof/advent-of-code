#[cfg(test)]
mod tests {
    use crate::aoc2022::day2;

    #[test]
    fn test_main() {
        let (part1, part2) = day2::main(&crate::default_path(2022, 2));
        assert_eq!(part1, 11841);
        assert_eq!(part2, 13022);
    }
}

fn round_score(them: &str, us: &str) -> i32 {
    let pick_score = match us {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
    let outcome_score = match them {
        "A" => match us {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => -1000,
        },
        "B" => match us {
            "Y" => 3,
            "Z" => 6,
            "X" => 0,
            _ => -1000,
        },
        "C" => match us {
            "Z" => 3,
            "X" => 6,
            "Y" => 0,
            _ => -1000,
        },
        _ => -1000,
    };
    pick_score + outcome_score
}

fn second_strat<'a>(them: &'a str, us: &str) -> &'a str {
    match them {
        "A" => match us {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => "",
        },
        "B" => match us {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => "",
        },
        "C" => match us {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => "",
        },
        _ => "",
    }
}

pub fn main(path: &str) -> (i32, i32) {
    let data = crate::load_data(path);
    let rounds: Vec<&str> = data.split('\n').collect();
    let scores: Vec<i32> = rounds
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts = x.split(' ').collect::<Vec<&str>>();
            round_score(parts[0], parts[1])
        })
        .collect();

    let part1 = scores.iter().sum::<i32>();

    let scores: Vec<i32> = rounds
        .iter()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let parts = x.split(' ').collect::<Vec<&str>>();
            let them = parts[0];
            let us = second_strat(them, parts[1]);
            round_score(them, us)
        })
        .collect();
    let part2 = scores.iter().sum::<i32>();
    (part1, part2)
}
