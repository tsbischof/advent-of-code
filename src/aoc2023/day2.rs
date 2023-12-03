use regex::Regex;
use std::cmp;
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2023::day2;

    #[test]
    fn test_main() {
        let (part1, part2) = day2::main(&crate::default_path(2023, 2));
        assert_eq!(part1, 2149);
        assert_eq!(part2, 71274);
    }
}

#[derive(Debug)]
struct Move {
    red: u8,
    green: u8,
    blue: u8,
}

impl Move {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn max(self, other: &Move) -> Move {
        Move {
            red: cmp::max(self.red, other.red),
            green: cmp::max(self.green, other.green),
            blue: cmp::max(self.blue, other.blue),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u8,
    moves: Vec<Move>,
}

impl Game {
    pub fn new(id: u8, moves: Vec<Move>) -> Self {
        Self { id, moves }
    }
}

fn move_parser(text: &str) -> Move {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let re = Regex::new(r"\s*(\d+)\s*(\S+)\s*").unwrap();
    for elem in text.split(',') {
        let (_, [number, color]) = re.captures(elem).unwrap().extract();

        let n = number.parse::<u8>().ok().unwrap();
        match color {
            "blue" => blue = n,
            "green" => green = n,
            "red" => red = n,
            &_ => todo!(),
        }
    }
    Move::new(red, green, blue)
}

fn game_parser(text: &str) -> Option<Game> {
    let (_, [id_text, moves_text]) = Regex::new(r"Game (\d+):(.+)")
        .unwrap()
        .captures(text)
        .unwrap()
        .extract();
    let id = id_text.parse::<u8>();
    let moves: Vec<Move> = moves_text.split(';').map(move_parser).collect();
    match (id, moves) {
        (Ok(id), moves) => Some(Game::new(id, moves)),
        _ => None,
    }
}

fn game_is_possible(game: &Game, max_blue: u8, max_green: u8, max_red: u8) -> bool {
    game.moves
        .iter()
        .all(|m| m.red <= max_red && m.green <= max_green && m.blue <= max_blue)
}

fn max_per_color(game: Game) -> Move {
    game.moves
        .iter()
        .fold(Move::new(0, 0, 0), |mv, m| mv.max(m))
}

fn move_power(mv: &Move) -> u64 {
    (mv.red as u64) * (mv.green as u64) * (mv.blue as u64)
}

pub fn main(data_path: &PathBuf) -> (u64, u64) {
    let data = crate::load_data(data_path);
    let games = data.split('\n').filter(|x| !x.is_empty()).map(game_parser);

    let possible_games_id_sum: u64 = games
        .clone()
        .flatten()
        .filter(|g| game_is_possible(g, 14, 13, 12))
        .map(|g| g.id as u64)
        .sum();

    let min_dice_per_game: Vec<Move> = games.clone().flatten().map(max_per_color).collect();

    let part1 = possible_games_id_sum;
    let part2 = min_dice_per_game.iter().map(move_power).sum();

    (part1, part2)
}
