#[cfg(test)]
mod day1_tests {
    use super::*;

    static TEST_INPUT: &str = "\
1721
979
366
299
675
1456";

    static TEST_PARSED: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn convert_lines_to_number_array() {
        let input = String::from(TEST_INPUT);

        assert_eq!(Vec::from(TEST_PARSED), lines_to_number(input));
    }

    #[test]
    fn find_the_two_numbers_that_sum_to_2020() {
        assert_eq!(
            vec![1721, 299],
            get_pair_sum_to_2020(Vec::from(TEST_PARSED)).unwrap()
        );
    }

    #[test]
    fn day_1a_basic() {
        let input = String::from(TEST_INPUT);
        assert_eq!(514579, day_1a(input).unwrap());
    }

    #[test]
    fn find_the_three_numbers_that_sum_to_2020() {
        assert_eq!(
            vec![979, 366, 675],
            get_triple_sum_to_2020(Vec::from(TEST_PARSED)).unwrap()
        );
    }

    #[test]
    fn day_1b_basic() {
        let input = String::from(TEST_INPUT);
        assert_eq!(241861950, day_1b(input).unwrap());
    }
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Result};

fn lines_to_number(input: String) -> Vec<i32> {
    return input
        .split('\n')
        .filter(|&s| s != "")
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();
}

fn get_pair_sum_to_2020(inputs: Vec<i32>) -> Result<Vec<i32>> {
    get_pair_sum_to(inputs, 2020)
}

fn get_pair_sum_to(inputs: Vec<i32>, total: i32) -> Result<Vec<i32>> {
    let mut seen_inputs = HashSet::<i32>::new();

    for input in inputs {
        let remain = total - input;

        if seen_inputs.contains(&remain) {
            return Ok(vec![remain, input]);
        }

        seen_inputs.insert(input);
    }

    Err(Error::new(ErrorKind::Other, "No match found"))
}

fn get_triple_sum_to_2020(inputs: Vec<i32>) -> Result<Vec<i32>> {
    for (i, input) in inputs.iter().enumerate() {
        let remain = 2020 - input;

        match get_pair_sum_to(inputs[i..].to_vec(), remain) {
            Ok(pair) => {
                let mut out = pair.clone();
                out.insert(0, *input);
                return Ok(out);
            }
            Err(_) => {}
        }
    }

    Err(Error::new(ErrorKind::Other, "No match found"))
}

fn day_1a(input: String) -> Result<i32> {
    let inputs = lines_to_number(input);
    let sum_to_2020 = get_pair_sum_to_2020(inputs)?;

    Ok(sum_to_2020[0] * sum_to_2020[1])
}

fn day_1b(input: String) -> Result<i32> {
    let inputs = lines_to_number(input);
    let sum_to_2020 = get_triple_sum_to_2020(inputs)?;

    Ok(sum_to_2020[0] * sum_to_2020[1] * sum_to_2020[2])
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day_1a(input.clone())?;
    println!("Day 1A - {}", result);

    let result = day_1b(input)?;
    println!("Day 1B - {}", result);

    Ok(())
}
