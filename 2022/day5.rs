use crate::aoc;
use regex::Regex;

#[derive(Debug)]
struct Move {
    amount: u8,
    from: usize,
    to: usize,
}

fn parse_row(row: &str) -> Vec<char> {
    let size = (row.len() + 1) / 4;
    let mut v = Vec::new();

    for column in 0..size {
        let c: char = row.as_bytes()[4 * column + 1] as char;
        v.push(c);
    }
    v
}

fn rows_to_columns(rows: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if !rows.iter().all(|v| v.len() == rows[0].len()) {
        panic!("some rows had different lengths, expected all the same");
    }

    let mut v = Vec::new();
    let n_columns = rows[0].len();
    let n_rows = rows.len();

    for column in 0..n_columns {
        v.push(Vec::new());
        for row in 0..n_rows {
            if rows[row][column] != ' ' {
                v[column].push(rows[row][column]);
            }
        }
        v[column].reverse();
    }
    v
}

fn parse_move(_move: &str) -> Move {
    let re = Regex::new(r"move (?<amount>\d+) from (?<from>\d+) to (?<to>\d+)").unwrap();
    let Some(_m) = re.captures(_move) else {
        panic!("failed to parse {}", _move);
    };
    Move {
        amount: _m["amount"].parse().unwrap(),
        from: _m["from"].parse().unwrap(),
        to: _m["to"].parse().unwrap(),
    }
}

fn tops(columns: Vec<Vec<char>>) -> Vec<char> {
    let mut v = Vec::new();
    for c in columns.iter() {
        v.push(c[c.len() - 1]);
    }
    v
}

fn apply_move(columns: &mut Vec<Vec<char>>, _move: &Move) {
    let from = _move.from - 1;
    let to = _move.to - 1;

    if _move.to > columns.len() {
        for _ in 0..(_move.to - columns.len()) {
            columns.push(Vec::new());
        }
    }
    for _ in 0.._move.amount {
        let Some(c) = columns[from].pop() else {
            panic!("found empty column");
        };

        columns[to].push(c);
    }
}

fn apply_move_stack(columns: &mut Vec<Vec<char>>, _move: &Move) {
    let mut stack = Vec::new();
    let from = _move.from - 1;
    let to = _move.to - 1;

    if _move.to > columns.len() {
        for _ in 0..(_move.to - columns.len()) {
            columns.push(Vec::new());
        }
    }

    for _ in 0.._move.amount {
        let Some(c) = columns[from].pop() else {
            panic!("found empty column");
        };
        stack.push(c);
    }
    for _ in 0..stack.len() {
        let Some(c) = stack.pop() else {panic!("empty stack")};
        columns[to].push(c);
    }
}

pub fn main(path: &str) {
    let data = aoc::load_data(path);
    let rows: Vec<Vec<char>> = data
        .split("\n")
        .filter(|x| !x.is_empty() && x.contains('['))
        .map(parse_row)
        .collect();
    let start_columns: Vec<Vec<char>> = rows_to_columns(rows);
    let moves: Vec<Move> = data
        .split("\n")
        .filter(|x| x.contains("move"))
        .map(parse_move)
        .collect();

    let mut columns = start_columns.clone();
    for _move in moves.iter() {
        apply_move(&mut columns, _move);
    }
    let top: String = tops(columns).iter().map(|x| *x).collect();
    println!("{}", top);

    let mut columns = start_columns.clone();
    for _move in moves.iter() {
        apply_move_stack(&mut columns, _move);
    }
    let top: String = tops(columns).iter().map(|x| *x).collect();
    println!("{}", top);
}
