use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[derive(Debug, PartialEq)]
enum CommandType {
    NoOp,
    Accumulate,
    Jump,
}

#[derive(Debug, PartialEq)]
enum ExecutionResult {
    Loop(i32),
    Term(i32),
}

struct Command {
    command: CommandType,
    input: i32,
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    static TEST_INPUT: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn should_parse_a_command_string() {
        let command = parse_command("nop +0").unwrap();
        assert_eq!(CommandType::NoOp, command.command);
        assert_eq!(0, command.input);

        let command = parse_command("acc +3").unwrap();
        assert_eq!(CommandType::Accumulate, command.command);
        assert_eq!(3, command.input);

        let command = parse_command("jmp -17").unwrap();
        assert_eq!(CommandType::Jump, command.command);
        assert_eq!(-17, command.input);
    }

    #[test]
    fn should_parse_a_command_list() {
        let command_list = parse_command_list(TEST_INPUT);
        assert_eq!(9, command_list.len());
        assert_eq!(CommandType::NoOp, command_list[0].command);
        assert_eq!(CommandType::Accumulate, command_list[1].command);
        assert_eq!(1, command_list[1].input);
        assert_eq!(CommandType::Jump, command_list[2].command);
        assert_eq!(4, command_list[2].input);
    }

    #[test]
    fn should_calculate_the_next_command_index() {
        assert_eq!(
            3,
            get_next_index(
                2,
                &Command {
                    command: CommandType::Accumulate,
                    input: 0
                }
            )
        );
        assert_eq!(
            3,
            get_next_index(
                2,
                &Command {
                    command: CommandType::NoOp,
                    input: 0
                }
            )
        );
        assert_eq!(
            7,
            get_next_index(
                2,
                &Command {
                    command: CommandType::Jump,
                    input: 5
                }
            )
        );
        assert_eq!(
            1,
            get_next_index(
                2,
                &Command {
                    command: CommandType::Jump,
                    input: -1
                }
            )
        );
    }

    #[test]
    fn should_detect_loops() {
        assert_eq!(
            ExecutionResult::Loop(0),
            run_command_list(
                &vec!(Command {
                    command: CommandType::Jump,
                    input: 0,
                }),
                0,
                0,
                &mut HashSet::<usize>::new()
            )
        );
        assert_eq!(
            ExecutionResult::Term(0),
            run_command_list(
                &vec!(Command {
                    command: CommandType::NoOp,
                    input: 0,
                }),
                0,
                0,
                &mut HashSet::<usize>::new()
            )
        );
        assert_eq!(
            ExecutionResult::Term(0),
            run_command_list(
                &vec!(Command {
                    command: CommandType::Accumulate,
                    input: 0,
                }),
                0,
                0,
                &mut HashSet::<usize>::new()
            )
        );
    }

    #[test]
    fn day8a_test() {
        assert_eq!(5, day8a(TEST_INPUT));
    }

    #[test]
    fn day8b_test() {
        assert_eq!(8, day8b(TEST_INPUT));
    }
}

fn get_next_index(current_index: usize, command: &Command) -> usize {
    match command.command {
        CommandType::Jump => (current_index as i32 + command.input) as usize,
        _ => current_index + 1,
    }
}

fn parse_command(command: &str) -> Option<Command> {
    let mut split = command.split(" ");

    let commandtype = match split.next() {
        Some("acc") => Some(CommandType::Accumulate),
        Some("jmp") => Some(CommandType::Jump),
        Some("nop") => Some(CommandType::NoOp),
        Some(_) => None,
        None => None,
    };

    match split.next()?.parse::<i32>() {
        Err(_) => None,
        Ok(n) => Some(Command {
            command: commandtype?,
            input: n,
        }),
    }
}

fn parse_command_list(input: &str) -> Vec<Command> {
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_command(line))
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

fn run_command_list(
    command_list: &Vec<Command>,
    start_accumulator: i32,
    start_index: usize,
    seen_indexes: &mut HashSet<usize>,
) -> ExecutionResult {
    let mut accumulator = start_accumulator;
    let mut current_index = start_index;

    while current_index < command_list.len() {
        if seen_indexes.contains(&current_index) {
            return ExecutionResult::Loop(accumulator);
        }

        let current_command = match command_list.get(current_index) {
            None => break,
            Some(c) => c,
        };

        accumulator = process_command(accumulator, current_command);
        seen_indexes.insert(current_index);
        current_index = get_next_index(current_index, current_command);
    }

    if current_index == command_list.len() {
        ExecutionResult::Term(accumulator)
    } else {
        ExecutionResult::Loop(accumulator)
    }
}

fn process_command(accumulator: i32, command: &Command) -> i32 {
    match command.command {
        CommandType::Accumulate => accumulator + command.input,
        _ => accumulator,
    }
}

fn day8a(input: &str) -> i32 {
    let command_list = parse_command_list(input);
    let exec_result = run_command_list(&command_list, 0, 0, &mut HashSet::<usize>::new());

    match exec_result {
        ExecutionResult::Loop(n) => n,
        ExecutionResult::Term(_) => -1,
    }
}

fn day8b(input: &str) -> i32 {
    let command_list = parse_command_list(input);
    let mut accumulator = 0_i32;
    let mut current_index = 0;
    let mut seen_indexes = HashSet::<usize>::new();

    while current_index < command_list.len() {
        println!("{}", current_index);
        if seen_indexes.contains(&current_index) {
            return -2;
        }

        let current_command = match command_list.get(current_index) {
            None => return -1,
            Some(command) => command,
        };

        if current_command.command == CommandType::Accumulate {
            accumulator = process_command(accumulator, current_command);
            seen_indexes.insert(current_index);
            current_index = get_next_index(current_index, current_command);
            continue;
        }

        seen_indexes.insert(current_index);
        let test_next_index = get_next_index(
            current_index,
            &Command {
                command: match &current_command.command {
                    CommandType::Jump => CommandType::NoOp,
                    CommandType::NoOp => CommandType::Jump,
                    CommandType::Accumulate => CommandType::Accumulate,
                },
                input: current_command.input,
            },
        );

        if test_next_index == command_list.len() {
            return accumulator;
        }

        match run_command_list(
            &command_list,
            accumulator,
            test_next_index,
            &mut seen_indexes.iter().cloned().collect(),
        ) {
            ExecutionResult::Term(n) => return n,
            ExecutionResult::Loop(_) => {
                current_index = get_next_index(current_index, current_command);
            }
        }
    }

    accumulator
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day8a(input.as_str());
    println!("Day 8A - {}", result);

    let result = day8b(input.as_str());
    println!("Day 8B - {}", result);

    Ok(())
}
