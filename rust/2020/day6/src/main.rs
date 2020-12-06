use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

#[cfg(test)]
mod day6_tests {
    use super::*;

    static TEST_INPUT: &str = "
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn should_count_the_number_of_yes_answers_on_one_line() {
        assert_eq!(3, count_yes_answers("abc"));
        assert_eq!(2, count_yes_answers("ab"));
        assert_eq!(1, count_yes_answers("b"));
        assert_eq!(1, count_yes_answers("aaaa"));
        assert_eq!(2, count_yes_answers("aaab"));
    }

    #[test]
    fn should_count_the_number_of_yes_answers_on_multiple_lines() {
        assert_eq!(3, count_yes_answers("ab\nac"));
        assert_eq!(3, count_yes_answers("a\nb\nc"));
        assert_eq!(2, count_yes_answers("aa\nac"));
        assert_eq!(1, count_yes_answers("a\na\na\na"));
    }

    #[test]
    fn should_count_the_number_of_agreed_yes_answers_on_one_lines() {
        assert_eq!(3, count_agreed_yes_answers("abc"));
        assert_eq!(2, count_agreed_yes_answers("ab"));
        assert_eq!(1, count_agreed_yes_answers("b"));
        assert_eq!(1, count_agreed_yes_answers("aaaa"));
        assert_eq!(2, count_agreed_yes_answers("aaab"));
    }

    #[test]
    fn should_count_the_number_of_agreed_yes_answers_on_multiple_lines() {
        assert_eq!(1, count_agreed_yes_answers("ab\nac"));
        assert_eq!(0, count_agreed_yes_answers("a\nb\nc"));
        assert_eq!(2, count_agreed_yes_answers("ac\nac"));
        assert_eq!(1, count_agreed_yes_answers("a\na\na\na"));
    }

    #[test]
    fn day6a_tests() {
        assert_eq!(11, day6a(TEST_INPUT));
    }

    #[test]
    fn day6b_tests() {
        assert_eq!(6, day6b(TEST_INPUT));
    }
}

fn count_yes_answers(input: &str) -> usize {
    let answers: HashSet<char> = input
        .split("\n")
        .flat_map(|line| line.trim().chars())
        .collect();

    answers.len()
}

fn count_agreed_yes_answers(input: &str) -> usize {
    let grouped_answers: Vec<HashSet<char>> = input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| line.trim().chars().collect())
        .collect();

    if grouped_answers.len() == 0 {
        return 0;
    } else if grouped_answers.len() == 1 {
        return grouped_answers[0].len();
    }

    let (first, remain) = grouped_answers.split_at(1);

    first[0]
        .iter()
        .filter(|c| remain.iter().all(|v| v.contains(c)))
        .count()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day6a(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|line| count_yes_answers(line))
        .fold(0, |total, yes_answers| total + yes_answers)
}

fn day6b(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|line| count_agreed_yes_answers(line))
        .fold(0, |total, yes_answers| total + yes_answers)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day6a(input.as_str());
    println!("Day 6A - {}", result);

    let result = day6b(input.as_str());
    println!("Day 6B - {}", result);

    Ok(())
}
