use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[cfg(test)]
mod day9_tests {
    use super::*;

    static TEST_INPUT: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    static TEST_PARSED: [u32; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn should_parse_input_to_numbers() {
        assert_eq!(Vec::from(TEST_PARSED), file_to_numbers(TEST_INPUT));
    }

    #[test]
    fn should_take_the_first_n_numbers_as_preamble() {
        let input = Vec::from(TEST_PARSED);
        let result = calculate_preamble(&input, 5);
        assert_eq!(5, result.len());
        assert!(result.contains_key(&input[0]));
        assert_eq!(1, *result.get(&input[0]).unwrap());
        assert!(result.contains_key(&input[1]));
        assert_eq!(1, *result.get(&input[1]).unwrap());
        assert!(result.contains_key(&input[2]));
        assert_eq!(1, *result.get(&input[2]).unwrap());
        assert!(result.contains_key(&input[3]));
        assert_eq!(1, *result.get(&input[3]).unwrap());
        assert!(result.contains_key(&input[4]));
        assert_eq!(1, *result.get(&input[4]).unwrap());
    }

    #[test]
    fn should_increment_the_seen_count_when_the_same_number_appears_multiple_times() {
        let result = calculate_preamble(&vec![1, 1, 1, 1, 1, 1, 1], 5);
        assert_eq!(1, result.len());
        assert!(result.contains_key(&1));
        assert_eq!(5, *result.get(&1).unwrap());
    }

    #[test]
    fn should_replace_a_number_in_the_preamble_map() {
        let mut map: HashMap<u32, u32> = vec![1, 2, 3, 4, 5].iter().map(|&n| (n, 1)).collect();

        replace_preamble(&mut map, 1, 6);

        assert_eq!(5, map.len());
        assert!(!map.contains_key(&1));
        assert!(map.contains_key(&6));
        assert_eq!(1, *map.get(&6).unwrap());
    }

    #[test]
    fn should_decrement_the_seen_count_when_replacing_a_number_that_appears_multiple_times() {
        let mut map: HashMap<u32, u32> = vec![(1, 5)].into_iter().collect();
        replace_preamble(&mut map, 1, 2);
        assert_eq!(2, map.len());
        assert!(map.contains_key(&1));
        assert!(map.contains_key(&2));
        assert_eq!(4, *map.get(&1).unwrap());
        assert_eq!(1, *map.get(&2).unwrap());
    }

    #[test]
    fn should_check_if_a_number_is_created_from_the_preamble() {
        let map: HashMap<u32, u32> = vec![1, 2, 3, 4, 5].iter().map(|&n| (n, 1)).collect();

        assert!(is_created_from_preamble(&map, 3));
        assert!(is_created_from_preamble(&map, 8));
        assert!(!is_created_from_preamble(&map, 10));
        assert!(!is_created_from_preamble(&map, 15));
    }

    #[test]
    fn should_find_number_not_created_from_preamble() {
        let result = find_number_not_created_from_preamble(&Vec::from(TEST_PARSED), 5);
        assert!(result.is_some());
        assert_eq!(127, result.unwrap());
    }

    #[test]
    fn should_find_the_indexes_that_add_to_target() {
        let (start, end) = find_indexes_that_add_to(&Vec::from(TEST_PARSED), 127).unwrap();
        assert_eq!(2, start);
        assert_eq!(5, end);
    }
}

fn find_indexes_that_add_to(input: &Vec<u32>, target: u32) -> Option<(usize, usize)> {
    let input_length = input.len();
    for i in 0..input_length {
        let current_input = input[i];
        if current_input >= target {
            continue;
        }

        let mut remain = target - current_input;
        for j in i + 1..input_length {
            let next_input = input[j];
            if next_input > remain {
                break;
            } else if next_input == remain {
                return Some((i, j));
            }

            remain -= next_input;
        }
    }
    None
}

fn find_number_not_created_from_preamble(input: &Vec<u32>, preamble_length: usize) -> Option<u32> {
    let mut preamble = calculate_preamble(input, preamble_length);

    for n in preamble_length..input.len() {
        let to_check = input[n];
        if !is_created_from_preamble(&preamble, to_check) {
            return Some(to_check);
        }
        replace_preamble(&mut preamble, input[n - preamble_length], to_check);
    }

    None
}

fn is_created_from_preamble(map: &HashMap<u32, u32>, to_test: u32) -> bool {
    map.keys().any(|&n| {
        if n > to_test {
            return false;
        }

        match to_test - n {
            x if x == n => false,
            x => map.contains_key(&x),
        }
    })
}

fn replace_preamble(map: &mut HashMap<u32, u32>, to_remove: u32, to_add: u32) {
    let removed_count = map.entry(to_remove).or_insert(1_u32);
    if *removed_count > 1_u32 {
        *removed_count -= 1_u32;
    } else {
        map.remove(&to_remove);
    }

    let added_count = map.entry(to_add).or_insert(0);
    *added_count += 1;
}

fn calculate_preamble(input: &Vec<u32>, preamble_length: usize) -> HashMap<u32, u32> {
    input[..preamble_length]
        .iter()
        .fold(HashMap::<u32, u32>::new(), |mut map, &n| {
            {
                let count = map.entry(n).or_insert(0);
                *count += 1;
            }
            map
        })
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

fn day9a(input: &str) -> u32 {
    let numbers = file_to_numbers(input);
    find_number_not_created_from_preamble(&numbers, 25).unwrap_or(0)
}

fn day9b(input: &str) -> u32 {
    let numbers = file_to_numbers(input);
    let target = match find_number_not_created_from_preamble(&numbers, 25) {
        Some(t) => t,
        None => return 0,
    };
    let (start, end) = match find_indexes_that_add_to(&numbers, target) {
        Some(result) => result,
        None => return 0,
    };

    numbers[start..=end].iter().min().unwrap_or(&0_u32)
        + numbers[start..=end].iter().max().unwrap_or(&0_u32)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day9a(input.as_str());
    println!("Day 9A - {}", result);

    let result = day9b(input.as_str());
    println!("Day 9B - {}", result);

    Ok(())
}
