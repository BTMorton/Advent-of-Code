use std::cmp;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[cfg(test)]
mod day5_tests {
    use super::*;

    static TEST_INPUT: &str = "
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
    static TEST2_INPUT: &str = "
FFFFFFFLLL
FFFFFFFLLR
FFFFFFFLRL
FFFFFFFLRR
FFFFFFFRLR
FFFFFFFRRL
FFFFFFFRRR
FFFFFFBLLL";

    #[test]
    fn should_parse_a_row_number() {
        assert_eq!(70, parse_row_number("BFFFBBF"));
        assert_eq!(14, parse_row_number("FFFBBBF"));
        assert_eq!(102, parse_row_number("BBFFBBF"));
    }

    #[test]
    fn should_parse_a_seat_number() {
        assert_eq!(7, parse_seat_number("RRR"));
        assert_eq!(5, parse_seat_number("RLR"));
        assert_eq!(4, parse_seat_number("RLL"));
        assert_eq!(0, parse_seat_number("LLL"));
    }

    #[test]
    fn should_parse_a_ticket_to_a_seat_id() {
        assert_eq!(567, parse_ticket("BFFFBBFRRR"));
        assert_eq!(119, parse_ticket("FFFBBBFRRR"));
        assert_eq!(820, parse_ticket("BBFFBBFRLL"));
    }

    #[test]
    fn day5a_tests() {
        assert_eq!(820, day5a(TEST_INPUT));
    }

    #[test]
    fn day5b_tests() {
        assert_eq!(4, day5b(TEST2_INPUT));
    }
}

fn parse_row_number(input: &str) -> usize {
    input
        .chars()
        .map(|c| match c {
            'B' => 1,
            _ => 0,
        })
        .fold(0, |acc, b| acc * 2 + b)
}

fn parse_seat_number(input: &str) -> usize {
    input
        .chars()
        .map(|c| match c {
            'R' => 1,
            _ => 0,
        })
        .fold(0, |acc, b| acc * 2 + b)
}

fn parse_ticket(input: &str) -> usize {
    let (row, seat) = input.split_at(7);

    (parse_row_number(row) * 8) + parse_seat_number(seat)
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day5a(input: &str) -> usize {
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_ticket(line))
        .fold(0, |max, n| cmp::max(max, n))
}

fn day5b(input: &str) -> usize {
    let mut seat_ids: Vec<usize> = input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_ticket(line))
        .collect();
    seat_ids.sort();

    let first_id = seat_ids[0];
    let mut min_index = 0;
    let mut max_index = seat_ids.len();
    let mut next_index = (max_index / 2) + first_id;

    while min_index != max_index {
        let current = seat_ids[next_index];

        if seat_ids[next_index - 1] != current - 1 {
            return current - 1;
        } else if current == first_id + next_index {
            //  Go Up
            min_index = next_index;
        } else {
            //  Go Down
            max_index = next_index;
        }

        next_index = (max_index - min_index) / 2 + min_index;
    }

    0
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day5a(input.as_str());
    println!("Day 5A - {}", result);

    let result = day5b(input.as_str());
    println!("Day 5B - {}", result);

    Ok(())
}
