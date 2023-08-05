use iterwindows::IterArrayWindows;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use crate::aoc2022::day6;

    #[test]
    fn test_main() {
        let (part1, part2) = day6::main(&crate::default_path(2022, 6));
        assert_eq!(part1, 1655);
        assert_eq!(part2, 2665);
    }

    #[test]
    fn test_main_samples() {
        assert_eq!(
            day6::start_of_packet_marker::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            5
        );
        assert_eq!(
            day6::start_of_packet_marker::<4>("nppdvjthqldpwncqszvftbrmjlhg"),
            6
        );
        assert_eq!(
            day6::start_of_packet_marker::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
        assert_eq!(
            day6::start_of_packet_marker::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
        assert_eq!(
            day6::start_of_packet_marker::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
        assert_eq!(
            day6::start_of_packet_marker::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            23
        );
        assert_eq!(
            day6::start_of_packet_marker::<14>("nppdvjthqldpwncqszvftbrmjlhg"),
            23
        );
        assert_eq!(
            day6::start_of_packet_marker::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
        assert_eq!(
            day6::start_of_packet_marker::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }
}

fn start_of_packet_marker<const MATCH_LEN: usize>(msg: &str) -> usize {
    let Some(position) = msg.chars().array_windows::<MATCH_LEN>().position(|r| HashSet::from(r).len() == r.len()) else {panic!("could not find streak in {}", msg)};
    position + MATCH_LEN
}

pub fn main(data_path: &str) -> (usize, usize) {
    let data = crate::load_data(data_path);

    let part1 = start_of_packet_marker::<4>(&data);
    let part2 = start_of_packet_marker::<14>(&data);

    (part1, part2)
}
