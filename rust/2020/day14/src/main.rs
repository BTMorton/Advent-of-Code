use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[derive(Debug, PartialEq)]
enum Command {
    SetMask(String),
    SetMemory(usize, usize),
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    static TEST_INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static TEST2_INPUT: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn should_parse_the_input() {
        let commands = parse_input(TEST_INPUT);
        assert_eq!(
            Command::SetMask(String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
            commands[0]
        );
        assert_eq!(Command::SetMemory(8, 11), commands[1]);
        assert_eq!(Command::SetMemory(7, 101), commands[2]);
        assert_eq!(Command::SetMemory(8, 0), commands[3]);
    }

    #[test]
    fn should_parse_a_memory_command() {
        assert_eq!(Some(Command::SetMemory(8, 11)), parse_memset("mem[8] = 11"));
        assert_eq!(
            Some(Command::SetMemory(7, 101)),
            parse_memset("mem[7] = 101")
        );
        assert_eq!(Some(Command::SetMemory(8, 0)), parse_memset("mem[8] = 0"));
    }

    #[test]
    fn should_parse_a_bitmask_command() {
        assert_eq!(
            Some(Command::SetMask(String::from(
                "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            ))),
            parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        );
    }

    #[test]
    fn should_convert_a_mask_to_and_or_numbers() {
        let (and_mask, or_mask) =
            convert_mask(&String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(64, or_mask);
        assert_eq!(68719476733, and_mask);
    }

    #[test]
    fn should_calculate_memory_locations_based_upon_mask() {
        let expected = vec![26, 27, 58, 59];
        assert_eq!(
            expected,
            calculate_memory_locations(42, &String::from("000000000000000000000000000000X1001X"))
        );

        let expected = vec![16, 17, 18, 19, 24, 25, 26, 27];
        assert_eq!(
            expected,
            calculate_memory_locations(26, &String::from("00000000000000000000000000000000X0XX"))
        );
    }

    #[test]
    fn day14a_test() {
        let commands = parse_input(TEST_INPUT);
        assert_eq!(165, day14a(&commands));
    }

    #[test]
    fn day14b_test() {
        let commands = parse_input(TEST2_INPUT);
        assert_eq!(208, day14b(&commands));
    }
}

fn calculate_memory_locations(location: usize, mask: &String) -> Vec<usize> {
    let bits: Vec<usize> = format!("{:b}", location)
        .chars()
        .rev()
        .map(|c| match c {
            '1' => 1,
            _ => 0,
        })
        .collect();

    mask.chars()
        .enumerate()
        .map(|(i, c)| (mask.len() - i - 1, c))
        .map(|(i, c)| match c {
            'X' => vec![0, 1],
            '1' => vec![1],
            _ if i >= bits.len() => vec![0],
            _ => vec![bits[i]],
        })
        .fold(vec![0], |acc, arr| {
            acc.iter()
                .flat_map(|n| arr.iter().map(move |&b| n * 2 + b))
                .collect()
        })
}

fn convert_mask(mask: &String) -> (usize, usize) {
    mask.chars()
        .map(|c| match c {
            '1' => (1, 1),
            '0' => (0, 0),
            _ => (1, 0),
        })
        .fold((0, 0), |(and_acc, or_acc), (and, or)| {
            (and_acc * 2 + and, or_acc * 2 + or)
        })
}

fn parse_mask(input: &str) -> Option<Command> {
    let bitmask = input.split("=").nth(1)?;
    Some(Command::SetMask(bitmask.trim().to_string()))
}

fn parse_memset(input: &str) -> Option<Command> {
    let mut input_parts = input.split("=");
    let memory_location = input_parts
        .next()?
        .trim()
        .trim_start_matches("mem[")
        .trim_end_matches("]")
        .parse::<usize>()
        .unwrap();
    let new_value = input_parts.next()?.trim().parse::<usize>().unwrap();
    Some(Command::SetMemory(memory_location, new_value))
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|line| match line {
            l if l.starts_with("mask") => parse_mask(l),
            l if l.starts_with("mem") => parse_memset(l),
            _ => None,
        })
        .filter_map(|x| x)
        .collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day14a(program: &Vec<Command>) -> usize {
    let mut and_mask = 68719476735;
    let mut or_mask = 0;
    let mut memory = HashMap::<usize, usize>::new();

    for command in program {
        match command {
            Command::SetMask(mask) => {
                let (new_and_mask, new_or_mask) = convert_mask(mask);
                and_mask = new_and_mask;
                or_mask = new_or_mask;
            }
            Command::SetMemory(pos, value) => {
                memory.insert(*pos, value & and_mask | or_mask);
            }
        }
    }

    memory.values().filter(|&&x| x > 0).sum()
}

fn day14b(program: &Vec<Command>) -> usize {
    let mut mask: String = String::from("000000000000000000000000000000000000");
    let mut memory = HashMap::<usize, usize>::new();

    for command in program {
        match command {
            Command::SetMask(new_mask) => {
                mask = new_mask.to_string();
            }
            Command::SetMemory(pos, value) => {
                for calc_pos in calculate_memory_locations(*pos, &mask) {
                    memory.insert(calc_pos, *value);
                }
            }
        }
    }

    memory.values().filter(|&&x| x > 0).sum()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let program = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day14a(&program);
    println!(
        "Day 14A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day14b(&program);
    println!(
        "Day 14B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
