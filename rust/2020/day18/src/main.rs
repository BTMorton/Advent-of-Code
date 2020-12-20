use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[cfg(test)]
mod day18_tests {
    use super::*;

    static TEST_INPUT: &str = "1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn should_tokenise_an_equation() {
        assert_eq!(
            vec!["1", "+", "2", "*", "3", "+", "4", "*", "5", "+", "6"],
            tokenise("1 + 2 * 3 + 4 * 5 + 6".to_string())
        );
        assert_eq!(
            vec!["2", "*", "3", "+", "(", "4", "*", "5", ")"],
            tokenise("2 * 3 + (4 * 5)".to_string())
        );
        assert_eq!(
            vec![
                "5", "*", "9", "*", "(", "7", "*", "3", "*", "3", "+", "9", "*", "3", "+", "(",
                "8", "+", "6", "*", "4", ")", ")"
            ],
            tokenise("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string())
        );
        assert_eq!(
            vec![
                "(", "(", "2", "+", "4", "*", "9", ")", "*", "(", "6", "+", "9", "*", "8", "+",
                "6", ")", "+", "6", ")", "+", "2", "+", "4", "*", "2"
            ],
            tokenise("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string())
        );
    }

    #[test]
    fn should_perform_calculations_on_a_tokenised_string() {
        assert_eq!(
            Some(71),
            calculate(
                &get_part1_precedence(),
                &vec!["1", "+", "2", "*", "3", "+", "4", "*", "5", "+", "6"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(
            Some(26),
            calculate(
                &get_part1_precedence(),
                &vec!["2", "*", "3", "+", "(", "4", "*", "5", ")"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(
            Some(437),
            calculate(
                &get_part1_precedence(),
                &vec!["5", "+", "(", "8", "*", "3", "+", "9", "+", "3", "*", "4", "*", "3", ")"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(
            Some(12240),
            calculate(
                &get_part1_precedence(),
                &vec![
                    "5", "*", "9", "*", "(", "7", "*", "3", "*", "3", "+", "9", "*", "3", "+", "(",
                    "8", "+", "6", "*", "4", ")", ")"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            )
        );
        assert_eq!(
            Some(13632),
            calculate(
                &get_part1_precedence(),
                &vec![
                    "(", "(", "2", "+", "4", "*", "9", ")", "*", "(", "6", "+", "9", "*", "8", "+",
                    "6", ")", "+", "6", ")", "+", "2", "+", "4", "*", "2"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            )
        );
    }

    #[test]
    fn day18a_test() {
        let input = TEST_INPUT
            .lines()
            .map(|line| tokenise(line.to_string()))
            .collect();
        assert_eq!(26386, day18a(&input));
    }

    #[test]
    fn day18b_test() {
        let input = TEST_INPUT
            .lines()
            .map(|line| tokenise(line.to_string()))
            .collect();
        assert_eq!(693942, day18b(&input));
    }
}

fn get_part1_precedence() -> HashMap<String, usize> {
    vec![("+".to_string(), 100), ("*".to_string(), 100)]
        .into_iter()
        .collect()
}

fn get_part2_precedence() -> HashMap<String, usize> {
    vec![("+".to_string(), 200), ("*".to_string(), 100)]
        .into_iter()
        .collect()
}

fn parse_number(number: &String) -> Option<usize> {
    match number.parse::<usize>() {
        Ok(x) => Some(x),
        _ => None,
    }
}

fn apply_terms(
    terms: &HashMap<String, usize>,
    prec_min: usize,
    input: &Vec<String>,
    start_index: usize,
) -> Option<(usize, usize)> {
    let (mut current, mut next_index) = apply_bracket(terms, input, start_index)?;

    while next_index < input.len()
        && terms.contains_key(&input[next_index])
        && terms[&input[next_index]] >= prec_min
    {
        let next_result = apply_terms(terms, terms[&input[next_index]] + 1, input, next_index + 1)?;

        match input[next_index].as_str() {
            "*" => current *= next_result.0,
            "+" => current += next_result.0,
            _ => {}
        };

        next_index = next_result.1;
    }

    return Some((current, next_index));
}

fn apply_bracket(
    terms: &HashMap<String, usize>,
    input: &Vec<String>,
    start_index: usize,
) -> Option<(usize, usize)> {
    if input[start_index] == "(" {
        let (result, next_index) = apply_terms(terms, 0, input, start_index + 1)?;
        assert_eq!(")", input[next_index]);
        Some((result, next_index + 1))
    } else {
        Some((parse_number(&input[start_index])?, start_index + 1))
    }
    // let (mut current, mut i) = parse_next(input, start_index)?;

    // while i < input.len() {
    //     let result = match input[i].as_str() {
    //         "+" => apply_add(current, input, i + 1)?,
    //         "*" => apply_mult(current, input, i + 1)?,
    //         ")" => return Some((current, i + 1)),
    //         _ => return None,
    //     };

    //     current = result.0;
    //     i = result.1;
    // }

    // Some((current, input.len()))
}

fn calculate(terms: &HashMap<String, usize>, input: &Vec<String>) -> Option<usize> {
    match apply_terms(terms, 0, input, 0) {
        Some((result, _)) => Some(result),
        None => None,
    }
}

fn tokenise(line: String) -> Vec<String> {
    line.replace("(", "( ")
        .replace(")", " )")
        .split(" ")
        .filter(|&token| token != "")
        .map(|c| c.to_string())
        .collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day18a(input: &Vec<Vec<String>>) -> usize {
    let precedence = get_part1_precedence();
    input
        .iter()
        .map(|line| calculate(&precedence, line).unwrap())
        .sum()
}

fn day18b(input: &Vec<Vec<String>>) -> usize {
    let precedence = get_part2_precedence();
    input
        .iter()
        .map(|line| calculate(&precedence, line).unwrap())
        .sum()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let lines = input
        .lines()
        .map(|line| tokenise(line.to_string()))
        .collect();

    use std::time::Instant;
    let total = Instant::now();

    let result = day18a(&lines);
    println!(
        "Day 18A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day18b(&lines);
    println!(
        "Day 18B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
