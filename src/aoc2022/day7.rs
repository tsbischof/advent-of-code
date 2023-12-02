use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use crate::aoc2022::day7;

    #[test]
    #[ignore]
    fn test_main() {
        let (part1, part2) = day7::main(&crate::default_path(2022, 7));
        assert_eq!(part1, 1655);
        assert_eq!(part2, 2665);
    }

    #[test]
    fn test_main_sample() {
        /*let data = crate::load_data(&crate::sample_path(2022, 7));
        let sys = day7::load_system(&data);
        assert_eq!(sys.cd("e").du(), 584);
        assert_eq!(sys.cd("a").du(), 94853);
        assert_eq!(sys.cd("d").du(), 24933642);
        assert_eq!(sys.cd("/").du(), 48381165); */
    }
}

#[derive(Debug)]
struct Ls;

fn parse_ls(cmd: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(cmd)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(cmd: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(cmd)
}

#[derive(Debug)]
enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_path(cmd: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(cmd)
}

fn parse_command(cmd: &str) -> IResult<&str, Command> {
    let (cmd, _) = tag("$ ")(cmd)?;
    alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(cmd)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(entry: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(entry)
}

pub fn main(data_path: &PathBuf) -> (usize, usize) {
    let data = crate::load_data(data_path);

    let part1 = 0;
    let part2 = 0;

    (part1, part2)
}
