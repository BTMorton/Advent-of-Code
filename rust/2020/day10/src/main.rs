use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[cfg(test)]
mod day10_tests {
    use super::*;

    static TEST_INPUT: &str = "
16
10
15
5
1
11
7
19
6
12
4";

    static TEST2_INPUT: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    static TEST2_PARSED: [u32; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    static TEST_PARSED: [u32; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    #[test]
    fn should_parse_input_to_numbers() {
        assert_eq!(Vec::from(TEST_PARSED), file_to_numbers(TEST_INPUT));
    }

    #[test]
    fn should_calculate_differences_between_voltages() {
        let mut input = Vec::from(TEST_PARSED);
        input.sort();
        let result = calculate_difference_jumps(&input);
        assert_eq!(7, result.0);
        assert_eq!(0, result.1);
        assert_eq!(5, result.2);

        let mut input = Vec::from(TEST2_PARSED);
        input.sort();
        let result = calculate_difference_jumps(&input);
        assert_eq!(22, result.0);
        assert_eq!(0, result.1);
        assert_eq!(10, result.2);
    }

    #[test]
    fn should_calculate_permutations_of_a_subblock() {
        let mut input = Vec::from(TEST_PARSED);
        input.sort();
        // input.insert(0, 0);
        assert_eq!(8, calculate_permutations(&input));

        let mut input = Vec::from(TEST2_PARSED);
        input.sort();
        // input.insert(0, 0);
        assert_eq!(19208, calculate_permutations(&input));
    }

    #[test]
    fn day10a_test() {
        assert_eq!(35, day10a(TEST_INPUT));
        assert_eq!(220, day10a(TEST2_INPUT));
    }

    #[test]
    fn day10b_test() {
        assert_eq!(8, day10b(TEST_INPUT));
        assert_eq!(19208, day10b(TEST2_INPUT));
    }
}

fn calculate_permutations(block: &Vec<u32>) -> u64 {
    let mut seen_map = HashMap::<u32, u64>::new();

    for i in (0..block.len()).rev() {
        let current = block[i];
        let mut current_permutations = 0;

        for j in block[i + 1..min(i + 4, block.len())]
            .iter()
            .filter(|&&x| x <= current + 3)
        {
            current_permutations += seen_map.get(j).unwrap_or(&0);
        }

        current_permutations = max(current_permutations, 1);
        seen_map.insert(current, current_permutations);
    }

    (0..min(3, block.len()))
        .map(|i| block[i])
        .filter(|&n| n <= 3)
        .map(|n| seen_map.get(&n).unwrap_or(&0))
        .sum()
}

fn calculate_difference_jumps(sorted_input: &Vec<u32>) -> (usize, usize, usize) {
    let mut last = 0;
    let mut result = (0, 0, 1);

    for &num in sorted_input {
        match num - last {
            1 => result.0 += 1,
            2 => result.1 += 1,
            3 => result.2 += 1,
            _ => {}
        }

        last = num;
    }

    result
}

fn file_to_numbers(input: &str) -> Vec<u32> {
    return input
        .split('\n')
        .filter(|&s| s != "")
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day10a(input: &str) -> usize {
    let mut numbers = file_to_numbers(input);
    numbers.sort();
    let result = calculate_difference_jumps(&numbers);
    result.0 * result.2
}

fn day10b(input: &str) -> u64 {
    let mut numbers = file_to_numbers(input);
    numbers.sort();
    calculate_permutations(&numbers)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day10a(input.as_str());
    println!("Day 10A - {}", result);

    let result = day10b(input.as_str());
    println!("Day 10B - {}", result);

    Ok(())
}
